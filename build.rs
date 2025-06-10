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
