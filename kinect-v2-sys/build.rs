use std::path::Path;

fn main() {
    let kinect_sdk_path = Path::new("C:\\Program Files\\Microsoft SDKs\\Kinect\\v2.0_1409");
    #[cfg(target_arch = "x86")]
    let lib_path = kinect_sdk_path.join("Lib").join("x86");
    #[cfg(target_arch = "x86_64")]
    let lib_path = kinect_sdk_path.join("Lib").join("x64");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=Kinect20");
}

/*
 *
 * Use the following code to generate bindings for the Kinect V2 SDK on the fly.
 * Make sure to have the Kinect V2 SDK installed first.
 *
extern crate bindgen;
fn main() {
    let kinect_sdk_path = Path::new("C:\\Program Files\\Microsoft SDKs\\Kinect\\v2.0_1409");
    #[cfg(target_arch = "x86")]
    let lib_path = kinect_sdk_path.join("Lib").join("x86");
    #[cfg(target_arch = "x86_64")]
    let lib_path = kinect_sdk_path.join("Lib").join("x64");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=Kinect20");

    println!("cargo:rerun-if-changed=include/wrapper.h");

    let inc_path = kinect_sdk_path.join("inc");
    let mut inc_arg = String::from("-I");
    inc_arg.push_str(inc_path.to_str().unwrap());

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .clang_arg(inc_arg.as_str())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rustified_enum(".*") // Generate Rust-style enums
        .derive_default(true) // Attempt to derive Default for structs
        .derive_debug(false) // Attempt to derive Debug for structs
        .derive_copy(false)
        .no_default("tagMONITORINFOEXA") // Don't derive Default for this struct
        .no_default("tagMONITORINFOEXW") // Don't derive Default for this struct (precaution)
        .blocklist_type("HRESULT") // Prevent bindgen from generating HRESULT
        .raw_line("pub use windows::core::HRESULT;") // Use the HRESULT from the windows crate
        .blocklist_type("HANDLE") // Prevent bindgen from generating UINT
        .raw_line("pub type HANDLE = windows::Win32::Foundation::HANDLE;") // Use the HANDLE from the windows crate
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let out_path = Path::new(&out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
*/
