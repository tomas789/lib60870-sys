//! Safe Rust bindings for IEC 60870-5-101/104 protocols.
//!
//! This crate provides safe wrappers around the [lib60870-C](https://github.com/mz-automation/lib60870)
//! library for implementing IEC 60870-5-104 (TCP/IP) communication.
//!
//! ## Features
//!
//! - **Client (Master)**: Connect to IEC 104 servers and send commands
//! - **Server (Slave)**: Accept connections and send spontaneous data
//! - **Safe API**: Automatic memory management, Rust callbacks
//! - **Common types**: Quality descriptors, cause of transmission, timestamps
//!
//! ## Quick Start - Client
//!
//! ```no_run
//! use lib60870::client::{Connection, ConnectionBuilder};
//! use lib60870::types::{CauseOfTransmission, QOI_STATION};
//!
//! let mut conn = ConnectionBuilder::new("127.0.0.1", 2404)
//!     .originator_address(3)
//!     .build()
//!     .expect("Failed to create connection");
//!
//! conn.set_handlers(
//!     |event| println!("Event: {:?}", event),
//!     |asdu| {
//!         println!("Received: {:?}", asdu);
//!         for obj in asdu.parse_objects() {
//!             println!("  {:?}", obj);
//!         }
//!         true
//!     },
//! );
//!
//! if conn.connect() {
//!     conn.send_start_dt();
//!     conn.send_interrogation(CauseOfTransmission::Activation, 1, QOI_STATION);
//!     std::thread::sleep(std::time::Duration::from_secs(5));
//! }
//! ```
//!
//! ## Quick Start - Server
//!
//! ```no_run
//! use lib60870::server::{Server, ServerBuilder};
//! use lib60870::types::{CauseOfTransmission, Quality};
//!
//! let mut server = ServerBuilder::new()
//!     .local_port(2404)
//!     .build()
//!     .expect("Failed to create server");
//!
//! server.set_connection_event_handler(|event| {
//!     println!("Connection event: {:?}", event);
//! });
//!
//! server.set_interrogation_handler(|conn, asdu, qoi| {
//!     println!("Interrogation for group {}", qoi);
//!     conn.send_act_con(asdu, false);
//!     // Send data...
//!     conn.send_act_term(asdu);
//!     true
//! });
//!
//! server.start();
//!
//! // Send spontaneous data
//! loop {
//!     server.send_measured_scaled(
//!         CauseOfTransmission::Spontaneous,
//!         1,    // Common address
//!         100,  // IOA
//!         42,   // Value
//!         Quality::GOOD,
//!     );
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//! }
//! ```
//!
//! ## Crate Features
//!
//! - `tls` - Enable TLS support (downloads and links mbedtls 2.28)
//! - `debug` - Enable printf debug output in lib60870
//! - `no-threads` - Disable threading support (for embedded systems)
//! - `tcp-keepalive` - Enable TCP keep-alive
//!
//! ## License
//!
//! lib60870 is dual-licensed under GPLv3 and a commercial license.
//! See <https://github.com/mz-automation/lib60870> for details.

// Modules
pub mod asdu;
pub mod client;
pub mod info;
pub mod server;
pub mod time;
pub mod types;

/// Raw FFI bindings - use only if you need low-level access.
pub mod sys;

// Convenience re-exports
pub use asdu::AsduRef;
pub use client::{Connection, ConnectionBuilder};
pub use info::{
    DoublePoint, DoublePointValue, InfoObject, Ioa, MeasuredFloat, MeasuredNormalized,
    MeasuredScaled, SingleCommand, SinglePoint,
};
pub use server::{MasterConnection, Server, ServerBuilder};
pub use time::Timestamp;
pub use types::{
    CauseOfTransmission, ConnectionEvent, PeerConnectionEvent, Quality, ServerMode, TypeId,
    DEFAULT_PORT, DEFAULT_TLS_PORT, QOI_STATION,
};

/// Get the lib60870 library version.
///
/// Returns a tuple of (major, minor, patch) version numbers.
pub fn library_version() -> (i32, i32, i32) {
    let version = unsafe { sys::Lib60870_getLibraryVersionInfo() };
    (version.major, version.minor, version.patch)
}

/// Enable or disable debug output.
///
/// When enabled, lib60870 will print debug messages to stdout.
/// This requires the crate to be built with the `debug` feature.
pub fn set_debug_output(enabled: bool) {
    unsafe { sys::Lib60870_enableDebugOutput(enabled) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_version() {
        let (major, minor, patch) = library_version();
        assert_eq!(major, 2);
        assert_eq!(minor, 3);
        assert!(patch >= 0);
        println!("lib60870 v{}.{}.{}", major, minor, patch);
    }

    #[test]
    fn test_type_id_names() {
        assert_eq!(TypeId::SinglePoint.name(), "M_SP_NA_1");
        assert_eq!(TypeId::MeasuredScaled.name(), "M_ME_NB_1");
        assert_eq!(TypeId::SingleCommand.name(), "C_SC_NA_1");
    }

    #[test]
    fn test_quality_flags() {
        let q = Quality::GOOD;
        assert!(q.is_empty());

        let q = Quality::INVALID | Quality::BLOCKED;
        assert!(q.contains(Quality::INVALID));
        assert!(q.contains(Quality::BLOCKED));
        assert!(!q.contains(Quality::OVERFLOW));
    }

    #[test]
    fn test_timestamp() {
        let ts = Timestamp::now();
        assert!(ts.year() >= 2024);
        println!("Timestamp: {}", ts);
    }

    #[test]
    fn test_client_creation() {
        let conn = Connection::new("127.0.0.1", 2404);
        assert!(conn.is_some());
    }

    #[test]
    fn test_server_creation() {
        let server = Server::new();
        assert!(server.is_some());
    }
}
