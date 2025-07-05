use std::env;

#[allow(unused_must_use)]
fn main() {
    // Check if we're building in debug mode
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    // Only apply Windows-specific MSVC flags on Windows
    if cfg!(target_os = "windows") && profile == "debug" {
        // Force Debug MSVC runtime for Debug builds
        println!("cargo:rustc-link-arg=/MDd");
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:MSVCRT");
        println!("cargo:rustc-link-arg=/DEFAULTLIB:MSVCRTD");
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:LIBCMT");
        println!("cargo:rustc-link-arg=/DEFAULTLIB:LIBCMTD");

        // Set environment variable to force Debug runtime
        println!("cargo:rustc-env=_ITERATOR_DEBUG_LEVEL=2");
    }

    cxx_build::bridge("src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
