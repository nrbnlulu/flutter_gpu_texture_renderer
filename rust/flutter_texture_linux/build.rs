use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
extern crate pkg_config;

fn run_cmd(cmd: &str, args: &[&str], cwd: Option<&PathBuf>) {
    println!("\x1b[32mRunning command: {} {:?}\x1b[0m", cmd, args);
    let mut command = std::process::Command::new(cmd);
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }

    let status = command.status().unwrap();
    if !status.success() {
        panic!("Failed to execute command: {:?}", command);
    }
}
lazy_static! {
    static ref GIT_ROOT: PathBuf = std::env::current_dir()
        .unwrap()
        .join("../..")
        .canonicalize()
        .unwrap();
}

fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("CWD: {:?}", cwd.to_str());
    println!("GIT_ROOT: {:?}", GIT_ROOT.to_str());

    // let ephemeral_dir = EXAMPLE_DIR.join("linux").join("flutter").join("ephemeral");
    let ephemeral_dir = Path::new("/home/dev/Documents/flutter_linux_3.24.0-stable/flutter/bin/cache/artifacts/engine/linux-x64");
    let mut cmake_args = vec!["-B", "./build", "-DRustBuild=ON"];
    let eph_arg = format!("-DEPHEMERAL_DIR={}", ephemeral_dir.to_str().unwrap());
    cmake_args.push(eph_arg.as_str());

    if cfg!(feature = "debug") {
        cmake_args.push("-DCMAKE_BUILD_TYPE=Debug");
    }
    let root_linux_path = GIT_ROOT.join("linux");
    run_cmd("cmake", &cmake_args, Some(&root_linux_path));
    println!("Running ninja");
    run_cmd("cmake", &["--build", "./build"], Some(&root_linux_path));

    // find and link against the created C library
    println!(
        "cargo:rustc-link-search={}",
        root_linux_path.join("build").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=flutter_gpu_texture_renderer_plugin");
    let c_lib_api_header = root_linux_path.join("include/flutter_gpu_texture_renderer/api.h");
    let bindings = bindgen::Builder::default()
        .header(c_lib_api_header.to_str().unwrap())
        .clang_arg(&format!("-I{}", ephemeral_dir.to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let _ = bindings
        .write_to_file(cwd.join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
