//! IEC 60870-5-104 server (slave) implementation.
//!
//! The server accepts connections from clients (masters) and can send
//! spontaneous data and respond to commands.

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::asdu::Asdu;
use crate::sys;
use crate::time::Timestamp;
use crate::types::{CauseOfTransmission, PeerConnectionEvent, Quality, ServerMode};

/// Callback for connection requests.
///
/// Return `true` to accept the connection, `false` to reject.
pub type ConnectionRequestHandler = Box<dyn Fn(&str) -> bool + Send + Sync>;

/// Callback for connection events.
pub type ConnectionEventHandler = Box<dyn Fn(PeerConnectionEvent) + Send + Sync>;

/// Callback for interrogation commands.
///
/// The ASDU is cloned before being passed to the callback.
/// Return `true` if handled, `false` otherwise.
pub type InterrogationHandler = Box<dyn Fn(&MasterConnection, Asdu, u8) -> bool + Send + Sync>;

/// Callback for clock synchronization commands.
///
/// The ASDU is cloned before being passed to the callback.
/// Return `true` if handled, `false` otherwise.
pub type ClockSyncHandler = Box<dyn Fn(&MasterConnection, Asdu, &Timestamp) -> bool + Send + Sync>;

/// Callback for received ASDUs (commands).
///
/// The ASDU is cloned before being passed to the callback.
/// Return `true` if handled, `false` otherwise.
pub type AsduHandler = Box<dyn Fn(&MasterConnection, Asdu) -> bool + Send + Sync>;

/// A connection from a master (client) to this server.
///
/// This type represents a borrowed reference to an active client connection.
/// It is passed to server callbacks and can be used to send responses.
///
/// # Lifetime
///
/// **Important:** This type is only valid within the callback scope. The underlying
/// connection is owned by the C library and may be closed at any time after the
/// callback returns.
///
/// - ✅ Safe: Use within callbacks to send responses
/// - ❌ Unsafe: Store and use after callback returns
///
/// The type intentionally has no `Clone` implementation to prevent accidental storage.
#[repr(transparent)]
pub struct MasterConnection(sys::IMasterConnection);

impl MasterConnection {
    /// Create from raw pointer (for callbacks).
    pub(crate) unsafe fn from_ptr(ptr: sys::IMasterConnection) -> Self {
        Self(ptr)
    }

    /// Get the raw pointer for FFI interop.
    ///
    /// # Safety
    ///
    /// The returned pointer is only valid within the current callback scope.
    /// Do not store or use after the callback returns, as the connection
    /// may be closed by the C library at any time.
    pub fn as_ptr(&self) -> sys::IMasterConnection {
        self.0
    }

    /// Send an ASDU to this connection.
    pub fn send_asdu(&self, asdu: &Asdu) {
        unsafe {
            sys::IMasterConnection_sendASDU(self.0, asdu.as_ptr());
        }
    }

    /// Send an activation confirmation (ACT_CON).
    ///
    /// `negative` should be `true` for negative confirmation.
    pub fn send_act_con(&self, asdu: &Asdu, negative: bool) {
        unsafe {
            sys::IMasterConnection_sendACT_CON(self.0, asdu.as_ptr(), negative);
        }
    }

    /// Send an activation termination (ACT_TERM).
    pub fn send_act_term(&self, asdu: &Asdu) {
        unsafe {
            sys::IMasterConnection_sendACT_TERM(self.0, asdu.as_ptr());
        }
    }

    /// Get the application layer parameters for this connection.
    #[allow(dead_code)]
    pub(crate) fn app_layer_params(&self) -> sys::CS101_AppLayerParameters {
        unsafe { sys::IMasterConnection_getApplicationLayerParameters(self.0) }
    }
}

/// Internal state for callbacks.
struct CallbackState {
    connection_request_handler: Option<ConnectionRequestHandler>,
    connection_event_handler: Option<ConnectionEventHandler>,
    interrogation_handler: Option<InterrogationHandler>,
    clock_sync_handler: Option<ClockSyncHandler>,
    asdu_handler: Option<AsduHandler>,
}

/// Builder for creating a server.
pub struct ServerBuilder {
    max_low_priority_queue: i32,
    max_high_priority_queue: i32,
    local_address: String,
    local_port: i32,
    server_mode: ServerMode,
}

impl ServerBuilder {
    /// Create a new server builder with default settings.
    pub fn new() -> Self {
        Self {
            max_low_priority_queue: 10,
            max_high_priority_queue: 10,
            local_address: "0.0.0.0".to_string(),
            local_port: 2404,
            server_mode: ServerMode::SingleRedundancyGroup,
        }
    }

    /// Set the maximum queue sizes.
    pub fn queue_sizes(mut self, low_priority: i32, high_priority: i32) -> Self {
        self.max_low_priority_queue = low_priority;
        self.max_high_priority_queue = high_priority;
        self
    }

    /// Set the local address to bind to.
    pub fn local_address(mut self, address: &str) -> Self {
        self.local_address = address.to_string();
        self
    }

    /// Set the local port to listen on.
    pub fn local_port(mut self, port: u16) -> Self {
        self.local_port = port as i32;
        self
    }

    /// Set the server mode.
    pub fn server_mode(mut self, mode: ServerMode) -> Self {
        self.server_mode = mode;
        self
    }

    /// Build the server.
    pub fn build(self) -> Option<Server> {
        Server::new_with_config(
            self.max_low_priority_queue,
            self.max_high_priority_queue,
            &self.local_address,
            self.local_port,
            self.server_mode,
        )
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// An IEC 60870-5-104 server (slave).
///
/// # Example
///
/// ```no_run
/// use lib60870::server::{Server, ServerBuilder};
/// use lib60870::types::PeerConnectionEvent;
///
/// let mut server = ServerBuilder::new()
///     .local_address("0.0.0.0")
///     .local_port(2404)
///     .build()
///     .expect("Failed to create server");
///
/// server.set_connection_request_handler(|ip| {
///     println!("Connection request from {}", ip);
///     true // Accept all connections
/// });
///
/// server.set_connection_event_handler(|event| {
///     println!("Connection event: {:?}", event);
/// });
///
/// server.start();
///
/// // Server is running, enqueue data to send...
/// ```
pub struct Server {
    ptr: NonNull<sys::sCS104_Slave>,
    callback_state: Option<Arc<CallbackState>>,
}

unsafe impl Send for Server {}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            sys::CS104_Slave_stop(self.ptr.as_ptr());
            sys::CS104_Slave_destroy(self.ptr.as_ptr());
        }
    }
}

impl Server {
    /// Create a new server with default settings.
    pub fn new() -> Option<Self> {
        ServerBuilder::new().build()
    }

    fn new_with_config(
        max_low_priority_queue: i32,
        max_high_priority_queue: i32,
        local_address: &str,
        local_port: i32,
        server_mode: ServerMode,
    ) -> Option<Self> {
        let ptr =
            unsafe { sys::CS104_Slave_create(max_low_priority_queue, max_high_priority_queue) };
        let ptr = NonNull::new(ptr)?;

        let c_address = CString::new(local_address).ok()?;
        unsafe {
            sys::CS104_Slave_setLocalAddress(ptr.as_ptr(), c_address.as_ptr());
            sys::CS104_Slave_setLocalPort(ptr.as_ptr(), local_port);
            sys::CS104_Slave_setServerMode(ptr.as_ptr(), server_mode.as_raw());
        }

        Some(Self {
            ptr,
            callback_state: None,
        })
    }

    /// Get the raw pointer (for advanced use).
    pub fn as_ptr(&self) -> sys::CS104_Slave {
        self.ptr.as_ptr()
    }

    /// Get the application layer parameters.
    pub(crate) fn app_layer_params(&self) -> sys::CS101_AppLayerParameters {
        unsafe { sys::CS104_Slave_getAppLayerParameters(self.ptr.as_ptr()) }
    }

    /// Set the connection request handler.
    pub fn set_connection_request_handler<F>(&mut self, handler: F)
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.update_callback_state(|state| {
            state.connection_request_handler = Some(Box::new(handler));
        });
    }

    /// Set the connection event handler.
    pub fn set_connection_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(PeerConnectionEvent) + Send + Sync + 'static,
    {
        self.update_callback_state(|state| {
            state.connection_event_handler = Some(Box::new(handler));
        });
    }

    /// Set the interrogation handler.
    pub fn set_interrogation_handler<F>(&mut self, handler: F)
    where
        F: Fn(&MasterConnection, Asdu, u8) -> bool + Send + Sync + 'static,
    {
        self.update_callback_state(|state| {
            state.interrogation_handler = Some(Box::new(handler));
        });
    }

    /// Set the clock synchronization handler.
    pub fn set_clock_sync_handler<F>(&mut self, handler: F)
    where
        F: Fn(&MasterConnection, Asdu, &Timestamp) -> bool + Send + Sync + 'static,
    {
        self.update_callback_state(|state| {
            state.clock_sync_handler = Some(Box::new(handler));
        });
    }

    /// Set the ASDU handler for commands.
    pub fn set_asdu_handler<F>(&mut self, handler: F)
    where
        F: Fn(&MasterConnection, Asdu) -> bool + Send + Sync + 'static,
    {
        self.update_callback_state(|state| {
            state.asdu_handler = Some(Box::new(handler));
        });
    }

    fn update_callback_state<F>(&mut self, f: F)
    where
        F: FnOnce(&mut CallbackState),
    {
        // Create new state with existing handlers
        let mut new_state = CallbackState {
            connection_request_handler: None,
            connection_event_handler: None,
            interrogation_handler: None,
            clock_sync_handler: None,
            asdu_handler: None,
        };
        f(&mut new_state);

        let state = Arc::new(new_state);
        let state_ptr = Arc::as_ptr(&state) as *mut c_void;
        self.callback_state = Some(state);

        unsafe {
            sys::CS104_Slave_setConnectionRequestHandler(
                self.ptr.as_ptr(),
                Some(connection_request_trampoline),
                state_ptr,
            );
            sys::CS104_Slave_setConnectionEventHandler(
                self.ptr.as_ptr(),
                Some(connection_event_trampoline),
                state_ptr,
            );
            sys::CS104_Slave_setInterrogationHandler(
                self.ptr.as_ptr(),
                Some(interrogation_trampoline),
                state_ptr,
            );
            sys::CS104_Slave_setClockSyncHandler(
                self.ptr.as_ptr(),
                Some(clock_sync_trampoline),
                state_ptr,
            );
            sys::CS104_Slave_setASDUHandler(
                self.ptr.as_ptr(),
                Some(asdu_handler_trampoline),
                state_ptr,
            );
        }
    }

    /// Start the server.
    pub fn start(&self) {
        unsafe {
            sys::CS104_Slave_start(self.ptr.as_ptr());
        }
    }

    /// Stop the server.
    pub fn stop(&self) {
        unsafe {
            sys::CS104_Slave_stop(self.ptr.as_ptr());
        }
    }

    /// Check if the server is running.
    pub fn is_running(&self) -> bool {
        unsafe { sys::CS104_Slave_isRunning(self.ptr.as_ptr()) }
    }

    /// Enqueue an ASDU to be sent to all connected clients.
    ///
    /// This is used to send spontaneous data updates.
    pub fn enqueue_asdu(&self, asdu: &Asdu) {
        unsafe {
            sys::CS104_Slave_enqueueASDU(self.ptr.as_ptr(), asdu.as_ptr());
        }
    }

    /// Send a single-point information value.
    ///
    /// This is a convenience method that creates and enqueues an ASDU.
    pub fn send_single_point(
        &self,
        cot: CauseOfTransmission,
        ca: u16,
        ioa: u32,
        value: bool,
        quality: Quality,
    ) {
        unsafe {
            let al_params = self.app_layer_params();
            let asdu =
                sys::CS101_ASDU_create(al_params, false, cot.as_raw(), 0, ca as i32, false, false);
            if asdu.is_null() {
                return;
            }

            let io = sys::SinglePointInformation_create(
                std::ptr::null_mut(),
                ioa as i32,
                value,
                quality.bits(),
            );
            if !io.is_null() {
                sys::CS101_ASDU_addInformationObject(asdu, io as sys::InformationObject);
                sys::InformationObject_destroy(io as sys::InformationObject);
            }

            sys::CS104_Slave_enqueueASDU(self.ptr.as_ptr(), asdu);
            sys::CS101_ASDU_destroy(asdu);
        }
    }

    /// Send a measured scaled value.
    ///
    /// This is a convenience method that creates and enqueues an ASDU.
    pub fn send_measured_scaled(
        &self,
        cot: CauseOfTransmission,
        ca: u16,
        ioa: u32,
        value: i16,
        quality: Quality,
    ) {
        unsafe {
            let al_params = self.app_layer_params();
            let asdu =
                sys::CS101_ASDU_create(al_params, false, cot.as_raw(), 0, ca as i32, false, false);
            if asdu.is_null() {
                return;
            }

            let io = sys::MeasuredValueScaled_create(
                std::ptr::null_mut(),
                ioa as i32,
                value as i32,
                quality.bits(),
            );
            if !io.is_null() {
                sys::CS101_ASDU_addInformationObject(asdu, io as sys::InformationObject);
                sys::InformationObject_destroy(io as sys::InformationObject);
            }

            sys::CS104_Slave_enqueueASDU(self.ptr.as_ptr(), asdu);
            sys::CS101_ASDU_destroy(asdu);
        }
    }

    /// Send a measured float value.
    pub fn send_measured_float(
        &self,
        cot: CauseOfTransmission,
        ca: u16,
        ioa: u32,
        value: f32,
        quality: Quality,
    ) {
        unsafe {
            let al_params = self.app_layer_params();
            let asdu =
                sys::CS101_ASDU_create(al_params, false, cot.as_raw(), 0, ca as i32, false, false);
            if asdu.is_null() {
                return;
            }

            let io = sys::MeasuredValueShort_create(
                std::ptr::null_mut(),
                ioa as i32,
                value,
                quality.bits(),
            );
            if !io.is_null() {
                sys::CS101_ASDU_addInformationObject(asdu, io as sys::InformationObject);
                sys::InformationObject_destroy(io as sys::InformationObject);
            }

            sys::CS104_Slave_enqueueASDU(self.ptr.as_ptr(), asdu);
            sys::CS101_ASDU_destroy(asdu);
        }
    }
}

// C callback trampolines

unsafe extern "C" fn connection_request_trampoline(
    parameter: *mut c_void,
    ip_address: *const std::os::raw::c_char,
) -> bool {
    if parameter.is_null() || ip_address.is_null() {
        return true;
    }
    let state = &*(parameter as *const CallbackState);
    if let Some(ref handler) = state.connection_request_handler {
        let ip = std::ffi::CStr::from_ptr(ip_address).to_str().unwrap_or("");
        handler(ip)
    } else {
        true
    }
}

unsafe extern "C" fn connection_event_trampoline(
    parameter: *mut c_void,
    _connection: sys::IMasterConnection,
    event: sys::CS104_PeerConnectionEvent,
) {
    if parameter.is_null() {
        return;
    }
    let state = &*(parameter as *const CallbackState);
    if let Some(ref handler) = state.connection_event_handler {
        if let Some(event) = PeerConnectionEvent::from_raw(event) {
            handler(event);
        }
    }
}

unsafe extern "C" fn interrogation_trampoline(
    parameter: *mut c_void,
    connection: sys::IMasterConnection,
    asdu: sys::CS101_ASDU,
    qoi: sys::QualifierOfInterrogation,
) -> bool {
    if parameter.is_null() || connection.is_null() || asdu.is_null() {
        return false;
    }
    let state = &*(parameter as *const CallbackState);
    if let Some(ref handler) = state.interrogation_handler {
        let conn = MasterConnection::from_ptr(connection);
        if let Some(owned_asdu) = Asdu::clone_from_ptr(asdu) {
            handler(&conn, owned_asdu, qoi)
        } else {
            false
        }
    } else {
        false
    }
}

unsafe extern "C" fn clock_sync_trampoline(
    parameter: *mut c_void,
    connection: sys::IMasterConnection,
    asdu: sys::CS101_ASDU,
    new_time: sys::CP56Time2a,
) -> bool {
    if parameter.is_null() || connection.is_null() || asdu.is_null() || new_time.is_null() {
        return false;
    }
    let state = &*(parameter as *const CallbackState);
    if let Some(ref handler) = state.clock_sync_handler {
        let conn = MasterConnection::from_ptr(connection);
        if let Some(owned_asdu) = Asdu::clone_from_ptr(asdu) {
            let time = Timestamp(*new_time);
            handler(&conn, owned_asdu, &time)
        } else {
            false
        }
    } else {
        false
    }
}

unsafe extern "C" fn asdu_handler_trampoline(
    parameter: *mut c_void,
    connection: sys::IMasterConnection,
    asdu: sys::CS101_ASDU,
) -> bool {
    if parameter.is_null() || connection.is_null() || asdu.is_null() {
        return false;
    }
    let state = &*(parameter as *const CallbackState);
    if let Some(ref handler) = state.asdu_handler {
        let conn = MasterConnection::from_ptr(connection);
        if let Some(owned_asdu) = Asdu::clone_from_ptr(asdu) {
            handler(&conn, owned_asdu)
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
    fn test_server_builder() {
        let server = ServerBuilder::new()
            .local_address("127.0.0.1")
            .local_port(2404)
            .queue_sizes(10, 10)
            .build();
        assert!(server.is_some());
    }

    #[test]
    fn test_server_create() {
        let server = Server::new();
        assert!(server.is_some());
    }
}
