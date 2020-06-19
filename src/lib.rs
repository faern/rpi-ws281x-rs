use std::convert::TryFrom;
use std::mem;
use std::os::raw::c_int;
use std::ptr;

/// Re-export of the low level bindings to `rpi_ws281x`.
pub use rpi_ws281x_sys as sys;

mod error;
pub use error::{Error, Result};

mod led;
pub use led::Led;

mod strip_type;
pub use strip_type::{InvalidStripTypeError, StripType};

/// `usize` version of `sys::RPI_PWM_CHANNELS`.
pub const NUM_CHANNELS: usize = sys::RPI_PWM_CHANNELS as usize;

#[repr(transparent)]
pub struct ChannelBuilder(sys::ws2811_channel_t);

impl ChannelBuilder {
    /// Creates a new [`ChannelBuilder`] for the given GPIO pin with the given amount of LEDs.
    pub fn new(gpio_pin: u8, led_count: u16) -> Self {
        ChannelBuilder(sys::ws2811_channel_t {
            gpionum: c_int::from(gpio_pin),
            invert: 0,
            count: c_int::from(led_count),
            strip_type: StripType::Gbr.as_raw(),
            leds: ptr::null_mut(),
            brightness: 255,
            wshift: 0,
            rshift: 0,
            gshift: 0,
            bshift: 0,
            gamma: ptr::null_mut(),
        })
    }

    /// Sets the type of LED strip. Defaults to `StripType::Gbr`.
    pub fn strip_type(mut self, strip_type: StripType) -> Self {
        self.0.strip_type = strip_type.as_raw();
        self
    }

    /// Sets if the output IO should be inverted or not. Defaults to `false`.
    pub fn invert(mut self, invert: bool) -> Self {
        self.0.invert = c_int::from(invert);
        self
    }

    /// Sets the brightness of the channel between 0 and 255. Defaults to full brightness, 255.
    pub fn brightness(mut self, brightness: u8) -> Self {
        self.0.brightness = brightness;
        self
    }

    pub fn build(self) -> Channel {
        Channel(self.0)
    }
}

/// A channel represents one GPIO pin sending a signal to one strip of LEDs.
/// There can be up to `NUM_CHANNELS` `Channel`s on one [`Controller`].
///
/// The channel instance is handed over to [`Builder::channels`].
#[repr(transparent)]
pub struct Channel(sys::ws2811_channel_t);

impl Channel {
    /// Creates a new [`ChannelBuilder`] for the given GPIO pin with the given amount of LEDs.
    pub fn builder(gpio_pin: u8, led_count: u16) -> ChannelBuilder {
        ChannelBuilder::new(gpio_pin, led_count)
    }

    /// Returns a disabled LED strip channel.
    ///
    /// # Example
    ///
    /// ```
    /// # use rpi_ws281x::{Channel, Controller};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let channel = Channel::builder(10, 25).build();
    /// let controller = Controller::builder(10)
    ///     .channels([channel, Channel::disabled()])
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn disabled() -> Self {
        Self(sys::ws2811_channel_t {
            gpionum: 0,
            invert: 0,
            count: 0,
            strip_type: 0,
            leds: ptr::null_mut(),
            brightness: 0,
            wshift: 0,
            rshift: 0,
            gshift: 0,
            bshift: 0,
            gamma: ptr::null_mut(),
        })
    }

    /// Creates a `Channel` directly from the underlying C struct. This is highly unsafe and
    /// you are recommended to use the safe builder pattern via [`Channel::builder`] instead.
    ///
    /// # Safety
    ///
    /// `channel` must be correctly set up. See C library for implementation.
    pub unsafe fn from_raw(channel: sys::ws2811_channel_t) -> Self {
        Self(channel)
    }
}

impl From<Channel> for sys::ws2811_channel_t {
    fn from(channel: Channel) -> sys::ws2811_channel_t {
        channel.0
    }
}

/// A builder for [`Controller`] structs. Sets up and initializes the hardware for controlling the
/// LEDs and returns a controller that is then used for actually rendering anything to the LEDs.
#[repr(transparent)]
pub struct ControllerBuilder(sys::ws2811_t);

impl ControllerBuilder {
    /// Creates a new [`Controller`] builder using the given DMA channel.
    ///
    /// Take care to use a free DMA channel. Selecting one that is already in use might interfere
    /// with other hardware and for example corrupt your SD card. This code cannot recommend
    /// a safe default since that depends on the hardware/firmware and OS version.
    pub fn new(dma_channel: u8) -> Self {
        Self(sys::ws2811_t {
            render_wait_time: 0,
            device: ptr::null_mut(),
            rpi_hw: ptr::null(),
            freq: sys::WS2811_TARGET_FREQ,
            dmanum: i32::from(dma_channel),
            channel: [Channel::disabled().0, Channel::disabled().0],
        })
    }

    /// Creates a `ControllerBuilder` directly from the underlying C struct.
    ///
    /// This is highly unsafe and you are recommended to use the safe builder pattern via
    /// [`Controller::builder`] instead.
    ///
    /// # Safety
    ///
    /// `controller` must be correctly set up. See C library for implementation.
    pub unsafe fn from_raw(controller: sys::ws2811_t) -> Self {
        Self(controller)
    }

    /// Sets the frequency in Hz that the controller will output data at.
    pub fn freq(mut self, freq: u32) -> Self {
        self.0.freq = freq;
        self
    }

    /// Sets the channel first on the controller. More convenient to call than
    /// [`ControllerBuilder::channels`] for use cases with only one LED strip.
    pub fn channel(mut self, channel: Channel) -> Self {
        self.0.channel[0] = channel.0;
        self
    }

    /// Sets all channels on the controller.
    pub fn channels(mut self, channels: [Channel; NUM_CHANNELS]) -> Self {
        // This transmute is safe because `Channel` is a newtype with `#[repr(transparent)]`.
        self.0.channel = unsafe { mem::transmute(channels) };
        self
    }

    /// Tries to initialize the hardware to control LEDs in the way the builder is configured.
    /// Returns the [`Controller`] on success.
    pub fn build(mut self) -> Result<Controller> {
        assert_eq!(
            usize::try_from(sys::RPI_PWM_CHANNELS).unwrap(),
            self.0.channel.len()
        );
        match unsafe { sys::ws2811_init(&mut self.0) } {
            sys::ws2811_return_t::WS2811_SUCCESS => Ok(Controller(self.0)),
            error => Err(Error(error)),
        }
    }
}

/// A ws281x LED controller. Instances of this type are created via the [`Builder`].
#[repr(transparent)]
pub struct Controller(sys::ws2811_t);

impl Controller {
    pub fn builder(dma_channel: u8) -> ControllerBuilder {
        ControllerBuilder::new(dma_channel)
    }

    /// Creates a `Controller` directly from the underlying C struct.
    ///
    /// This is highly unsafe and you are recommended to use the safe builder pattern via
    /// [`Controller::builder`] instead.
    ///
    /// # Safety
    ///
    /// `controller` must be correctly set up and [`sys::ws2811_init`] already called on it.
    /// See C library for implementation.
    pub unsafe fn from_raw(controller: sys::ws2811_t) -> Self {
        Self(controller)
    }

    /// Returns a mutable slice where all the LED values can be set directly.
    ///
    /// # Panics
    ///
    /// Panics if `channel_index >= NUM_CHANNELS`.
    pub fn buffer<'a>(&'a mut self, channel_index: usize) -> &mut [Led] {
        // This casting to `*mut Led` is safe because Led is a newtype struct over ws2811_led_t
        // with #[repr(transparent])].
        let leds_ptr: *mut Led = self.0.channel[channel_index].leds as *mut Led;
        let count = usize::try_from(self.0.channel[channel_index].count).unwrap();
        // SAFETY: We trust the C library to have initialized the leds ptr and count correctly.
        unsafe { std::slice::from_raw_parts_mut::<'a, Led>(leds_ptr, count) }
    }

    /// Render what is currently in the buffers to the LEDs.
    ///
    /// See [`render_buffer`] for a way to supply the buffer and render it in one call.
    pub fn render(&mut self) -> Result<()> {
        match unsafe { sys::ws2811_render(&mut self.0) } {
            sys::ws2811_return_t::WS2811_SUCCESS => Ok(()),
            error => Err(Error(error)),
        }
    }

    /// Renders the given buffers instead of the buffers held by this [`Controller`] instance.
    /// This is a more effective way of rendering a pre-filled buffer than copying it into
    /// the internal buffer available via `Controller::raw_buffer`.
    ///
    /// # Panics
    ///
    /// Panics if any of the `&[Led]` slices are not the same length as the corresponding
    /// [`Channel`]s `led_count` as given to the [`Channel`] constructor.
    pub fn render_buffer(&mut self, buffers: [&[Led]; NUM_CHANNELS]) -> Result<()> {
        let original_leds_ptrs: [*mut sys::ws2811_led_t; NUM_CHANNELS] =
            [self.0.channel[0].leds, self.0.channel[1].leds];

        assert_eq!(self.0.channel[0].count as usize, buffers[0].len());
        assert_eq!(self.0.channel[1].count as usize, buffers[1].len());

        self.0.channel[0].leds = buffers[0].as_ptr() as *mut _;
        self.0.channel[1].leds = buffers[1].as_ptr() as *mut _;

        let render_result = self.render();

        self.0.channel[0].leds = original_leds_ptrs[0];
        self.0.channel[1].leds = original_leds_ptrs[1];

        render_result
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        unsafe { sys::ws2811_fini(&mut self.0) };
    }
}
