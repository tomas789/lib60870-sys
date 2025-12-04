//! Application Service Data Unit (ASDU) for IEC 60870-5.
//!
//! An ASDU contains one or more information objects of the same type.

use crate::sys;
use crate::types::{CauseOfTransmission, TypeId};

/// An owned ASDU (Application Service Data Unit).
///
/// This type stores a complete copy of the ASDU data in an inline buffer (296 bytes),
/// making it safe to store and pass around freely. The data is copied from the
/// original ASDU using `CS101_ASDU_clone`.
///
/// # Example
///
/// ```ignore
/// // In a callback, clone the ASDU to keep it
/// let owned_asdu = Asdu::clone_from_ptr(raw_ptr);
/// // Now safe to store or send to another thread
/// ```
pub struct Asdu {
    /// Static buffer containing the ASDU data (296 bytes inline).
    /// This field holds the data that `ptr` points into.
    #[allow(dead_code)]
    buffer: sys::sCS101_StaticASDU,
    /// Pointer into the buffer (set by CS101_ASDU_clone)
    ptr: sys::CS101_ASDU,
}

// Safety: The buffer is fully owned and self-contained
unsafe impl Send for Asdu {}
unsafe impl Sync for Asdu {}

impl Clone for Asdu {
    fn clone(&self) -> Self {
        // Clone into a new buffer
        let mut buffer = sys::sCS101_StaticASDU::default();
        let ptr = unsafe { sys::CS101_ASDU_clone(self.ptr, &mut buffer) };
        Self { buffer, ptr }
    }
}

impl Asdu {
    /// Clone an ASDU from a raw pointer.
    ///
    /// This creates an owned copy of the ASDU data in an inline buffer.
    ///
    /// # Safety
    /// The pointer must be a valid `CS101_ASDU`.
    pub unsafe fn clone_from_ptr(ptr: sys::CS101_ASDU) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        let mut buffer = sys::sCS101_StaticASDU::default();
        let cloned_ptr = sys::CS101_ASDU_clone(ptr, &mut buffer);
        if cloned_ptr.is_null() {
            return None;
        }
        Some(Self {
            buffer,
            ptr: cloned_ptr,
        })
    }

    /// Get the raw pointer.
    ///
    /// The pointer is valid for the lifetime of this `Asdu`.
    pub fn as_ptr(&self) -> sys::CS101_ASDU {
        self.ptr
    }

    /// Get the type identification of this ASDU.
    pub fn type_id(&self) -> Option<TypeId> {
        let raw = unsafe { sys::CS101_ASDU_getTypeID(self.ptr) };
        TypeId::from_raw(raw)
    }

    /// Get the raw type identification value.
    pub fn type_id_raw(&self) -> sys::IEC60870_5_TypeID {
        unsafe { sys::CS101_ASDU_getTypeID(self.ptr) }
    }

    /// Get the cause of transmission.
    pub fn cot(&self) -> Option<CauseOfTransmission> {
        let raw = unsafe { sys::CS101_ASDU_getCOT(self.ptr) };
        CauseOfTransmission::from_raw(raw)
    }

    /// Get the raw cause of transmission value.
    pub fn cot_raw(&self) -> sys::CS101_CauseOfTransmission {
        unsafe { sys::CS101_ASDU_getCOT(self.ptr) }
    }

    /// Get the common address (station address).
    pub fn common_address(&self) -> u16 {
        unsafe { sys::CS101_ASDU_getCA(self.ptr) as u16 }
    }

    /// Get the originator address.
    pub fn originator_address(&self) -> u8 {
        unsafe { sys::CS101_ASDU_getOA(self.ptr) as u8 }
    }

    /// Get the number of information objects in this ASDU.
    pub fn element_count(&self) -> usize {
        unsafe { sys::CS101_ASDU_getNumberOfElements(self.ptr) as usize }
    }

    /// Check if this is a test ASDU.
    pub fn is_test(&self) -> bool {
        unsafe { sys::CS101_ASDU_isTest(self.ptr) }
    }

    /// Check if this is a negative confirmation.
    pub fn is_negative(&self) -> bool {
        unsafe { sys::CS101_ASDU_isNegative(self.ptr) }
    }

    /// Check if this ASDU uses a sequence (compact) format.
    ///
    /// In sequence format, only the first IOA is transmitted and subsequent
    /// objects have consecutive addresses.
    pub fn is_sequence(&self) -> bool {
        unsafe { sys::CS101_ASDU_isSequence(self.ptr) }
    }

    /// Get an information object element by index.
    ///
    /// Returns the raw pointer to the information object which must be
    /// cast to the appropriate type based on `type_id()`.
    ///
    /// # Safety
    /// The returned pointer is only valid while this ASDU is valid.
    /// The caller must destroy the object when done using `InformationObject_destroy`.
    pub unsafe fn get_element_raw(&self, index: usize) -> sys::InformationObject {
        sys::CS101_ASDU_getElement(self.ptr, index as i32)
    }
}

impl std::fmt::Debug for Asdu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Asdu")
            .field("type_id", &self.type_id())
            .field("cot", &self.cot())
            .field("ca", &self.common_address())
            .field("elements", &self.element_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asdu_size() {
        // Verify the static ASDU buffer size
        assert_eq!(
            std::mem::size_of::<sys::sCS101_StaticASDU>(),
            296,
            "Static ASDU buffer should be 296 bytes"
        );
    }
}
