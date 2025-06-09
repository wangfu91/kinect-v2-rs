extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=include/Kinect.h");
    println!(
        "cargo:rustc-link-search=native=C:\\Program Files\\Microsoft SDKs\\Kinect\\v2.0_1409\\Lib\\x64"
    );
    println!("cargo:rustc-link-lib=Kinect20");

    let bindings = bindgen::Builder::default()
        .header("include/Kinect.h")
        .allowlist_file("include/Kinect.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rustified_enum(".*") // Generate Rust-style enums
        .derive_default(true) // Attempt to derive Default for structs
        .derive_debug(true) // Attempt to derive Debug for structs
        .no_default("tagMONITORINFOEXA") // Don't derive Default for this struct
        .no_default("tagMONITORINFOEXW") // Don't derive Default for this struct (precaution)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
