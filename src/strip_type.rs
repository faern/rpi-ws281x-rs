use crate::sys;
use std::convert::TryFrom;
use std::fmt;

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
    pub(crate) fn as_raw(self) -> i32 {
        i32::try_from(self as u32).unwrap()
    }
}

impl std::str::FromStr for StripType {
    type Err = InvalidStripTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rgb" => Ok(Self::Rgb),
            "rbg" => Ok(Self::Rbg),
            "grb" => Ok(Self::Grb),
            "gbr" => Ok(Self::Gbr),
            "brg" => Ok(Self::Brg),
            "bgr" => Ok(Self::Bgr),
            "rgbw" => Ok(Self::Rgbw),
            "rbgw" => Ok(Self::Rbgw),
            "grbw" => Ok(Self::Grbw),
            "gbrw" => Ok(Self::Gbrw),
            "brgw" => Ok(Self::Brgw),
            "bgrw" => Ok(Self::Bgrw),
            _ => Err(InvalidStripTypeError(())),
        }
    }
}

/// An error representing trying to parse a [`StripType`] from a string that does not represent
/// a valid LED strip type.
#[derive(Debug)]
pub struct InvalidStripTypeError(());

impl fmt::Display for InvalidStripTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Invalid LED strip type".fmt(f)
    }
}

impl std::error::Error for InvalidStripTypeError {}
