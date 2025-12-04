//! Safe Rust bindings to lib60870-C, an IEC 60870-5-101/104 protocol implementation.
//!
//! This crate provides safe wrappers around the lib60870 C library for implementing
//! IEC 60870-5-104 (TCP/IP) communication.
//!
//! ## Quick Start
//!
//! ```no_run
//! use lib60870::{Connection, CauseOfTransmission};
//!
//! // Create a connection to a server
//! let conn = Connection::new("127.0.0.1", 2404).expect("Failed to create connection");
//!
//! // Connect to the server
//! if conn.connect() {
//!     println!("Connected!");
//!
//!     // Send interrogation command
//!     conn.send_interrogation_command(CauseOfTransmission::Activation, 1, 20);
//! }
//! // Connection is automatically closed and destroyed when dropped
//! ```
//!
//! ## Features
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

use std::ffi::CString;
use std::ptr::NonNull;

use foreign_types::{ForeignType, ForeignTypeRef};

/// Raw FFI bindings - use these only if you need low-level access.
pub mod sys;

// Re-export useful constants
pub use sys::IEC_60870_5_104_DEFAULT_PORT;

/// Cause of transmission values for IEC 60870-5-101/104.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CauseOfTransmission {
    /// Periodic, cyclic transmission
    Periodic = sys::CS101_CauseOfTransmission_CS101_COT_PERIODIC,
    /// Background scan
    Background = sys::CS101_CauseOfTransmission_CS101_COT_BACKGROUND_SCAN,
    /// Spontaneous transmission
    Spontaneous = sys::CS101_CauseOfTransmission_CS101_COT_SPONTANEOUS,
    /// Initialized
    Initialized = sys::CS101_CauseOfTransmission_CS101_COT_INITIALIZED,
    /// Interrogation request
    Request = sys::CS101_CauseOfTransmission_CS101_COT_REQUEST,
    /// Activation
    Activation = sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION,
    /// Activation confirmation
    ActivationCon = sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION_CON,
    /// Deactivation
    Deactivation = sys::CS101_CauseOfTransmission_CS101_COT_DEACTIVATION,
    /// Deactivation confirmation
    DeactivationCon = sys::CS101_CauseOfTransmission_CS101_COT_DEACTIVATION_CON,
    /// Activation termination
    ActivationTermination = sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION_TERMINATION,
    /// Interrogated by station interrogation
    InterrogatedByStation = sys::CS101_CauseOfTransmission_CS101_COT_INTERROGATED_BY_STATION,
}

impl CauseOfTransmission {
    /// Convert to the raw C enum value.
    pub fn as_raw(self) -> sys::CS101_CauseOfTransmission {
        self as sys::CS101_CauseOfTransmission
    }
}

// ============================================================================
// CS104 Connection (Client)
// ============================================================================

/// Reference to an IEC 60870-5-104 connection (borrowed).
pub struct ConnectionRef(std::marker::PhantomData<sys::sCS104_Connection>);

impl ConnectionRef {
    /// Get the raw pointer.
    #[allow(dead_code)]
    fn as_ptr(&self) -> *mut sys::sCS104_Connection {
        self as *const _ as *mut _
    }
}

unsafe impl ForeignTypeRef for ConnectionRef {
    type CType = sys::sCS104_Connection;
}

/// An IEC 60870-5-104 client connection.
///
/// This type represents an owned connection to an IEC 104 server (slave).
/// When dropped, the connection is automatically closed and destroyed.
///
/// # Example
///
/// ```no_run
/// use lib60870::Connection;
///
/// let conn = Connection::new("192.168.1.100", 2404).unwrap();
/// if conn.connect() {
///     println!("Connected to server");
/// }
/// ```
pub struct Connection(NonNull<sys::sCS104_Connection>);

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

unsafe impl ForeignType for Connection {
    type CType = sys::sCS104_Connection;
    type Ref = ConnectionRef;

    unsafe fn from_ptr(ptr: *mut Self::CType) -> Self {
        Self(NonNull::new_unchecked(ptr))
    }

    fn as_ptr(&self) -> *mut Self::CType {
        self.0.as_ptr()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            sys::CS104_Connection_destroy(self.as_ptr());
        }
    }
}

impl Connection {
    /// Create a new connection to an IEC 104 server.
    ///
    /// # Arguments
    ///
    /// * `hostname` - The hostname or IP address of the server
    /// * `port` - The TCP port (use 2404 for standard IEC 104, or -1 for default)
    ///
    /// # Returns
    ///
    /// Returns `Some(Connection)` on success, `None` if the connection could not be created.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use lib60870::Connection;
    ///
    /// // Connect to default port
    /// let conn = Connection::new("192.168.1.100", 2404);
    ///
    /// // Use default port constant
    /// let conn = Connection::new("192.168.1.100", lib60870::IEC_60870_5_104_DEFAULT_PORT as i32);
    /// ```
    pub fn new(hostname: &str, port: i32) -> Option<Self> {
        let c_hostname = CString::new(hostname).ok()?;
        let ptr = unsafe { sys::CS104_Connection_create(c_hostname.as_ptr(), port) };
        NonNull::new(ptr).map(Self)
    }

    /// Connect to the server (blocking).
    ///
    /// This will establish a TCP connection and perform the IEC 104 STARTDT handshake.
    ///
    /// # Returns
    ///
    /// Returns `true` if connection was successful, `false` otherwise.
    pub fn connect(&self) -> bool {
        unsafe { sys::CS104_Connection_connect(self.as_ptr()) }
    }

    /// Start an asynchronous connection attempt.
    ///
    /// The connection will be established in the background. Use callbacks to
    /// be notified of the connection state.
    pub fn connect_async(&self) {
        unsafe { sys::CS104_Connection_connectAsync(self.as_ptr()) }
    }

    /// Close the connection.
    ///
    /// This sends a STOPDT message and closes the TCP connection.
    /// The connection object can be reused for a new connection.
    pub fn close(&self) {
        unsafe { sys::CS104_Connection_close(self.as_ptr()) }
    }

    /// Send a general interrogation command.
    ///
    /// # Arguments
    ///
    /// * `cot` - Cause of transmission (usually `CauseOfTransmission::Activation`)
    /// * `ca` - Common address of the ASDU (station address)
    /// * `qoi` - Qualifier of interrogation (20 = station interrogation)
    ///
    /// # Returns
    ///
    /// Returns `true` if the command was sent successfully.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use lib60870::{Connection, CauseOfTransmission};
    ///
    /// let conn = Connection::new("127.0.0.1", 2404).unwrap();
    /// conn.connect();
    ///
    /// // Send station interrogation
    /// conn.send_interrogation_command(CauseOfTransmission::Activation, 1, 20);
    /// ```
    pub fn send_interrogation_command(&self, cot: CauseOfTransmission, ca: i32, qoi: u8) -> bool {
        unsafe {
            sys::CS104_Connection_sendInterrogationCommand(
                self.as_ptr(),
                cot.as_raw(),
                ca,
                qoi.into(),
            )
        }
    }

    /// Send a counter interrogation command.
    ///
    /// # Arguments
    ///
    /// * `cot` - Cause of transmission
    /// * `ca` - Common address of the ASDU
    /// * `qcc` - Qualifier of counter interrogation command
    ///
    /// # Returns
    ///
    /// Returns `true` if the command was sent successfully.
    pub fn send_counter_interrogation_command(&self, cot: CauseOfTransmission, ca: i32, qcc: u8) -> bool {
        unsafe {
            sys::CS104_Connection_sendCounterInterrogationCommand(
                self.as_ptr(),
                cot.as_raw(),
                ca,
                qcc.into(),
            )
        }
    }

    /// Send a clock synchronization command.
    ///
    /// # Arguments
    ///
    /// * `ca` - Common address of the ASDU
    /// * `timestamp` - The timestamp to synchronize to (milliseconds since epoch)
    ///
    /// # Returns
    ///
    /// Returns `true` if the command was sent successfully.
    pub fn send_clock_sync_command(&self, ca: i32, timestamp: u64) -> bool {
        unsafe {
            let mut time = std::mem::zeroed::<sys::sCP56Time2a>();
            sys::CP56Time2a_setFromMsTimestamp(&mut time, timestamp);
            sys::CS104_Connection_sendClockSyncCommand(self.as_ptr(), ca, &mut time)
        }
    }

    /// Send a test command.
    ///
    /// # Arguments
    ///
    /// * `ca` - Common address of the ASDU
    ///
    /// # Returns
    ///
    /// Returns `true` if the command was sent successfully.
    pub fn send_test_command(&self, ca: i32) -> bool {
        unsafe { sys::CS104_Connection_sendTestCommand(self.as_ptr(), ca) }
    }

    /// Check if the transmit buffer is full.
    ///
    /// When the buffer is full, no more messages can be sent until
    /// some are acknowledged by the server.
    pub fn is_transmit_buffer_full(&self) -> bool {
        unsafe { sys::CS104_Connection_isTransmitBufferFull(self.as_ptr()) }
    }
}

// ============================================================================
// Library information
// ============================================================================

/// Get the lib60870 library version.
///
/// # Returns
///
/// A tuple of (major, minor, patch) version numbers.
///
/// # Example
///
/// ```
/// let (major, minor, patch) = lib60870::library_version();
/// println!("lib60870 v{}.{}.{}", major, minor, patch);
/// ```
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
        
        // lib60870 v2.3.x
        assert_eq!(major, 2);
        assert_eq!(minor, 3);
        assert!(patch >= 0);
        
        println!("lib60870 version: {}.{}.{}", major, minor, patch);
    }

    #[test]
    fn test_connection_create_and_drop() {
        // Test that we can create a connection and it gets properly destroyed
        let conn = Connection::new("127.0.0.1", 2404);
        assert!(conn.is_some(), "Should be able to create a connection object");
        
        // Connection is dropped here - verify no crash
        drop(conn);
    }

    #[test]
    fn test_connection_invalid_hostname() {
        // Empty hostname should still create a connection object
        // (the error will happen on connect)
        let conn = Connection::new("", 2404);
        assert!(conn.is_some());
    }

    #[test]
    fn test_cause_of_transmission_values() {
        assert_eq!(
            CauseOfTransmission::Activation.as_raw(),
            sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION
        );
        assert_eq!(
            CauseOfTransmission::Spontaneous.as_raw(),
            sys::CS101_CauseOfTransmission_CS101_COT_SPONTANEOUS
        );
    }

    #[test]
    fn test_sys_module_accessible() {
        // Verify we can access raw bindings through sys module
        let cot_str = unsafe {
            let ptr = sys::CS101_CauseOfTransmission_toString(
                sys::CS101_CauseOfTransmission_CS101_COT_SPONTANEOUS,
            );
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        };
        assert_eq!(cot_str, "SPONTANEOUS");
    }
}
