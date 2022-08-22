extern crate bindgen;
use cmake::Config;
use std::env;
use std::path::PathBuf;

pub fn main() {
    let os = match build_target::target_os().unwrap() {
        build_target::Os::Android => "Android",
        build_target::Os::Linux => "Linux",
        build_target::Os::MacOs => "Darwin",
        build_target::Os::iOs => "Darwin",
        build_target::Os::Windows => "Windows",
        _ => panic!("Unsupported OS"),
    };
    let arch = match build_target::target_arch().unwrap() {
        build_target::Arch::X86 => "x86",
        build_target::Arch::X86_64 => "x86_64",
        build_target::Arch::AARCH64 => "aarch64",
        _ => panic!("Unsupported arch")
    };
    let ndk = env::var("ANDROID_NDK_HOME").unwrap_or_default();

    let dst = Config::new("third_party/cpuinfo")
        .define("CPUINFO_LIBRARY_TYPE", "static")
        .define("CPUINFO_BUILD_UNIT_TESTS", "OFF")
        .define("CPUINFO_BUILD_MOCK_TESTS", "OFF")
        .define("CPUINFO_BUILD_BENCHMARKS", "OFF")
        .define("CPUINFO_BUILD_PKG_CONFIG", "OFF")
        .define("CMAKE_SYSTEM_NAME", os)
        .define("CMAKE_SYSTEM_PROCESSOR", arch)
        .define("ANDROID_NDK", ndk)
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
