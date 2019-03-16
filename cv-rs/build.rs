use cmake::Config;

#[cfg(unix)]
mod unix {
    use std::env;

    pub fn opencv_include() -> String {
        if let Ok(dir) = env::var("OPENCV_DIR") {
            format!("{}/include/opencv4", dir)
        } else {
            "/usr/local/include/opencv4".into()
        }
    }

    pub fn opencv_link() {
        let cargo_rustc_link_search =
            env::var("OPENCV_LIB").unwrap_or("/usr/local/opt/opencv@4/lib".into());

        println!("cargo:rustc-link-search=native={}", cargo_rustc_link_search);
        println!("cargo:rustc-link-lib=opencv_core");
        println!("cargo:rustc-link-lib=opencv_features2d");
        println!("cargo:rustc-link-lib=opencv_xfeatures2d");
        println!("cargo:rustc-link-lib=opencv_highgui");
        println!("cargo:rustc-link-lib=opencv_img_hash");
        println!("cargo:rustc-link-lib=opencv_imgcodecs");
        println!("cargo:rustc-link-lib=opencv_imgproc");
        println!("cargo:rustc-link-lib=opencv_objdetect");
        println!("cargo:rustc-link-lib=opencv_videoio");
        println!("cargo:rustc-link-lib=opencv_video");
        if cfg!(feature = "cuda") {
            println!("cargo:rustc-link-lib=opencv_cudaobjdetect");
        }
    }
}

#[cfg(unix)]
use unix::*;

fn main() {
    let mut cfg = Config::new("native");
    let dst = cfg.build();
    println!("cargo:rerun-if-changed=./native");
    println!("cargo:rerun-if-changed=./build.rs");
    opencv_link();
    println!("cargo:rustc-link-lib=static=opencv-wrapper");
    println!(
        "cargo:rustc-link-search=native={}",
        format!("{}/opencv-wrapper/lib", dst.display())
    );
    println!(
        "cargo:rustc-link-search=native={}",
        format!("{}/opencv-wrapper/include", dst.display())
    );
}

/*
fn main() {
    let files = get_files("native");

    let mut opencv_config = cc::Build::new();
    opencv_config
        .cpp(true)
        .files(files)
        .include("native")
        .include(opencv_include());

    if cfg!(not(target_env = "msvc")) {
        opencv_config.flag("--std=c++11");
    }

    if cfg!(feature = "cuda") {
        let cuda_files = get_files("native/cuda");
        opencv_config.files(cuda_files);
    }

    opencv_config.compile("libopencv-wrapper.a");
    opencv_link();
}

fn get_files(path: &str) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok().map(|x| x.path()))
        .filter(|x| x.extension().map(|e| e == "cc").unwrap_or(false))
        .collect::<Vec<_>>()
}
*/
