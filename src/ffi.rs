/* automatically generated by rust-bindgen */

// Generated using bindgen 0.54.0
// Generated against rpi_ws281x 6a720cbd42d30be28e0f5c5ff6b1c00a4588a29b

pub const RPI_PWM_CHANNELS: u32 = 2;
pub const WS2811_TARGET_FREQ: u32 = 800000;
pub const SK6812_STRIP_RGBW: u32 = 403703808;
pub const SK6812_STRIP_RBGW: u32 = 403701768;
pub const SK6812_STRIP_GRBW: u32 = 403181568;
pub const SK6812_STRIP_GBRW: u32 = 403177488;
pub const SK6812_STRIP_BRGW: u32 = 402657288;
pub const SK6812_STRIP_BGRW: u32 = 402655248;
pub const WS2811_STRIP_RGB: u32 = 1050624;
pub const WS2811_STRIP_RBG: u32 = 1048584;
pub const WS2811_STRIP_GRB: u32 = 528384;
pub const WS2811_STRIP_GBR: u32 = 524304;
pub const WS2811_STRIP_BRG: u32 = 4104;
pub const WS2811_STRIP_BGR: u32 = 2064;
#[repr(C)]
pub struct rpi_hw_t {
    pub type_: u32,
    pub hwver: u32,
    pub periph_base: u32,
    pub videocore_base: u32,
    pub desc: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ws2811_device {
    _unused: [u8; 0],
}
pub type ws2811_led_t = u32;
#[repr(C)]
pub struct ws2811_channel_t {
    pub gpionum: ::std::os::raw::c_int,
    pub invert: ::std::os::raw::c_int,
    pub count: ::std::os::raw::c_int,
    pub strip_type: ::std::os::raw::c_int,
    pub leds: *mut ws2811_led_t,
    pub brightness: u8,
    pub wshift: u8,
    pub rshift: u8,
    pub gshift: u8,
    pub bshift: u8,
    pub gamma: *mut u8,
}
#[repr(C)]
pub struct ws2811_t {
    pub render_wait_time: u64,
    pub device: *mut ws2811_device,
    pub rpi_hw: *const rpi_hw_t,
    pub freq: u32,
    pub dmanum: ::std::os::raw::c_int,
    pub channel: [ws2811_channel_t; 2usize],
}
impl ws2811_return_t {
    pub const WS2811_RETURN_STATE_COUNT: ws2811_return_t = ws2811_return_t::WS2811_ERROR_SPI_SETUP;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ws2811_return_t {
    WS2811_SUCCESS = 0,
    WS2811_ERROR_GENERIC = -1,
    WS2811_ERROR_OUT_OF_MEMORY = -2,
    WS2811_ERROR_HW_NOT_SUPPORTED = -3,
    WS2811_ERROR_MEM_LOCK = -4,
    WS2811_ERROR_MMAP = -5,
    WS2811_ERROR_MAP_REGISTERS = -6,
    WS2811_ERROR_GPIO_INIT = -7,
    WS2811_ERROR_PWM_SETUP = -8,
    WS2811_ERROR_MAILBOX_DEVICE = -9,
    WS2811_ERROR_DMA = -10,
    WS2811_ERROR_ILLEGAL_GPIO = -11,
    WS2811_ERROR_PCM_SETUP = -12,
    WS2811_ERROR_SPI_SETUP = -13,
    WS2811_ERROR_SPI_TRANSFER = -14,
}
extern "C" {
    pub fn ws2811_init(ws2811: *mut ws2811_t) -> ws2811_return_t;
}
extern "C" {
    pub fn ws2811_fini(ws2811: *mut ws2811_t);
}
extern "C" {
    pub fn ws2811_render(ws2811: *mut ws2811_t) -> ws2811_return_t;
}
extern "C" {
    pub fn ws2811_get_return_t_str(state: ws2811_return_t) -> *const ::std::os::raw::c_char;
}
