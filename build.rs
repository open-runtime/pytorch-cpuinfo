extern crate bindgen;
use cmake::Config;
use std::env;
use std::path::PathBuf;

pub fn main() {
    let dst = Config::new("third_party/cpuinfo")
        .define("CPUINFO_LIBRARY_TYPE", "static")
        .define("CPUINFO_BUILD_UNIT_TESTS", "OFF")
        .define("CPUINFO_BUILD_MOCK_TESTS", "OFF")
        .define("CPUINFO_BUILD_BENCHMARKS", "OFF")
        .define("CPUINFO_BUILD_PKG_CONFIG", "OFF")
        .build();

    let bindings = bindgen::Builder::default()
        .header("third_party/cpuinfo/include/cpuinfo.h")
        .generate()
        .expect("Failed to generate Rust bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cpuinfo.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}/lib/", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64/", dst.display());
    println!("cargo:rustc-link-lib=static=cpuinfo");
    println!("cargo:rustc-link-lib=static=clog");
}
