use std::env;
use std::path::Path;
use std::path::PathBuf;

static WEBP_INCLUDE_DIR: &'static str = "/usr/local/include";

fn generate_bindings() {
    let webp_include_path = env::var("WEBP_INCLUDE").unwrap_or(WEBP_INCLUDE_DIR.to_string());
    //let output_path = env::var("OUT_DIR").unwrap();
    let output_path = "./src/";
    let generator = Generator {
        webp_include_path: Path::new(&webp_include_path),
        output_path: Path::new(&output_path),
    };

    let headers = ["encode", "decode", "types", "mux"];
    generator.generate(&headers)
}

struct Generator<'a> {
    webp_include_path: &'a Path,
    output_path: &'a Path,
}

impl<'a> Generator<'a> {
    fn generate(&self, names: &[&str]) {
        let mut codegen_config = bindgen::CodegenConfig::empty();
        codegen_config.set(bindgen::CodegenConfig::FUNCTIONS, true);
        codegen_config.set(bindgen::CodegenConfig::TYPES, true);
        codegen_config.set(bindgen::CodegenConfig::CONSTRUCTORS, true);
        codegen_config.set(bindgen::CodegenConfig::METHODS, true);

        let mut builder = bindgen::builder();

        for name in names {
            let header_path = self.webp_include_path.join(
                PathBuf::from("webp/header.h")
                    .with_file_name(name)
                    .with_extension("h"),
            );
            builder = builder.header(format!("{}", header_path.display()));
        }
        let png_library_path = Path::new("/usr/local/include");
        let zlib_library_path = Path::new("/usr/local/opt/zlib/include");
        let jpeg_library_path = if cfg!(target_os = "linux") {
            Path::new("/opt/libjpeg-turbo/include")
        } else {
            Path::new("/usr/local/include")
        };
        cc::Build::new()
            .define("WEBP_HAVE_PNG", None)
            .define("WEBP_HAVE_JPEG", None)
            .file("pngwebp/util.c")
            .file("pngwebp/jpegdec.c")
            .file("pngwebp/pngdec.c")
            .file("pngwebp/imageio_util.c")
            .file("pngwebp/metadata.c")
            .file("pngwebp/metadata_write.c")
            .file("pngwebp/metadata_read.c")
            .include(jpeg_library_path)
            .include(png_library_path)
            .include(zlib_library_path)
            .compile("libpngdec.a");

        let bindings = builder
            .derive_default(true)
            .with_codegen_config(codegen_config)
            .header("png.h")
            .header("pngwebp/jpegdec.h")
            .header("pngwebp/pngdec.h")
            .header("pngwebp/imageio_util.h")
            .header("pngwebp/metadata.h")
            .header("pngwebp/metadata_write.h")
            .header("pngwebp/metadata_read.h")
            .generate_inline_functions(true)
            // If there are linking errors and the generated bindings have weird looking
            // #link_names (that start with \u{1}), the make sure to flip that to false.
            .trust_clang_mangling(true)
            .rustfmt_bindings(true)
            .rustfmt_configuration_file(Some(PathBuf::from("../rustfmt.toml")))
            .layout_tests(true)
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(self.output_path.join("webp_bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}

fn main() {
    println!("cargo:rustc-link-lib=static=png");
    println!("cargo:rustc-link-lib=static=jpeg");
    println!("cargo:rustc-link-lib=static=webp");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=exif");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=pngwebp");
    println!("cargo:rerun-if-changed=pngwebp/jpegdec.h");
    println!("cargo:rerun-if-changed=pngwebp/jpegdec.c");
    println!("cargo:rerun-if-changed=pngwebp/metadata_write.h");
    println!("cargo:rerun-if-changed=pngwebp/metadata_write.c");
    println!("cargo:rerun-if-changed=pngwebp/metadata_read.h");
    println!("cargo:rerun-if-changed=pngwebp/metadata_read.c");

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=native=/opt/libjpeg-turbo/lib64");
        println!("cargo:rustc-link-search=/opt/libjpeg-turbo/include");
    }
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-search=/usr/local/include");
    println!("cargo:rustc-link-search=native=/usr/local/include/webp");
    println!("cargo:rustc-link-search=native=/usr/include/");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/local/opt/zlib/lib");
    println!("cargo:rustc-link-search=native=/usr/local/opt/zlib/include");
    generate_bindings();
}
