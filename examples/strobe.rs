use std::thread;
use std::time::Duration;

use ws281x::Led;

const INTERVAL: Duration = Duration::from_millis(300);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let led_count: u16 = 19;

    let on = vec![Led::ON; led_count as usize];
    let off = vec![Led::OFF; led_count as usize];

    let mut strip = ws281x::Builder::new(10)
        .channel(0, ws281x::Channel::new(10, led_count).brightness(100))
        .build()?;

    loop {
        strip.render_buffer([&on[..], &[]])?;
        thread::sleep(INTERVAL);

        strip.render_buffer([&off[..], &[]])?;
        thread::sleep(INTERVAL);
    }
}
