# Kinect V2 Rust Bindings 🦀🎮

kinect-v2: <br />
[![Crates.io](https://img.shields.io/crates/v/kinect-v2.svg)](https://crates.io/crates/kinect-v2)
[![Docs](https://docs.rs/kinect-v2/badge.svg)](https://docs.rs/kinect-v2)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

kinect-v2-sys: <br />
[![Crates.io](https://img.shields.io/crates/v/kinect-v2-sys.svg)](https://crates.io/crates/kinect-v2-sys)
[![Docs](https://docs.rs/kinect-v2-sys/badge.svg)](https://docs.rs/kinect-v2-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

**Kinect V2 Rust Bindings** provide safe and idiomatic Rust access to the Kinect V2 Windows SDK, enabling you to capture color, depth, infrared, body, audio, and multi-source frames from your Kinect V2 sensor on Windows.

> **Note:** This project currently supports **Windows** only. It uses the official Kinect V2 SDK, which is not available on other platforms. Future plans include cross-platform support using the [libfreenect2](https://github.com/OpenKinect/libfreenect2) library.

---

## Features ✨
- Safe and idiomatic Rust API for Kinect V2
- Access to color, depth, infrared, body, audio, and multi-source frames
- Low-level FFI bindings to the official Kinect V2 SDK
- Examples and documentation

## Requirements ⚠️
- **Windows** (Kinect V2 SDK is Windows-only)
- Kinect V2 sensor and adapter
- [Kinect for Windows SDK 2.0](https://www.microsoft.com/en-us/download/details.aspx?id=44561) installed
- Rust 1.70+ (2024 edition)

## Getting Started 🚀

Add to your `Cargo.toml`:

```toml
[dependencies]
kinect-v2 = "0.1" # Replace with the latest version
```

## Example Usage 📝

```rust
use kinect_v2::color_capture::ColorFrameCapture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let color_capture = ColorFrameCapture::new()?;
    for frame_result in color_capture.iter()? {
        match frame_result {
            Ok(frame_data) => {
                println!("Color frame size: {}x{}, bytes: {}", frame_data.width, frame_data.height, frame_data.bytes.len());
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}
```

> More examples can be found in the [`examples/`](./kinect-v2/examples/) directory.

## Building 🛠️

```powershell
# Clone the repo
$ git clone https://github.com/wangfu91/kinect-v2-rs.git
$ cd kinect-v2-rs

# Build the project
$ cargo build --release
```

## Project Structure 📁
- `kinect-v2/` — High-level, idiomatic Rust wrapper
- `kinect-v2-sys/` — Low-level FFI bindings to the Kinect V2 SDK

## Roadmap 🗺️
- [x] Implement color frame capture
- [x] Implement depth frame capture
- [x] Implement infrared frame capture
- [x] Implement body frame capture
- [x] Implement audio frame capture
- [x] Implement multi-source frame capture
- [x] Publish to crates.io
- [ ] Add more examples and documentation
- [ ] Improve error handling and safety
- [ ] Add tests for all features
- [ ] Implement face detection features
- [ ] Implement gesture recognition features
- [ ] Cross-platform support using the [libfreenect2](https://github.com/OpenKinect/libfreenect2) library

## License 📄

MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgements 🙏
- [Microsoft Kinect for Windows SDK 2.0](https://www.microsoft.com/en-us/download/details.aspx?id=44561)
- [windows-rs](https://github.com/microsoft/windows-rs)
- [bindgen](https://github.com/rust-lang/rust-bindgen)

---

Feel free to open issues or PRs! Happy hacking! 🚀
