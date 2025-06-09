extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=include/Kinect.h");
    println!("cargo:rerun-if-changed=include/Kinect.INPC.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-xc") // Treat the following header as C
        .header("include/Kinect.h")
        .clang_arg("-xc") // Treat the following header as C
        .header("include/Kinect.INPC.h")
        .clang_arg("--target=x86_64-pc-windows-msvc")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(trivial_casts)]")
        .raw_line("#![allow(clippy::all)]") // Allow all clippy lints for generated code
        .raw_line("#![allow(unsafe_op_in_unsafe_fn)]") // Allow unsafe operations in unsafe functions
        .raw_line("#![allow(unused_imports)]") // Allow unused imports
        .rustified_enum(".*") // Generate Rust-style enums
        .derive_default(true) // Attempt to derive Default for structs
        .derive_debug(true) // Attempt to derive Debug for structs
        .no_default("tagMONITORINFOEXA") // Don't derive Default for this struct
        .no_default("tagMONITORINFOEXW") // Don't derive Default for this struct (precaution)
        // Add any necessary clang arguments here if specific SDK versions or paths are needed
        // For example: .clang_arg("-std=c++11") or .clang_arg("-IC:/path/to/windows_sdk_includes")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
