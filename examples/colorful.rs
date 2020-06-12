use std::f32::consts::PI;
use std::thread;
use std::time::{Duration, Instant};

use rpi_ws281x::{Led, StripType};

/// Full circle.
const TAU: f32 = 2.0 * PI;

const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);

const ANGLE_DIFF_RED: f32 = TAU / 200.0;
const ANGLE_DIFF_GREEN: f32 = -TAU / 140.0;
const ANGLE_DIFF_BLUE: f32 = TAU / 100.0;

const STD_DEV_RED: f32 = 0.5;
const STD_DEV_GREEN: f32 = 0.2;
const STD_DEV_BLUE: f32 = 0.12;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let led_count: u16 = 19;

    let mut strip = rpi_ws281x::Builder::new(10)
        .channel(
            0,
            rpi_ws281x::Channel::new(10, led_count)
                .strip_type(StripType::Grb)
                .brightness(255),
        )
        .build()?;

    let clear = vec![Led::OFF; led_count as usize];

    // The center of each gauss distributed intensity curve.
    let mut angle_red = 0.0;
    let mut angle_green = 0.0;
    let mut angle_blue = 0.0;

    let mut next_frame = Instant::now() + FRAME_DURATION;
    loop {
        // Clear the buffer
        strip.buffer(0).copy_from_slice(&clear[..]);

        // Draw the gauss distribution of colors centered over their current angle
        draw_gauss(strip.buffer(0), Led::RED, angle_red, STD_DEV_RED);
        draw_gauss(strip.buffer(0), Led::GREEN, angle_green, STD_DEV_GREEN);
        draw_gauss(strip.buffer(0), Led::BLUE, angle_blue, STD_DEV_BLUE);

        strip.render()?;

        angle_red = (angle_red + ANGLE_DIFF_RED) % TAU;
        angle_green = (angle_green + ANGLE_DIFF_GREEN) % TAU;
        angle_blue = (angle_blue + ANGLE_DIFF_BLUE) % TAU;

        if let Some(t) = next_frame.checked_duration_since(Instant::now()) {
            thread::sleep(t);
        } else {
            eprintln!("Rendering too slow to keep desired FPS");
        }
        next_frame += FRAME_DURATION;
    }
}

fn draw_gauss(leds: &mut [Led], color: Led, peak: f32, std_dev: f32) {
    let variance = std_dev.powi(2);
    let peak = peak.rem_euclid(TAU);
    let led_count = leds.len();
    for (i, led) in leds.iter_mut().enumerate() {
        let i_rad = (i as f32 / led_count as f32) * TAU;
        let diff_rad = ((peak - i_rad) + PI).rem_euclid(TAU) - PI;
        let scale = gauss(diff_rad, variance);
        *led += color * scale;
    }
}

fn gauss(x: f32, variance: f32) -> f32 {
    (x.powi(2) / (-2.0 * variance)).exp()
}
