//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
//!
//! The build script also sets the linker flags to tell it which link script to use.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if !target.contains("thumbv7em-none-eabihf") && !target.contains("thumbv6m-none-eabi") {
        eprintln!("Warning: Unsupported target platform: {}", target);
        // 仅打印警告而不是 panic
    }
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    // println!("cargo:rerun-if-changed=memory.x");

    // Specify linker arguments.

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg-bins=--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    // println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    // 获取项目名称
    // let pkg_name = env::var("CARGO_PKG_NAME").expect("Failed to get CARGO_PKG_NAME");
    // let output_hex_name = format!("{}.hex", pkg_name);

    // // 获取项目根目录
    // let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR"));
    // let output_file_path = project_root.join(&output_hex_name);

    // use std::process::Command;
    // Command::new("cargo")
    //     .args(["objcopy", "--bin", &pkg_name, "--", "-O", "ihex", output_file_path.to_str().expect("Failed to convert path to string")])
    //     .status()
    //     .expect("Failed to execute cargo objcopy");

}
// fn main() {
//     println!("cargo:rustc-link-arg-bins=--nmagic");
//     println!("cargo:rustc-link-arg-bins=-Tlink.x");
//     println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
// }