//! IEC 60870-5-104 client (master) implementation.
//!
//! The client connects to a server (slave/RTU) and can send commands
//! and receive data.

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::asdu::Asdu;
use crate::sys;
use crate::time::Timestamp;
use crate::types::{CauseOfTransmission, ConnectionEvent};

/// Callback for connection state changes.
pub type ConnectionHandler = Box<dyn Fn(ConnectionEvent) + Send + Sync>;

/// Callback for received ASDUs.
///
/// The ASDU is cloned before being passed to the callback, so it's safe
/// to store or send to another thread.
///
/// Return `true` to indicate the ASDU was handled, `false` otherwise.
pub type AsduHandler = Box<dyn Fn(Asdu) -> bool + Send + Sync>;

/// Builder for configuring a connection.
pub struct ConnectionBuilder {
    hostname: String,
    port: i32,
    originator_address: u8,
    connect_timeout_ms: u32,
}

impl ConnectionBuilder {
    /// Create a new connection builder.
    pub fn new(hostname: &str, port: u16) -> Self {
        Self {
            hostname: hostname.to_string(),
            port: port as i32,
            originator_address: 0,
            connect_timeout_ms: 10000,
        }
    }

    /// Set the originator address (0-255).
    pub fn originator_address(mut self, oa: u8) -> Self {
        self.originator_address = oa;
        self
    }

    /// Set the connection timeout in milliseconds.
    pub fn connect_timeout_ms(mut self, timeout: u32) -> Self {
        self.connect_timeout_ms = timeout;
        self
    }

    /// Build the connection.
    pub fn build(self) -> Option<Connection> {
        Connection::new_with_config(
            &self.hostname,
            self.port,
            self.originator_address,
            self.connect_timeout_ms,
        )
    }
}

/// Internal state for callbacks.
struct CallbackState {
    connection_handler: Option<ConnectionHandler>,
    asdu_handler: Option<AsduHandler>,
}

/// An IEC 60870-5-104 client connection.
///
/// This represents a connection to a server (slave/RTU). The connection
/// is automatically destroyed when dropped.
///
/// # Example
///
/// ```no_run
/// use lib60870::client::{Connection, ConnectionBuilder};
/// use lib60870::types::{CauseOfTransmission, ConnectionEvent, QOI_STATION};
///
/// let mut conn = ConnectionBuilder::new("127.0.0.1", 2404)
///     .originator_address(3)
///     .build()
///     .expect("Failed to create connection");
///
/// conn.set_connection_handler(|event| {
///     println!("Connection event: {:?}", event);
/// });
///
/// conn.set_asdu_handler(|asdu| {
///     println!("Received: {:?}", asdu);
///     for obj in asdu.parse_objects() {
///         println!("  {:?}", obj);
///     }
///     true // asdu is owned, can be stored if needed
/// });
///
/// if conn.connect() {
///     conn.send_start_dt();
///     conn.send_interrogation(CauseOfTransmission::Activation, 1, QOI_STATION);
/// }
/// ```
pub struct Connection {
    ptr: NonNull<sys::sCS104_Connection>,
    // Must be pinned because C callbacks hold a pointer to it
    callback_state: Option<Arc<CallbackState>>,
}

unsafe impl Send for Connection {}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            sys::CS104_Connection_destroy(self.ptr.as_ptr());
        }
    }
}

impl Connection {
    /// Create a new connection with default settings.
    pub fn new(hostname: &str, port: u16) -> Option<Self> {
        Self::new_with_config(hostname, port as i32, 0, 10000)
    }

    fn new_with_config(hostname: &str, port: i32, originator_address: u8, timeout_ms: u32) -> Option<Self> {
        let c_hostname = CString::new(hostname).ok()?;
        let ptr = unsafe { sys::CS104_Connection_create(c_hostname.as_ptr(), port) };
        let ptr = NonNull::new(ptr)?;

        // Configure originator address
        unsafe {
            let al_params = sys::CS104_Connection_getAppLayerParameters(ptr.as_ptr());
            if !al_params.is_null() {
                (*al_params).originatorAddress = originator_address as i32;
            }
            sys::CS104_Connection_setConnectTimeout(ptr.as_ptr(), timeout_ms as i32);
        }

        Some(Self {
            ptr,
            callback_state: None,
        })
    }

    /// Get the raw pointer (for advanced use).
    pub fn as_ptr(&self) -> sys::CS104_Connection {
        self.ptr.as_ptr()
    }

    /// Set the connection event handler.
    ///
    /// Note: Setting handlers individually may reset previously set handlers.
    /// Use `set_handlers()` to set both at once.
    pub fn set_connection_handler<F>(&mut self, handler: F)
    where
        F: Fn(ConnectionEvent) + Send + Sync + 'static,
    {
        let state = Arc::new(CallbackState {
            connection_handler: Some(Box::new(handler)),
            asdu_handler: None,
        });
        
        let state_ptr = Arc::as_ptr(&state) as *mut c_void;
        self.callback_state = Some(state);

        unsafe {
            sys::CS104_Connection_setConnectionHandler(
                self.ptr.as_ptr(),
                Some(connection_handler_trampoline),
                state_ptr,
            );
        }
    }

    /// Set the ASDU received handler.
    ///
    /// The ASDU passed to the callback is an owned clone, safe to store.
    pub fn set_asdu_handler<F>(&mut self, handler: F)
    where
        F: Fn(Asdu) -> bool + Send + Sync + 'static,
    {
        let state = Arc::new(CallbackState {
            connection_handler: None,
            asdu_handler: Some(Box::new(handler)),
        });
        
        let state_ptr = Arc::as_ptr(&state) as *mut c_void;
        self.callback_state = Some(state);

        unsafe {
            sys::CS104_Connection_setASDUReceivedHandler(
                self.ptr.as_ptr(),
                Some(asdu_handler_trampoline),
                state_ptr,
            );
        }
    }

    /// Set both connection and ASDU handlers at once.
    ///
    /// This is the recommended way to set handlers as it avoids issues
    /// with handler state management.
    pub fn set_handlers<C, A>(&mut self, connection_handler: C, asdu_handler: A)
    where
        C: Fn(ConnectionEvent) + Send + Sync + 'static,
        A: Fn(Asdu) -> bool + Send + Sync + 'static,
    {
        let state = Arc::new(CallbackState {
            connection_handler: Some(Box::new(connection_handler)),
            asdu_handler: Some(Box::new(asdu_handler)),
        });
        
        let state_ptr = Arc::as_ptr(&state) as *mut c_void;
        self.callback_state = Some(state);

        unsafe {
            sys::CS104_Connection_setConnectionHandler(
                self.ptr.as_ptr(),
                Some(connection_handler_trampoline),
                state_ptr,
            );
            sys::CS104_Connection_setASDUReceivedHandler(
                self.ptr.as_ptr(),
                Some(asdu_handler_trampoline),
                state_ptr,
            );
        }
    }

    /// Connect to the server (blocking).
    ///
    /// Returns `true` if the connection was successful.
    pub fn connect(&self) -> bool {
        unsafe { sys::CS104_Connection_connect(self.ptr.as_ptr()) }
    }

    /// Start an asynchronous connection.
    ///
    /// Use the connection handler to be notified when the connection
    /// is established or fails.
    pub fn connect_async(&self) {
        unsafe { sys::CS104_Connection_connectAsync(self.ptr.as_ptr()) }
    }

    /// Send STARTDT (start data transfer) activation.
    ///
    /// This must be called after connecting to enable data transfer.
    pub fn send_start_dt(&self) {
        unsafe { sys::CS104_Connection_sendStartDT(self.ptr.as_ptr()) }
    }

    /// Send STOPDT (stop data transfer) activation.
    pub fn send_stop_dt(&self) {
        unsafe { sys::CS104_Connection_sendStopDT(self.ptr.as_ptr()) }
    }

    /// Close the connection.
    pub fn close(&self) {
        unsafe { sys::CS104_Connection_close(self.ptr.as_ptr()) }
    }

    /// Send a general interrogation command.
    ///
    /// # Arguments
    /// * `cot` - Cause of transmission (usually `Activation`)
    /// * `ca` - Common address (station address)
    /// * `qoi` - Qualifier of interrogation (use `QOI_STATION` for station interrogation)
    pub fn send_interrogation(&self, cot: CauseOfTransmission, ca: u16, qoi: u8) -> bool {
        unsafe {
            sys::CS104_Connection_sendInterrogationCommand(
                self.ptr.as_ptr(),
                cot.as_raw(),
                ca as i32,
                qoi.into(),
            )
        }
    }

    /// Send a counter interrogation command.
    pub fn send_counter_interrogation(&self, cot: CauseOfTransmission, ca: u16, qcc: u8) -> bool {
        unsafe {
            sys::CS104_Connection_sendCounterInterrogationCommand(
                self.ptr.as_ptr(),
                cot.as_raw(),
                ca as i32,
                qcc.into(),
            )
        }
    }

    /// Send a clock synchronization command.
    pub fn send_clock_sync(&self, ca: u16, time: &Timestamp) -> bool {
        unsafe {
            sys::CS104_Connection_sendClockSyncCommand(
                self.ptr.as_ptr(),
                ca as i32,
                time.as_raw() as *const _ as *mut _,
            )
        }
    }

    /// Send a test command.
    pub fn send_test_command(&self, ca: u16) -> bool {
        unsafe { sys::CS104_Connection_sendTestCommand(self.ptr.as_ptr(), ca as i32) }
    }

    /// Send a test command with timestamp.
    pub fn send_test_command_with_time(&self, ca: u16, tsc: u16, time: &Timestamp) -> bool {
        unsafe {
            sys::CS104_Connection_sendTestCommandWithTimestamp(
                self.ptr.as_ptr(),
                ca as i32,
                tsc,
                time.as_raw() as *const _ as *mut _,
            )
        }
    }

    /// Send a single command (C_SC_NA_1).
    ///
    /// # Arguments
    /// * `cot` - Cause of transmission
    /// * `ca` - Common address
    /// * `ioa` - Information object address
    /// * `state` - Command state (true = ON, false = OFF)
    /// * `select` - Select/Execute (true = select, false = execute)
    /// * `qualifier` - Qualifier (0 = no additional definition)
    pub fn send_single_command(
        &self,
        cot: CauseOfTransmission,
        ca: u16,
        ioa: u32,
        state: bool,
        select: bool,
        qualifier: u8,
    ) -> bool {
        unsafe {
            let sc = sys::SingleCommand_create(
                std::ptr::null_mut(),
                ioa as i32,
                state,
                select,
                qualifier as i32,
            );
            if sc.is_null() {
                return false;
            }
            let result = sys::CS104_Connection_sendProcessCommandEx(
                self.ptr.as_ptr(),
                cot.as_raw(),
                ca as i32,
                sc as sys::InformationObject,
            );
            sys::SingleCommand_destroy(sc);
            result
        }
    }

    /// Check if the transmit buffer is full.
    pub fn is_transmit_buffer_full(&self) -> bool {
        unsafe { sys::CS104_Connection_isTransmitBufferFull(self.ptr.as_ptr()) }
    }
}

// C callback trampolines

unsafe extern "C" fn connection_handler_trampoline(
    parameter: *mut c_void,
    _connection: sys::CS104_Connection,
    event: sys::CS104_ConnectionEvent,
) {
    if parameter.is_null() {
        return;
    }
    let state = &*(parameter as *const CallbackState);
    if let Some(ref handler) = state.connection_handler {
        if let Some(event) = ConnectionEvent::from_raw(event) {
            handler(event);
        }
    }
}

unsafe extern "C" fn asdu_handler_trampoline(
    parameter: *mut c_void,
    _address: i32,
    asdu: sys::CS101_ASDU,
) -> bool {
    if parameter.is_null() || asdu.is_null() {
        return false;
    }
    let state = &*(parameter as *const CallbackState);
    if let Some(ref handler) = state.asdu_handler {
        // Clone the ASDU so the callback gets an owned copy
        if let Some(owned_asdu) = Asdu::clone_from_ptr(asdu) {
            handler(owned_asdu)
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_builder() {
        let conn = ConnectionBuilder::new("127.0.0.1", 2404)
            .originator_address(3)
            .connect_timeout_ms(5000)
            .build();
        assert!(conn.is_some());
    }

    #[test]
    fn test_connection_create() {
        let conn = Connection::new("127.0.0.1", 2404);
        assert!(conn.is_some());
    }
}

