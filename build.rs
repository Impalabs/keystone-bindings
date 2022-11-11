#[cfg(feature = "build-from-src")]
use cmake;
#[cfg(feature = "use-system-lib")]
use pkg_config;

#[cfg(feature = "build-from-src")]
fn build_keystone() {
    let dest = cmake::Config::new("keystone")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("BUILD_LIBS_ONLY", "1")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("LLVM_TARGETS_TO_BUILD", "all")
        // Prevent python from leaving behind `.pyc` files which break `cargo package`
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dest.display());
    println!("cargo:rustc-link-lib=keystone");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=shell32");
    }
}

fn main() {
    if cfg!(feature = "use-system-lib") {
        #[cfg(feature = "use-system-lib")]
        pkg_config::find_library("keystone").expect("Could not find system keystone");
    } else {
        #[cfg(feature = "build-from-src")]
        build_keystone();
    }
}
