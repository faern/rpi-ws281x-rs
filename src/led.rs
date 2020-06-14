use std::fmt;use crate::sys;

/// Represents a single LED on a strip of ws281x LEDs. Contains a one byte value for the brightness
/// of the red, green, blue and white channels of the LED. The library represents an LED strip
/// as a slice of these structs.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Led(sys::ws2811_led_t);

impl Led {
    /// All channels turned off.
    pub const OFF: Self = Self::new(0, 0, 0, 0);

    /// All channels on max brightness.
    pub const ON: Self = Self::new(255, 255, 255, 255);

    /// White channel full on, but all RGB channels off.
    pub const WHITE: Self = Self::new(255, 0, 0, 0);

    /// All RGB channels full on, but white channel off.
    pub const RGB_WHITE: Self = Self::new(0, 255, 255, 255);

    /// Red channel fully on, all other channels off.
    pub const RED: Self = Self::new(0, 255, 0, 0);

    /// Green channel fully on, all other channels off.
    pub const GREEN: Self = Self::new(0, 0, 255, 0);

    /// Blue channel fully on, all other channels off.
    pub const BLUE: Self = Self::new(0, 0, 0, 255);

    /// Creates a new [`Led`] instance with the given channel brightness values.
    #[inline(always)]
    pub const fn new(white: u8, red: u8, green: u8, blue: u8) -> Self {
        Self(sys::ws2811_led_t::from_be_bytes([white, red, green, blue]))
    }

    /// Creates a new [`Led`] from float values. The floats are still expected to be in the range
    /// `0-255` and will be clamped to that range.
    fn from_f32s(white: f32, red: f32, green: f32, blue: f32) -> Self {
        let w = white.round().min(255.0).max(0.0) as u8;
        let r = red.round().min(255.0).max(0.0) as u8;
        let g = green.round().min(255.0).max(0.0) as u8;
        let b = blue.round().min(255.0).max(0.0) as u8;
        Self::new(w, r, g, b)
    }

    /// Returns the brightness value for the white channel.
    pub const fn white(&self) -> u8 {
        let [w, _r, _g, _b] = self.0.to_be_bytes();
        w
    }

    /// Returns the brightness value for the red channel.
    pub const fn red(&self) -> u8 {
        let [_w, r, _g, _b] = self.0.to_be_bytes();
        r
    }

    /// Returns the brightness value for the green channel.
    pub const fn green(&self) -> u8 {
        let [_w, _r, g, _b] = self.0.to_be_bytes();
        g
    }

    /// Returns the brightness value for the blue channel.
    pub const fn blue(&self) -> u8 {
        let [_w, _r, _g, b] = self.0.to_be_bytes();
        b
    }
}

impl fmt::Debug for Led {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [w, r, g, b] = self.0.to_be_bytes();
        f.debug_struct("Led")
            .field("w", &w)
            .field("r", &r)
            .field("g", &g)
            .field("b", &b)
            .finish()
    }
}

impl core::ops::Add for Led {
    type Output = Self;

    /// Adds together the brightness values for two `Led`s. This operation uses is saturating
    /// addition. So if two `Led`s are added together and a channel value becomes more than 255,
    /// the resulting `Led` will have brightness 255 for that channel.
    fn add(self, rhs: Self) -> Self {
        let [w1, r1, g1, b1] = self.0.to_be_bytes();
        let [w2, r2, g2, b2] = rhs.0.to_be_bytes();
        Self::new(
            w1.saturating_add(w2),
            r1.saturating_add(r2),
            g1.saturating_add(g2),
            b1.saturating_add(b2),
        )
    }
}

impl core::ops::AddAssign<Led> for Led {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::Mul<f32> for Led {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        let [w, r, g, b] = self.0.to_be_bytes();
        let w = w as f32 * rhs;
        let r = r as f32 * rhs;
        let g = g as f32 * rhs;
        let b = b as f32 * rhs;
        Self::from_f32s(w, r, g, b)
    }
}

impl core::ops::MulAssign<f32> for Led {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl From<sys::ws2811_led_t> for Led {
    fn from(raw: sys::ws2811_led_t) -> Self {
        Self(raw)
    }
}

impl From<Led> for sys::ws2811_led_t {
    fn from(led: Led) -> sys::ws2811_led_t {
        led.0
    }
}

#[cfg(test)]
mod tests {
    use super::Led;

    #[test]
    fn mul() {
        let led = Led::new(0, 1, 100, 200);

        assert_eq!(u32::from(led * 0.0), 0);
        assert_eq!(led * 1.0, led);
        assert_eq!(led * -9.1, Led::new(0, 0, 0, 0));
        assert_eq!(led * 0.99, Led::new(0, 1, 99, 198));
        assert_eq!(led * 1.499, Led::new(0, 1, 150, 255));
        assert_eq!(led * 1.5, Led::new(0, 2, 150, 255));
        assert_eq!(led * 2.0, Led::new(0, 2, 200, 255));
        assert_eq!(led * 1000.0, Led::new(0, 255, 255, 255));
        assert_eq!(led * f32::INFINITY, Led::MAX);
    }

    #[test]
    fn add() {
        let led = Led::new(0, 1, 100, 200);

        assert_eq!(led + Led::new(0, 0, 0, 0), led);
        assert_eq!(led + Led::new(1, 2, 3, 4), Led::new(1, 3, 103, 204));

        let bright = Led::new(201, 202, 203, 204);
        assert_eq!(bright + bright, Led::MAX);
    }

    #[test]
    fn get_channels() {
        let led = Led::new(1, 2, 100, 200);
        assert_eq!(led.white(), 1);
        assert_eq!(led.red(), 2);
        assert_eq!(led.green(), 100);
        assert_eq!(led.blue(), 200);
    }
}
