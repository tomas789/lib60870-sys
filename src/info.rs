//! Information object types for IEC 60870-5.
//!
//! Information objects carry the actual data values in ASDUs.

use crate::sys;
use crate::types::Quality;

/// Information object address (IOA).
pub type Ioa = u32;

// ============================================================================
// Single Point Information (M_SP_NA_1)
// ============================================================================

/// Single-point information (boolean status).
///
/// Used for status indications like switch positions, alarm states, etc.
#[derive(Debug, Clone, Copy)]
pub struct SinglePoint {
    /// Information object address
    pub ioa: Ioa,
    /// Boolean value (true = ON, false = OFF)
    pub value: bool,
    /// Quality descriptor
    pub quality: Quality,
}

impl SinglePoint {
    /// Parse from a raw information object pointer.
    ///
    /// # Safety
    /// The pointer must be a valid SinglePointInformation object.
    pub unsafe fn from_raw(io: sys::InformationObject) -> Option<Self> {
        if io.is_null() {
            return None;
        }
        let spi = io as sys::SinglePointInformation;
        let result = Self {
            ioa: sys::InformationObject_getObjectAddress(io) as Ioa,
            value: sys::SinglePointInformation_getValue(spi),
            quality: Quality::from_bits_truncate(sys::SinglePointInformation_getQuality(spi) as u8),
        };
        sys::SinglePointInformation_destroy(spi);
        Some(result)
    }
}

// ============================================================================
// Double Point Information (M_DP_NA_1)
// ============================================================================

/// Double-point state values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DoublePointValue {
    /// Indeterminate or intermediate state
    Intermediate = 0,
    /// Determined state OFF
    Off = 1,
    /// Determined state ON
    On = 2,
    /// Indeterminate state
    Indeterminate = 3,
}

impl DoublePointValue {
    /// Create from raw value.
    pub fn from_raw(raw: sys::DoublePointValue) -> Self {
        match raw {
            sys::DoublePointValue_IEC60870_DOUBLE_POINT_INTERMEDIATE => Self::Intermediate,
            sys::DoublePointValue_IEC60870_DOUBLE_POINT_OFF => Self::Off,
            sys::DoublePointValue_IEC60870_DOUBLE_POINT_ON => Self::On,
            _ => Self::Indeterminate,
        }
    }
}

/// Double-point information (two-bit status).
///
/// Used for equipment with distinct ON/OFF states where intermediate
/// positions need to be detected.
#[derive(Debug, Clone, Copy)]
pub struct DoublePoint {
    /// Information object address
    pub ioa: Ioa,
    /// Double-point value
    pub value: DoublePointValue,
    /// Quality descriptor
    pub quality: Quality,
}

impl DoublePoint {
    /// Parse from a raw information object pointer.
    ///
    /// # Safety
    /// The pointer must be a valid DoublePointInformation object.
    pub unsafe fn from_raw(io: sys::InformationObject) -> Option<Self> {
        if io.is_null() {
            return None;
        }
        let dpi = io as sys::DoublePointInformation;
        let result = Self {
            ioa: sys::InformationObject_getObjectAddress(io) as Ioa,
            value: DoublePointValue::from_raw(sys::DoublePointInformation_getValue(dpi)),
            quality: Quality::from_bits_truncate(sys::DoublePointInformation_getQuality(dpi) as u8),
        };
        sys::DoublePointInformation_destroy(dpi);
        Some(result)
    }
}

// ============================================================================
// Measured Value, Scaled (M_ME_NB_1)
// ============================================================================

/// Measured value with scaled representation.
///
/// The value is a 16-bit signed integer (-32768 to 32767).
#[derive(Debug, Clone, Copy)]
pub struct MeasuredScaled {
    /// Information object address
    pub ioa: Ioa,
    /// Scaled value (-32768 to 32767)
    pub value: i16,
    /// Quality descriptor
    pub quality: Quality,
}

impl MeasuredScaled {
    /// Parse from a raw information object pointer.
    ///
    /// # Safety
    /// The pointer must be a valid MeasuredValueScaled object.
    pub unsafe fn from_raw(io: sys::InformationObject) -> Option<Self> {
        if io.is_null() {
            return None;
        }
        let mvs = io as sys::MeasuredValueScaled;
        let result = Self {
            ioa: sys::InformationObject_getObjectAddress(io) as Ioa,
            value: sys::MeasuredValueScaled_getValue(mvs) as i16,
            quality: Quality::from_bits_truncate(sys::MeasuredValueScaled_getQuality(mvs) as u8),
        };
        sys::MeasuredValueScaled_destroy(mvs);
        Some(result)
    }
}

// ============================================================================
// Measured Value, Normalized (M_ME_NA_1)
// ============================================================================

/// Measured value with normalized representation.
///
/// The value is normalized to the range -1.0 to ~1.0.
#[derive(Debug, Clone, Copy)]
pub struct MeasuredNormalized {
    /// Information object address
    pub ioa: Ioa,
    /// Normalized value (-1.0 to ~1.0)
    pub value: f32,
    /// Quality descriptor
    pub quality: Quality,
}

impl MeasuredNormalized {
    /// Parse from a raw information object pointer.
    ///
    /// # Safety
    /// The pointer must be a valid MeasuredValueNormalized object.
    pub unsafe fn from_raw(io: sys::InformationObject) -> Option<Self> {
        if io.is_null() {
            return None;
        }
        let mvn = io as sys::MeasuredValueNormalized;
        let result = Self {
            ioa: sys::InformationObject_getObjectAddress(io) as Ioa,
            value: sys::MeasuredValueNormalized_getValue(mvn),
            quality: Quality::from_bits_truncate(sys::MeasuredValueNormalized_getQuality(mvn) as u8),
        };
        sys::MeasuredValueNormalized_destroy(mvn);
        Some(result)
    }
}

// ============================================================================
// Measured Value, Short Float (M_ME_NC_1)
// ============================================================================

/// Measured value with short floating point representation.
#[derive(Debug, Clone, Copy)]
pub struct MeasuredFloat {
    /// Information object address
    pub ioa: Ioa,
    /// Floating point value
    pub value: f32,
    /// Quality descriptor
    pub quality: Quality,
}

impl MeasuredFloat {
    /// Parse from a raw information object pointer.
    ///
    /// # Safety
    /// The pointer must be a valid MeasuredValueShort object.
    pub unsafe fn from_raw(io: sys::InformationObject) -> Option<Self> {
        if io.is_null() {
            return None;
        }
        let mvf = io as sys::MeasuredValueShort;
        let result = Self {
            ioa: sys::InformationObject_getObjectAddress(io) as Ioa,
            value: sys::MeasuredValueShort_getValue(mvf),
            quality: Quality::from_bits_truncate(sys::MeasuredValueShort_getQuality(mvf) as u8),
        };
        sys::MeasuredValueShort_destroy(mvf);
        Some(result)
    }
}

// ============================================================================
// Single Command (C_SC_NA_1)
// ============================================================================

/// Single command (switch ON/OFF).
#[derive(Debug, Clone, Copy)]
pub struct SingleCommand {
    /// Information object address
    pub ioa: Ioa,
    /// Command state (true = ON, false = OFF)
    pub state: bool,
    /// Select/Execute flag (true = select, false = execute)
    pub select: bool,
    /// Qualifier of command (0 = no additional definition)
    pub qualifier: u8,
}

impl SingleCommand {
    /// Parse from a raw information object pointer.
    ///
    /// # Safety
    /// The pointer must be a valid SingleCommand object.
    pub unsafe fn from_raw(io: sys::InformationObject) -> Option<Self> {
        if io.is_null() {
            return None;
        }
        let sc = io as sys::SingleCommand;
        let result = Self {
            ioa: sys::InformationObject_getObjectAddress(io) as Ioa,
            state: sys::SingleCommand_getState(sc),
            select: sys::SingleCommand_isSelect(sc),
            qualifier: sys::SingleCommand_getQU(sc) as u8,
        };
        sys::SingleCommand_destroy(sc);
        Some(result)
    }
}

// ============================================================================
// Helper to parse information objects from ASDU
// ============================================================================

use crate::asdu::Asdu;
use crate::types::TypeId;

/// Parsed information object from an ASDU.
#[derive(Debug, Clone)]
pub enum InfoObject {
    /// Single-point information
    SinglePoint(SinglePoint),
    /// Double-point information
    DoublePoint(DoublePoint),
    /// Measured value, scaled
    MeasuredScaled(MeasuredScaled),
    /// Measured value, normalized
    MeasuredNormalized(MeasuredNormalized),
    /// Measured value, floating point
    MeasuredFloat(MeasuredFloat),
    /// Single command
    SingleCommand(SingleCommand),
    /// Unknown or unsupported type
    Unknown {
        /// Raw type ID value from the ASDU
        type_id: sys::IEC60870_5_TypeID,
        /// Information object address
        ioa: Ioa,
    },
}

impl Asdu {
    /// Parse all information objects from this ASDU.
    ///
    /// Returns a vector of parsed information objects. Unknown types are
    /// returned as `InfoObject::Unknown`.
    pub fn parse_objects(&self) -> Vec<InfoObject> {
        let count = self.element_count();
        let type_id = self.type_id();
        let mut objects = Vec::with_capacity(count);

        for i in 0..count {
            let io = unsafe { self.get_element_raw(i) };
            if io.is_null() {
                continue;
            }

            let obj = match type_id {
                Some(TypeId::SinglePoint) | Some(TypeId::SinglePointTime) => {
                    unsafe { SinglePoint::from_raw(io) }.map(InfoObject::SinglePoint)
                }
                Some(TypeId::DoublePoint) | Some(TypeId::DoublePointTime) => {
                    unsafe { DoublePoint::from_raw(io) }.map(InfoObject::DoublePoint)
                }
                Some(TypeId::MeasuredScaled) | Some(TypeId::MeasuredScaledTime) => {
                    unsafe { MeasuredScaled::from_raw(io) }.map(InfoObject::MeasuredScaled)
                }
                Some(TypeId::MeasuredNormalized) => {
                    unsafe { MeasuredNormalized::from_raw(io) }.map(InfoObject::MeasuredNormalized)
                }
                Some(TypeId::MeasuredFloat) | Some(TypeId::MeasuredFloatTime) => {
                    unsafe { MeasuredFloat::from_raw(io) }.map(InfoObject::MeasuredFloat)
                }
                Some(TypeId::SingleCommand) | Some(TypeId::SingleCommandTime) => {
                    unsafe { SingleCommand::from_raw(io) }.map(InfoObject::SingleCommand)
                }
                _ => {
                    let ioa = unsafe { sys::InformationObject_getObjectAddress(io) as Ioa };
                    unsafe { sys::InformationObject_destroy(io) };
                    Some(InfoObject::Unknown {
                        type_id: self.type_id_raw(),
                        ioa,
                    })
                }
            };

            if let Some(obj) = obj {
                objects.push(obj);
            }
        }

        objects
    }
}
