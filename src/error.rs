use crate::ffi;
use std::ffi::CStr;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// A ws281x error.
pub struct Error(pub(crate) ffi::ws2811_return_t);

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ws281x::Error(\"{}\")", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char_ptr = unsafe { ffi::ws2811_get_return_t_str(self.0) };
        // SAFETY: We trust that the C library returns a valid string pointer
        let msg = unsafe { CStr::from_ptr(char_ptr) }.to_string_lossy();
        msg.fmt(f)
    }
}

impl std::error::Error for Error {}
