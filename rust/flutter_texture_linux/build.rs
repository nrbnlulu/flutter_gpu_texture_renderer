use lazy_static::lazy_static;
use std::{
    env,
    path::{Path, PathBuf},
};

fn run_cmd(cmd: &str, args: &[&str], cwd: Option<&PathBuf>) {
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
    static ref EXAMPLE_DIR: PathBuf = GIT_ROOT.join("example");
}

fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("CWD: {:?}", cwd.to_str());
    println!("GIT_ROOT: {:?}", GIT_ROOT.to_str());
    println!("EXAMPLE_DIR: {:?}", EXAMPLE_DIR.to_str());
    assert!(EXAMPLE_DIR.exists());
    run_cmd(
        "flutter",
        &["build", "linux", "--release"],
        Some(&EXAMPLE_DIR),
    );
    let ephemeral_dir = EXAMPLE_DIR.join("linux").join("flutter").join("ephemeral");
    assert!(ephemeral_dir.exists());
    // now we can run cmake for the flutter_texture_linux
    // e.g cmake -B ./build -G Ninja -DRustBuild=ON -DEPHEMERAL_DIR=/home/dev/Desktop/OS/flutter_gpu_texture_renderer/example/linux/flutter/ephemeral
    let mut cmake_args = vec!["-B", "./build", "-G", "Ninja", "-DRustBuild=ON"];
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


    println!("cargo:rustc-link-lib=libflutter_gpu_texture_renderer_plugin");
    let c_lib_api_header = root_linux_path.join("include/flutter_gpu_texture_renderer/api.h");
    // now link against flutter and friends
    println!("cargo:rustc-link-search={}", ephemeral_dir.to_str().unwrap());


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(c_lib_api_header.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = bindings
        .write_to_file(cwd.join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
