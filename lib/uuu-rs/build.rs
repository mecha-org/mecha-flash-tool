#![allow(improper_ctypes)]

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    #[cfg(not(target_os = "windows"))]
    {
        let dst = Config::new("./vendor/mfgtools/")
            .define("CMAKE_BUILD_TYPE", "Release")
            .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
            .build();

        println!("cargo:rustc-link-search={}/build/libuuu", dst.display());
    }

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=/opt/homebrew/lib");
        println!("cargo:rustc-link-lib=c++");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=./vendor/mfgtools/msvc/x64/Release/");
        println!("cargo:rustc-link-search=./vendor/mfgtools/libusb/build/v143/x64/Release/lib/");
        println!("cargo:rustc-link-search=./vendor/mfgtools/zstd/build/VS2010/bin/x64_Release");
        println!("cargo:rustc-link-lib=dylib=libusb-1.0");
        println!("cargo:rustc-link-lib=dylib=bzip2");
        println!("cargo:rustc-link-lib=dylib=tinyxml2");
        println!("cargo:rustc-link-lib=dylib=zlib");
        println!("cargo:rustc-link-lib=dylib=libzstd");
        println!("cargo:rustc-link-lib=dylib=libuuu");
    }
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=dylib=usb-1.0");
        println!("cargo:rustc-link-lib=dylib=crypto");
        println!("cargo:rustc-link-lib=dylib=z");
        println!("cargo:rustc-link-lib=dylib=zstd");
        println!("cargo:rustc-link-lib=dylib=bz2");
        println!("cargo:rustc-link-lib=dylib=tinyxml2");
        println!("cargo:rustc-link-lib=dylib=ssl");
        println!("cargo:rustc-link-lib=static=uuc_s");
    }

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-I./vendor/mfgtools/libuuu")
        .clang_arg("-I./vendor/mfgtools/libusb/libusb")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
