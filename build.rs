fn main() {
    cc::Build::new()
        .file("rpi_ws281x/ws2811.c")
        .file("rpi_ws281x/dma.c")
        .file("rpi_ws281x/pcm.c")
        .file("rpi_ws281x/pwm.c")
        .file("rpi_ws281x/mailbox.c")
        .file("rpi_ws281x/rpihw.c")
        .compile("ws281x");
}
