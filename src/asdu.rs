//! Application Service Data Unit (ASDU) for IEC 60870-5.
//!
//! An ASDU contains one or more information objects of the same type.

use crate::sys;
use crate::types::{CauseOfTransmission, TypeId};

/// A borrowed reference to an ASDU.
///
/// This type is used when receiving ASDUs in callbacks. The ASDU is owned
/// by the library and should not be stored beyond the callback scope.
#[repr(transparent)]
pub struct AsduRef(pub(crate) sys::CS101_ASDU);

impl AsduRef {
    /// Create from a raw pointer (for use in callbacks).
    ///
    /// # Safety
    /// The pointer must be valid for the lifetime of the returned reference.
    pub(crate) unsafe fn from_ptr<'a>(ptr: sys::CS101_ASDU) -> &'a Self {
        &*(ptr as *const sys::CS101_ASDU as *const AsduRef)
    }

    /// Get the raw pointer.
    pub fn as_ptr(&self) -> sys::CS101_ASDU {
        self.0
    }

    /// Get the type identification of this ASDU.
    pub fn type_id(&self) -> Option<TypeId> {
        let raw = unsafe { sys::CS101_ASDU_getTypeID(self.0) };
        TypeId::from_raw(raw)
    }

    /// Get the raw type identification value.
    pub fn type_id_raw(&self) -> u32 {
        unsafe { sys::CS101_ASDU_getTypeID(self.0) }
    }

    /// Get the cause of transmission.
    pub fn cot(&self) -> Option<CauseOfTransmission> {
        let raw = unsafe { sys::CS101_ASDU_getCOT(self.0) };
        CauseOfTransmission::from_raw(raw)
    }

    /// Get the raw cause of transmission value.
    pub fn cot_raw(&self) -> u32 {
        unsafe { sys::CS101_ASDU_getCOT(self.0) }
    }

    /// Get the common address (station address).
    pub fn common_address(&self) -> u16 {
        unsafe { sys::CS101_ASDU_getCA(self.0) as u16 }
    }

    /// Get the originator address.
    pub fn originator_address(&self) -> u8 {
        unsafe { sys::CS101_ASDU_getOA(self.0) as u8 }
    }

    /// Get the number of information objects in this ASDU.
    pub fn element_count(&self) -> usize {
        unsafe { sys::CS101_ASDU_getNumberOfElements(self.0) as usize }
    }

    /// Check if this is a test ASDU.
    pub fn is_test(&self) -> bool {
        unsafe { sys::CS101_ASDU_isTest(self.0) }
    }

    /// Check if this is a negative confirmation.
    pub fn is_negative(&self) -> bool {
        unsafe { sys::CS101_ASDU_isNegative(self.0) }
    }

    /// Check if this ASDU uses a sequence (compact) format.
    ///
    /// In sequence format, only the first IOA is transmitted and subsequent
    /// objects have consecutive addresses.
    pub fn is_sequence(&self) -> bool {
        unsafe { sys::CS101_ASDU_isSequence(self.0) }
    }

    /// Get an information object element by index.
    ///
    /// Returns the raw pointer to the information object which must be
    /// cast to the appropriate type based on `type_id()`.
    ///
    /// # Safety
    /// The returned pointer is only valid while the ASDU is valid.
    /// The caller must destroy the object when done using `InformationObject_destroy`.
    pub unsafe fn get_element_raw(&self, index: usize) -> sys::InformationObject {
        sys::CS101_ASDU_getElement(self.0, index as i32)
    }
}

impl std::fmt::Debug for AsduRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Asdu")
            .field("type_id", &self.type_id())
            .field("cot", &self.cot())
            .field("ca", &self.common_address())
            .field("elements", &self.element_count())
            .finish()
    }
}

