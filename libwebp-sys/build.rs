extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;
use bindgen::Builder;
use std::fs;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(n) => n,
        Err(e) => panic!("\n{} failed with {}\n", stringify!($e), e),
    })
}

#[allow(dead_code)]
fn err_to_panic<T, E: Display>(result: Result<T, E>) -> T {
    match result {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    }
}

fn run(command: &mut Command) {
    println!("{:?}", command);
    let string = format!("{:?}", command);
    let status = err_to_panic(command.status());
    if !status.success() {
        panic!("`{}` did not execute successfully", string);
    }
}

 pub fn print_libs(out_dir: &Path) {
        let out_str = out_dir.to_str().unwrap();
        println!("cargo:rustc-link-search=native={}/lib", out_str);
}

fn cp_r(dir: &Path, dst: &Path) {
    for entry in t!(fs::read_dir(dir)) {
        let entry = entry.expect("entry");
        let path = entry.path();
        let dst = dst.join(path.file_name().unwrap());
        if t!(fs::metadata(&path)).is_file() {
            t!(fs::copy(path, dst));
        } else {
            t!(fs::create_dir_all(&dst));
            cp_r(&path, &dst);
        }
    }
}


fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let build = PathBuf::from(&env::var("OUT_DIR").unwrap()).join("build");
    let src = PathBuf::from(&env::current_dir().unwrap().join("libwebp"));
    println!("{:?}",src);
    err_to_panic(env::set_current_dir(src));
    run(& mut Command::new("./autogen.sh"));
    run(& mut Command::new("./configure")
     .args(&["--prefix", out_path.to_str().unwrap()])
     ); // Install on the outdir
    run(Command::new("make").arg("install"));
    println!("cargo:rustc-link-lib=static=webp");
    print_libs(&out_path);

    let bindings = Builder::default()
        .header(out_path.join("include").join("webp").join("decode.h").to_str().unwrap())
        .header(out_path.join("include").join("webp").join("encode.h").to_str().unwrap())
        .header(out_path.join("include").join("webp").join("types.h").to_str().unwrap())
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("webp_bindings.rs"))
        .expect("Couldn't write bindings!");
}