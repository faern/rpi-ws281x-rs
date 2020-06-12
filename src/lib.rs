use std::convert::TryFrom;
use std::os::raw::c_int;
use std::ptr;

pub use rpi_ws281x_sys as sys;

mod error;
pub use error::{Error, Result};
mod led;
pub use led::Led;

/// A channel represents one GPIO pin sending a signal to one strip of LEDs.
/// There can be up to `sys::RPI_PWM_CHANNELS` `Channel`s on one [`Controller`].
///
/// The channel instance is handed over to [`Builder::channel`].
pub struct Channel(sys::ws2811_channel_t);

impl Channel {
    /// Creates a new channel on the given GPIO pin with the given amount of LEDs.
    pub fn new(gpio_pin: u8, led_count: u16) -> Self {
        Self(sys::ws2811_channel_t {
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

    /// Returns a disabled LED strip channel. Used internally.
    fn disabled_raw() -> sys::ws2811_channel_t {
        sys::ws2811_channel_t {
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
        }
    }
}

/// A builder for [`Controller`] structs. Sets up and initializes the hardware for controlling the
/// LEDs and returns a controller that is then used for actually rendering anything to the LEDs.
pub struct Builder(sys::ws2811_t);

impl Builder {
    /// Creates a new [`Controller`] builder using the given DMA channel.
    pub fn new(dma_channel: u8) -> Self {
        Self(sys::ws2811_t {
            render_wait_time: 0,
            device: ptr::null_mut(),
            rpi_hw: ptr::null(),
            freq: sys::WS2811_TARGET_FREQ,
            dmanum: i32::from(dma_channel),
            channel: [Channel::disabled_raw(), Channel::disabled_raw()],
        })
    }

    /// Sets the frequency in Hz that the controller will output data at.
    pub fn freq(mut self, freq: u32) -> Self {
        self.0.freq = freq;
        self
    }

    /// Sets a channel on the controller.
    ///
    /// # Panics
    ///
    /// Panics if `index >= rpi_ws281x_sys::RPI_PWM_CHANNELS`.
    pub fn channel(mut self, index: usize, channel: Channel) -> Self {
        self.0.channel[index] = channel.0;
        self
    }

    /// Tries to initialize the hardware to control LEDs in the way the builder is configured.
    /// Returns the [`Controller`] on success.
    pub fn build(mut self) -> Result<Controller> {
        assert_eq!(usize::try_from(sys::RPI_PWM_CHANNELS).unwrap(), self.0.channel.len());
        match unsafe { sys::ws2811_init(&mut self.0) } {
            sys::ws2811_return_t::WS2811_SUCCESS => Ok(Controller(self.0)),
            error => Err(Error(error)),
        }
    }
}

/// A ws281x LED controller. Instances of this type are created via the [`Builder`].
pub struct Controller(sys::ws2811_t);

impl Controller {
    /// Returns a mutable slice where all the LED values can be set directly.
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
    pub fn render_buffer(&mut self, buffers: [&[Led]; sys::RPI_PWM_CHANNELS as usize]) -> Result<()> {
        let original_leds_ptrs: [*mut sys::ws2811_led_t; sys::RPI_PWM_CHANNELS as usize] =
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

/// Represents the type of LEDs that should be controlled. This controls the order that the
/// separate color channels are transmitted in over the wire, and if the white channel is
/// present or not.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum StripType {
    Rgb = sys::WS2811_STRIP_RGB,
    Rbg = sys::WS2811_STRIP_RBG,
    Grb = sys::WS2811_STRIP_GRB,
    Gbr = sys::WS2811_STRIP_GBR,
    Brg = sys::WS2811_STRIP_BRG,
    Bgr = sys::WS2811_STRIP_BGR,
    Rgbw = sys::SK6812_STRIP_RGBW,
    Rbgw = sys::SK6812_STRIP_RBGW,
    Grbw = sys::SK6812_STRIP_GRBW,
    Gbrw = sys::SK6812_STRIP_GBRW,
    Brgw = sys::SK6812_STRIP_BRGW,
    Bgrw = sys::SK6812_STRIP_BGRW,
}

impl StripType {
    fn as_raw(self) -> i32 {
        i32::try_from(self as u32).unwrap()
    }
}
