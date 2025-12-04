//! Common types and enums for IEC 60870-5-101/104.

use crate::sys;

/// Default TCP port for IEC 60870-5-104.
pub const DEFAULT_PORT: u16 = sys::IEC_60870_5_104_DEFAULT_PORT as u16;

/// Default TLS port for IEC 60870-5-104.
pub const DEFAULT_TLS_PORT: u16 = sys::IEC_60870_5_104_DEFAULT_TLS_PORT as u16;

/// Qualifier of interrogation - station interrogation.
pub const QOI_STATION: u8 = sys::IEC60870_QOI_STATION as u8;

// ============================================================================
// Cause of Transmission
// ============================================================================

/// Cause of transmission for ASDUs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CauseOfTransmission {
    /// Periodic, cyclic transmission
    Periodic,
    /// Background scan
    Background,
    /// Spontaneous transmission
    Spontaneous,
    /// Initialized
    Initialized,
    /// Interrogation request
    Request,
    /// Activation
    Activation,
    /// Activation confirmation
    ActivationCon,
    /// Deactivation
    Deactivation,
    /// Deactivation confirmation
    DeactivationCon,
    /// Activation termination
    ActivationTermination,
    /// Return information caused by remote command
    ReturnRemote,
    /// Return information caused by local command
    ReturnLocal,
    /// Interrogated by station interrogation
    InterrogatedByStation,
    /// Unknown type identification
    UnknownType,
    /// Unknown cause of transmission
    UnknownCot,
    /// Unknown common address
    UnknownCa,
    /// Unknown information object address
    UnknownIoa,
}

impl CauseOfTransmission {
    /// Convert to raw C value.
    pub fn as_raw(self) -> sys::CS101_CauseOfTransmission {
        match self {
            Self::Periodic => sys::CS101_CauseOfTransmission_CS101_COT_PERIODIC,
            Self::Background => sys::CS101_CauseOfTransmission_CS101_COT_BACKGROUND_SCAN,
            Self::Spontaneous => sys::CS101_CauseOfTransmission_CS101_COT_SPONTANEOUS,
            Self::Initialized => sys::CS101_CauseOfTransmission_CS101_COT_INITIALIZED,
            Self::Request => sys::CS101_CauseOfTransmission_CS101_COT_REQUEST,
            Self::Activation => sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION,
            Self::ActivationCon => sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION_CON,
            Self::Deactivation => sys::CS101_CauseOfTransmission_CS101_COT_DEACTIVATION,
            Self::DeactivationCon => sys::CS101_CauseOfTransmission_CS101_COT_DEACTIVATION_CON,
            Self::ActivationTermination => {
                sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION_TERMINATION
            }
            Self::ReturnRemote => sys::CS101_CauseOfTransmission_CS101_COT_RETURN_INFO_REMOTE,
            Self::ReturnLocal => sys::CS101_CauseOfTransmission_CS101_COT_RETURN_INFO_LOCAL,
            Self::InterrogatedByStation => {
                sys::CS101_CauseOfTransmission_CS101_COT_INTERROGATED_BY_STATION
            }
            Self::UnknownType => sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_TYPE_ID,
            Self::UnknownCot => sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_COT,
            Self::UnknownCa => sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_CA,
            Self::UnknownIoa => sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_IOA,
        }
    }

    /// Try to create from raw C value.
    pub fn from_raw(raw: sys::CS101_CauseOfTransmission) -> Option<Self> {
        match raw {
            sys::CS101_CauseOfTransmission_CS101_COT_PERIODIC => Some(Self::Periodic),
            sys::CS101_CauseOfTransmission_CS101_COT_BACKGROUND_SCAN => Some(Self::Background),
            sys::CS101_CauseOfTransmission_CS101_COT_SPONTANEOUS => Some(Self::Spontaneous),
            sys::CS101_CauseOfTransmission_CS101_COT_INITIALIZED => Some(Self::Initialized),
            sys::CS101_CauseOfTransmission_CS101_COT_REQUEST => Some(Self::Request),
            sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION => Some(Self::Activation),
            sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION_CON => Some(Self::ActivationCon),
            sys::CS101_CauseOfTransmission_CS101_COT_DEACTIVATION => Some(Self::Deactivation),
            sys::CS101_CauseOfTransmission_CS101_COT_DEACTIVATION_CON => {
                Some(Self::DeactivationCon)
            }
            sys::CS101_CauseOfTransmission_CS101_COT_ACTIVATION_TERMINATION => {
                Some(Self::ActivationTermination)
            }
            sys::CS101_CauseOfTransmission_CS101_COT_RETURN_INFO_REMOTE => Some(Self::ReturnRemote),
            sys::CS101_CauseOfTransmission_CS101_COT_RETURN_INFO_LOCAL => Some(Self::ReturnLocal),
            sys::CS101_CauseOfTransmission_CS101_COT_INTERROGATED_BY_STATION => {
                Some(Self::InterrogatedByStation)
            }
            sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_TYPE_ID => Some(Self::UnknownType),
            sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_COT => Some(Self::UnknownCot),
            sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_CA => Some(Self::UnknownCa),
            sys::CS101_CauseOfTransmission_CS101_COT_UNKNOWN_IOA => Some(Self::UnknownIoa),
            _ => None,
        }
    }
}

// ============================================================================
// Type ID (ASDU type identification)
// ============================================================================

/// ASDU type identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeId {
    // === Monitor direction (M_*) ===
    /// Single-point information (M_SP_NA_1)
    SinglePoint,
    /// Single-point with time tag CP56Time2a (M_SP_TB_1)
    SinglePointTime,
    /// Double-point information (M_DP_NA_1)
    DoublePoint,
    /// Double-point with time tag CP56Time2a (M_DP_TB_1)
    DoublePointTime,
    /// Step position information (M_ST_NA_1)
    StepPosition,
    /// Bitstring of 32 bits (M_BO_NA_1)
    Bitstring32,
    /// Measured value, normalized (M_ME_NA_1)
    MeasuredNormalized,
    /// Measured value, scaled (M_ME_NB_1)
    MeasuredScaled,
    /// Measured value, scaled with time CP56Time2a (M_ME_TE_1)
    MeasuredScaledTime,
    /// Measured value, short floating point (M_ME_NC_1)
    MeasuredFloat,
    /// Measured value, short floating point with time CP56Time2a (M_ME_TF_1)
    MeasuredFloatTime,
    /// Integrated totals (M_IT_NA_1)
    IntegratedTotals,
    /// End of initialization (M_EI_NA_1)
    EndOfInit,

    // === Control direction (C_*) ===
    /// Single command (C_SC_NA_1)
    SingleCommand,
    /// Single command with time CP56Time2a (C_SC_TA_1)
    SingleCommandTime,
    /// Double command (C_DC_NA_1)
    DoubleCommand,
    /// Regulating step command (C_RC_NA_1)
    RegulatingStep,
    /// Set point command, normalized (C_SE_NA_1)
    SetpointNormalized,
    /// Set point command, scaled (C_SE_NB_1)
    SetpointScaled,
    /// Set point command, short floating point (C_SE_NC_1)
    SetpointFloat,

    // === System commands ===
    /// Interrogation command (C_IC_NA_1)
    Interrogation,
    /// Counter interrogation command (C_CI_NA_1)
    CounterInterrogation,
    /// Read command (C_RD_NA_1)
    Read,
    /// Clock synchronization (C_CS_NA_1)
    ClockSync,
    /// Test command (C_TS_NA_1)
    TestCommand,
    /// Test command with time CP56Time2a (C_TS_TA_1)
    TestCommandTime,
    /// Reset process command (C_RP_NA_1)
    ResetProcess,
}

impl TypeId {
    /// Convert to raw C value.
    pub fn as_raw(self) -> sys::IEC60870_5_TypeID {
        match self {
            Self::SinglePoint => sys::IEC60870_5_TypeID_M_SP_NA_1,
            Self::SinglePointTime => sys::IEC60870_5_TypeID_M_SP_TB_1,
            Self::DoublePoint => sys::IEC60870_5_TypeID_M_DP_NA_1,
            Self::DoublePointTime => sys::IEC60870_5_TypeID_M_DP_TB_1,
            Self::StepPosition => sys::IEC60870_5_TypeID_M_ST_NA_1,
            Self::Bitstring32 => sys::IEC60870_5_TypeID_M_BO_NA_1,
            Self::MeasuredNormalized => sys::IEC60870_5_TypeID_M_ME_NA_1,
            Self::MeasuredScaled => sys::IEC60870_5_TypeID_M_ME_NB_1,
            Self::MeasuredScaledTime => sys::IEC60870_5_TypeID_M_ME_TE_1,
            Self::MeasuredFloat => sys::IEC60870_5_TypeID_M_ME_NC_1,
            Self::MeasuredFloatTime => sys::IEC60870_5_TypeID_M_ME_TF_1,
            Self::IntegratedTotals => sys::IEC60870_5_TypeID_M_IT_NA_1,
            Self::EndOfInit => sys::IEC60870_5_TypeID_M_EI_NA_1,
            Self::SingleCommand => sys::IEC60870_5_TypeID_C_SC_NA_1,
            Self::SingleCommandTime => sys::IEC60870_5_TypeID_C_SC_TA_1,
            Self::DoubleCommand => sys::IEC60870_5_TypeID_C_DC_NA_1,
            Self::RegulatingStep => sys::IEC60870_5_TypeID_C_RC_NA_1,
            Self::SetpointNormalized => sys::IEC60870_5_TypeID_C_SE_NA_1,
            Self::SetpointScaled => sys::IEC60870_5_TypeID_C_SE_NB_1,
            Self::SetpointFloat => sys::IEC60870_5_TypeID_C_SE_NC_1,
            Self::Interrogation => sys::IEC60870_5_TypeID_C_IC_NA_1,
            Self::CounterInterrogation => sys::IEC60870_5_TypeID_C_CI_NA_1,
            Self::Read => sys::IEC60870_5_TypeID_C_RD_NA_1,
            Self::ClockSync => sys::IEC60870_5_TypeID_C_CS_NA_1,
            Self::TestCommand => sys::IEC60870_5_TypeID_C_TS_NA_1,
            Self::TestCommandTime => sys::IEC60870_5_TypeID_C_TS_TA_1,
            Self::ResetProcess => sys::IEC60870_5_TypeID_C_RP_NA_1,
        }
    }

    /// Try to create from raw C value.
    pub fn from_raw(raw: sys::IEC60870_5_TypeID) -> Option<Self> {
        match raw {
            sys::IEC60870_5_TypeID_M_SP_NA_1 => Some(Self::SinglePoint),
            sys::IEC60870_5_TypeID_M_SP_TB_1 => Some(Self::SinglePointTime),
            sys::IEC60870_5_TypeID_M_DP_NA_1 => Some(Self::DoublePoint),
            sys::IEC60870_5_TypeID_M_DP_TB_1 => Some(Self::DoublePointTime),
            sys::IEC60870_5_TypeID_M_ST_NA_1 => Some(Self::StepPosition),
            sys::IEC60870_5_TypeID_M_BO_NA_1 => Some(Self::Bitstring32),
            sys::IEC60870_5_TypeID_M_ME_NA_1 => Some(Self::MeasuredNormalized),
            sys::IEC60870_5_TypeID_M_ME_NB_1 => Some(Self::MeasuredScaled),
            sys::IEC60870_5_TypeID_M_ME_TE_1 => Some(Self::MeasuredScaledTime),
            sys::IEC60870_5_TypeID_M_ME_NC_1 => Some(Self::MeasuredFloat),
            sys::IEC60870_5_TypeID_M_ME_TF_1 => Some(Self::MeasuredFloatTime),
            sys::IEC60870_5_TypeID_M_IT_NA_1 => Some(Self::IntegratedTotals),
            sys::IEC60870_5_TypeID_M_EI_NA_1 => Some(Self::EndOfInit),
            sys::IEC60870_5_TypeID_C_SC_NA_1 => Some(Self::SingleCommand),
            sys::IEC60870_5_TypeID_C_SC_TA_1 => Some(Self::SingleCommandTime),
            sys::IEC60870_5_TypeID_C_DC_NA_1 => Some(Self::DoubleCommand),
            sys::IEC60870_5_TypeID_C_RC_NA_1 => Some(Self::RegulatingStep),
            sys::IEC60870_5_TypeID_C_SE_NA_1 => Some(Self::SetpointNormalized),
            sys::IEC60870_5_TypeID_C_SE_NB_1 => Some(Self::SetpointScaled),
            sys::IEC60870_5_TypeID_C_SE_NC_1 => Some(Self::SetpointFloat),
            sys::IEC60870_5_TypeID_C_IC_NA_1 => Some(Self::Interrogation),
            sys::IEC60870_5_TypeID_C_CI_NA_1 => Some(Self::CounterInterrogation),
            sys::IEC60870_5_TypeID_C_RD_NA_1 => Some(Self::Read),
            sys::IEC60870_5_TypeID_C_CS_NA_1 => Some(Self::ClockSync),
            sys::IEC60870_5_TypeID_C_TS_NA_1 => Some(Self::TestCommand),
            sys::IEC60870_5_TypeID_C_TS_TA_1 => Some(Self::TestCommandTime),
            sys::IEC60870_5_TypeID_C_RP_NA_1 => Some(Self::ResetProcess),
            _ => None,
        }
    }

    /// Get the name of this type ID.
    pub fn name(self) -> &'static str {
        let ptr = unsafe { sys::TypeID_toString(self.as_raw()) };
        if ptr.is_null() {
            "UNKNOWN"
        } else {
            unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("UNKNOWN") }
        }
    }
}

// ============================================================================
// Quality Descriptor
// ============================================================================

bitflags::bitflags! {
    /// Quality descriptor flags for information objects.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Quality: u8 {
        /// Good quality (no flags set)
        const GOOD = 0;
        /// Overflow flag
        const OVERFLOW = sys::IEC60870_QUALITY_OVERFLOW as u8;
        /// Reserved (bit 1)
        const RESERVED = sys::IEC60870_QUALITY_RESERVED as u8;
        /// Elapsed time invalid
        const ELAPSED_TIME_INVALID = sys::IEC60870_QUALITY_ELAPSED_TIME_INVALID as u8;
        /// Blocked
        const BLOCKED = sys::IEC60870_QUALITY_BLOCKED as u8;
        /// Substituted
        const SUBSTITUTED = sys::IEC60870_QUALITY_SUBSTITUTED as u8;
        /// Not topical
        const NOT_TOPICAL = sys::IEC60870_QUALITY_NON_TOPICAL as u8;
        /// Invalid
        const INVALID = sys::IEC60870_QUALITY_INVALID as u8;
    }
}

impl Default for Quality {
    fn default() -> Self {
        Quality::GOOD
    }
}

// ============================================================================
// Connection Events
// ============================================================================

/// Connection events for the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionEvent {
    /// TCP connection opened
    Opened,
    /// TCP connection closed
    Closed,
    /// Connection attempt failed
    Failed,
    /// Received STARTDT confirmation
    StartDtCon,
    /// Received STOPDT confirmation
    StopDtCon,
}

impl ConnectionEvent {
    /// Try to create from raw C value.
    pub fn from_raw(raw: sys::CS104_ConnectionEvent) -> Option<Self> {
        match raw {
            sys::CS104_ConnectionEvent_CS104_CONNECTION_OPENED => Some(Self::Opened),
            sys::CS104_ConnectionEvent_CS104_CONNECTION_CLOSED => Some(Self::Closed),
            sys::CS104_ConnectionEvent_CS104_CONNECTION_FAILED => Some(Self::Failed),
            sys::CS104_ConnectionEvent_CS104_CONNECTION_STARTDT_CON_RECEIVED => {
                Some(Self::StartDtCon)
            }
            sys::CS104_ConnectionEvent_CS104_CONNECTION_STOPDT_CON_RECEIVED => {
                Some(Self::StopDtCon)
            }
            _ => None,
        }
    }
}

/// Peer connection events for the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeerConnectionEvent {
    /// Connection opened
    Opened,
    /// Connection closed
    Closed,
    /// Connection activated (STARTDT received)
    Activated,
    /// Connection deactivated (STOPDT received)
    Deactivated,
}

impl PeerConnectionEvent {
    /// Try to create from raw C value.
    pub fn from_raw(raw: sys::CS104_PeerConnectionEvent) -> Option<Self> {
        match raw {
            sys::CS104_PeerConnectionEvent_CS104_CON_EVENT_CONNECTION_OPENED => Some(Self::Opened),
            sys::CS104_PeerConnectionEvent_CS104_CON_EVENT_CONNECTION_CLOSED => Some(Self::Closed),
            sys::CS104_PeerConnectionEvent_CS104_CON_EVENT_ACTIVATED => Some(Self::Activated),
            sys::CS104_PeerConnectionEvent_CS104_CON_EVENT_DEACTIVATED => Some(Self::Deactivated),
            _ => None,
        }
    }
}

/// Server mode for CS104 slave.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ServerMode {
    /// Single redundancy group (default)
    #[default]
    SingleRedundancyGroup,
    /// Connection is activated on request (multiple clients)
    ConnectionIsRedundancyGroup,
    /// Multiple redundancy groups
    MultipleRedundancyGroups,
}

impl ServerMode {
    /// Convert to raw C value.
    pub fn as_raw(self) -> sys::CS104_ServerMode {
        match self {
            Self::SingleRedundancyGroup => sys::CS104_ServerMode_CS104_MODE_SINGLE_REDUNDANCY_GROUP,
            Self::ConnectionIsRedundancyGroup => {
                sys::CS104_ServerMode_CS104_MODE_CONNECTION_IS_REDUNDANCY_GROUP
            }
            Self::MultipleRedundancyGroups => {
                sys::CS104_ServerMode_CS104_MODE_MULTIPLE_REDUNDANCY_GROUPS
            }
        }
    }
}
