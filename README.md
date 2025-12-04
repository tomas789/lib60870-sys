# lib60870-sys

Rust FFI bindings to [lib60870-C](https://github.com/mz-automation/lib60870), an IEC 60870-5-101/104 protocol implementation.

## Usage

```toml
[dependencies]
lib60870-sys = { path = "../lib60870-sys" }

# With TLS support:
lib60870-sys = { path = "../lib60870-sys", features = ["tls"] }
```

## Example

```rust
use lib60870_sys::*;
use std::ffi::CString;

fn main() {
    // Print library version
    let version = unsafe { Lib60870_getLibraryVersionInfo() };
    println!("lib60870 v{}.{}.{}", version.major, version.minor, version.patch);

    // Create a CS104 connection
    let host = CString::new("127.0.0.1").unwrap();
    let connection = unsafe { CS104_Connection_create(host.as_ptr(), 2404) };

    if !connection.is_null() {
        // Try to connect (will fail if no server is running)
        let connected = unsafe { CS104_Connection_connect(connection) };
        println!("Connected: {}", connected);

        // Clean up
        unsafe { CS104_Connection_destroy(connection) };
    }
}
```

## Features

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

## License

lib60870 is dual-licensed under **GPLv3** and a commercial license.
See [lib60870 repository](https://github.com/mz-automation/lib60870) for details.

