use std::env;
use std::path::Path;
use std::path::PathBuf;

static TURBO_INCLUDE_DIR: &'static str = "/usr/local/include";

fn generate_bindings() {
    let turbo_include_path = env::var("TURBO_INCLUDE").unwrap_or(TURBO_INCLUDE_DIR.to_string());
    //let output_path = env::var("OUT_DIR").unwrap();
    let output_path = "./src/";
    let generator = Generator {
        turbo_include_path: Path::new(&turbo_include_path),
        output_path: Path::new(&output_path),
    };

    let headers = ["jconfig","jerror","jpeglib"];
    generator.generate(&headers)
}

struct Generator<'a> {
    turbo_include_path: &'a Path,
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
            let header_path = self.turbo_include_path.join(
                PathBuf::from("header.h")
                    .with_file_name(name)
                    .with_extension("h"),
            );
            builder = builder.header(format!("{}", header_path.display()));
        }

        let bindings = builder
            .derive_default(true)
            .with_codegen_config(codegen_config)
            .generate_inline_functions(false)
            // If there are linking errors and the generated bindings have weird looking
            // #link_names (that start with \u{1}), the make sure to flip that to false.
            .trust_clang_mangling(false)
            .rustfmt_bindings(true)
            .rustfmt_configuration_file(Some(PathBuf::from("../rustfmt.toml")))
            .layout_tests(false)
            .ctypes_prefix("libc")
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(self.output_path.join("turbo_bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}

fn main() {
    generate_bindings();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=static=turbo");
    println!("cargo:rustc-link-search=native=/opt/libjpeg-turbo/include");
    println!("cargo:rustc-link-search=native=/opt/libjpeg-turbo/lib64");
}
