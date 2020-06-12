use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=WS281X_LIB_DIR");
    match env::var_os("WS281X_LIB_DIR").map(PathBuf::from) {
        Some(lib_dir) => {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=ws281x");
        }
        None => cc::Build::new()
            .file("rpi_ws281x/ws2811.c")
            .file("rpi_ws281x/dma.c")
            .file("rpi_ws281x/pcm.c")
            .file("rpi_ws281x/pwm.c")
            .file("rpi_ws281x/mailbox.c")
            .file("rpi_ws281x/rpihw.c")
            .compile("ws281x"),
    }
}
