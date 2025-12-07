# lib60870-sys

[![CI](https://github.com/tomas789/lib60870-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/tomas789/lib60870-sys/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/lib60870-sys.svg)](https://crates.io/crates/lib60870-sys)
[![Documentation](https://docs.rs/lib60870-sys/badge.svg)](https://docs.rs/lib60870-sys)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Raw FFI bindings to [lib60870-C](https://github.com/mz-automation/lib60870), an IEC 60870-5-101/104 protocol implementation.

This is a `-sys` crate that provides low-level, unsafe bindings to the C library. For a safe, idiomatic Rust API, consider using the [lib60870](https://crates.io/crates/lib60870) crate.

## Features

- **Raw FFI bindings** - Direct access to all lib60870-C functions

## Platform Support

| Platform | Status                                       |
| -------- | -------------------------------------------- |
| Linux    | ✅ Fully supported                            |
| macOS    | ✅ Fully supported                            |
| Windows  | ⚠️ Experimental (build works, runtime issues) |

> **Note:** Windows support is experimental. The library compiles successfully but may have runtime issues related to DLL dependencies. Contributions to improve Windows support are welcome!

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lib60870-sys = "0.4"

# With TLS support:
lib60870-sys = { version = "0.4", features = ["tls"] }
```

## Example

```rust
use lib60870_sys::*;

fn main() {
    // Get library version
    let version = unsafe { Lib60870_getLibraryVersionInfo() };
    println!(
        "lib60870 v{}.{}.{}",
        version.major, version.minor, version.patch
    );

    // Create a connection
    let ip = "127.0.0.1";
    let port = 2404;
    let conn = unsafe { CS104_Connection_create(ip.as_ptr() as *const std::os::raw::c_char, port) };

    if !conn.is_null() {
        println!("Connection created successfully");

        // Clean up
        unsafe { CS104_Connection_destroy(conn) };
        println!("Connection destroyed");
    } else {
        println!("Failed to create connection");
    }
}
```

Run the example:

```bash
cargo run --example version
```

> **Safety:** All functions in this crate are `unsafe` because they directly call C code. Users must ensure proper memory management, null pointer handling, and correct C string formatting.

## Cargo Features

| Feature         | Description                                 |
| --------------- | ------------------------------------------- |
| `tls`           | Enable TLS support (downloads mbedtls 2.28) |
| `debug`         | Enable printf debug output                  |
| `no-threads`    | Disable threading (for embedded systems)    |
| `tcp-keepalive` | Enable TCP keep-alive                       |

## How the Build Works

The build script (`build.rs`) automatically:

1. **Downloads lib60870 v2.3.6** from GitHub releases
2. **Downloads mbedtls v2.28.9** (only if `tls` feature is enabled)
3. **Compiles with CMake** - builds the static library
4. **Generates Rust bindings** with bindgen

All downloads are cached in `target/` so subsequent builds are fast.

## Pre-generated Bindings

For docs.rs and environments without network access, this crate includes pre-generated bindings in `src/bindings_pregenerated.rs`. These are used automatically when building with the `docsrs` cfg flag.

## Building from Source

Requirements:
- Rust 1.70 or later
- CMake 3.10 or later
- C compiler (GCC, Clang, or MSVC)
- Internet connection (for first build only)

```bash
cargo build
cargo build --features tls
```

## License

lib60870-C is dual-licensed under **GPLv3** and a commercial license. This crate inherits the GPL-3.0 license. See the [lib60870 repository](https://github.com/mz-automation/lib60870) for commercial licensing options.

## Links

- [lib60870-C Documentation](https://support.mz-automation.de/doc/lib60870/)
- [IEC 60870-5-104 Protocol](https://en.wikipedia.org/wiki/IEC_60870-5)
- [Repository](https://github.com/tomas789/lib60870-sys)
