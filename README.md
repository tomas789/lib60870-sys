# lib60870

Safe Rust bindings to [lib60870-C](https://github.com/mz-automation/lib60870), an IEC 60870-5-101/104 protocol implementation.

## Features

- **Safe API** - Automatic memory management with owned types
- **Client (Master)** - Connect to IEC 104 servers and send commands
- **Server (Slave)** - Accept connections and send spontaneous data
- **Callbacks** - Rust closures for handling events and data

## Usage

```toml
[dependencies]
lib60870 = { git = "https://github.com/..." }

# With TLS support:
lib60870 = { git = "https://github.com/...", features = ["tls"] }
```

## Quick Start - Client

```rust
use lib60870::client::ConnectionBuilder;
use lib60870::types::{CauseOfTransmission, QOI_STATION};

fn main() {
    let mut conn = ConnectionBuilder::new("127.0.0.1", 2404)
        .originator_address(3)
        .build()
        .expect("Failed to create connection");

    // Set up handlers
    conn.set_handlers(
        |event| println!("Connection event: {:?}", event),
        |asdu| {
            println!("Received ASDU: {:?}", asdu);
            for obj in asdu.parse_objects() {
                println!("  {:?}", obj);
            }
            true
        },
    );

    // Connect and send interrogation
    if conn.connect() {
        println!("Connected!");
        conn.send_start_dt();
        conn.send_interrogation(CauseOfTransmission::Activation, 1, QOI_STATION);
        
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    // Connection automatically closed on drop
}
```

## Quick Start - Server

```rust
use lib60870::server::ServerBuilder;
use lib60870::types::{CauseOfTransmission, Quality};

fn main() {
    let mut server = ServerBuilder::new()
        .local_port(2404)
        .build()
        .expect("Failed to create server");

    server.set_connection_event_handler(|event| {
        println!("Connection: {:?}", event);
    });

    server.set_interrogation_handler(|conn, asdu, qoi| {
        println!("Interrogation for group {}", qoi);
        conn.send_act_con(&asdu, false);
        // Send response data here...
        conn.send_act_term(&asdu);
        true
    });

    server.start();
    println!("Server running on port 2404");

    // Send periodic data
    loop {
        server.send_measured_scaled(
            CauseOfTransmission::Periodic,
            1,    // Common address
            100,  // IOA
            42,   // Value
            Quality::GOOD,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```

## Low-Level Access

For advanced use cases, raw FFI bindings are available via the `sys` module:

```rust
use lib60870::sys;

let version = unsafe { sys::Lib60870_getLibraryVersionInfo() };
println!("lib60870 v{}.{}.{}", version.major, version.minor, version.patch);
```

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

## License

lib60870 is dual-licensed under **GPLv3** and a commercial license.
See [lib60870 repository](https://github.com/mz-automation/lib60870) for details.
