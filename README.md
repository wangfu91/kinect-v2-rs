# Kinect V2 Rust Bindings 🦀🎮

[![Crates.io](https://img.shields.io/crates/v/kinect-v2.svg)](https://crates.io/crates/kinect-v2)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A Rust binding for the Kinect V2 Windows SDK. This project enables you to access Kinect V2 sensor data (color, depth, infrared, body, audio, etc.) from Rust on Windows.

## Features ✨
- Safe and idiomatic Rust API for Kinect V2
- Access to color, depth, infrared, body, audio, and multi-source frames
- Low-level FFI bindings to the official Kinect V2 SDK
- Examples and documentation

## Requirements ⚠️
- **Windows** (Kinect V2 SDK is Windows-only)
- Kinect V2 sensor and adapter
- Kinect for Windows SDK 2.0 installed
- Rust 1.70+ (2024 edition)

## Getting Started 🚀

Add to your `Cargo.toml`:

```toml
[dependencies]
kinect-v2 = "0.1" # Replace with the latest version
```

## Example Usage 📝

```rust
use kinect_v2::color_capture::ColorCapture;

fn main() {
    let color_capture = color_capture::ColorFrameCapture::new().unwrap();
    for frame_result in color_capture.iter().unwrap() {
        match frame_result {
            Ok(frame_data) => {
                println!("Color frame size: {}x{}", frame_data.width, frame_data.height);
            }
            Err(e) => {
                eprintln!("Error capturing color frame: {}", e);
            }
        }
    }
}
```

More examples can be found in the `examples/` directory.

## Building 🛠️

```powershell
# Clone the repo
$ git clone https://github.com/wangfu91/kinect-v2-rs.git
$ cd kinect-v2-rs

# Build the project
$ cargo build --release
```

## Project Structure 📁
- `kinect-v2/` — Idiomatic Rust wrapper
- `kinect-v2-sys/` — Safe Rust API and raw FFI bindings

## License 📄

MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgements 🙏
- [Microsoft Kinect for Windows SDK 2.0](https://www.microsoft.com/en-us/download/details.aspx?id=44561)
- [windows-rs](https://github.com/microsoft/windows-rs)
- [bindgen](https://github.com/rust-lang/rust-bindgen)

---

Feel free to open issues or PRs! Happy hacking! 🚀
