use std::env;
use std::path::Path;

#[cfg(target_os = "macos")]
static TURBO_INCLUDE_DIR: &'static str = "/usr/local/opt/jpeg-turbo";

#[cfg(target_os = "linux")]
static TURBO_INCLUDE_DIR: &'static str = "/opt/libjpeg-turbo";

fn main() {
    println!("cargo:rustc-link-lib=jpeg");
    // hzy: Don't use SPDK_DIR as environment variable here as SPDK 18.07 rely on this variable to
    // build (i.e. will fail the SPDK build if we use the same environment variable here)

    let turbo_include_path = env::var("TURBO_INCLUDE").unwrap_or(TURBO_INCLUDE_DIR.to_string());

    let include_path_jpeg_dir = format!("-I{}/include", turbo_include_path);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg(include_path_jpeg_dir)
        .derive_default(true)
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .blacklist_type("IPPORT_.*") // https://github.com/rust-lang-nursery/rust-bindgen/issues/687
        .blacklist_type("max_align_t") // https://github.com/rust-lang-nursery/rust-bindgen/issues/550
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = Path::new("./src/");
    bindings
        .write_to_file(out_path.join("jpeg_bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=./build.rs");
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/opt/jpeg-turbo/lib");
    } else {
        println!("cargo:rustc-link-search=native=/opt/libjpeg-turbo/lib64");
    }
}
