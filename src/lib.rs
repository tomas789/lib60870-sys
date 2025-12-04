//! FFI bindings to lib60870-C, an IEC 60870-5-101/104 protocol implementation.
//!
//! This crate provides raw FFI bindings to the lib60870 C library.
//! For higher-level Rust abstractions, consider wrapping this crate.
//!
//! ## Features
//!
//! - `tls` - Enable TLS support (downloads and links mbedtls 2.28)
//! - `debug` - Enable printf debug output in lib60870
//! - `no-threads` - Disable threading support (for embedded systems)
//! - `tcp-keepalive` - Enable TCP keep-alive
//!
//! ## TLS Support
//!
//! When the `tls` feature is enabled, the build script will automatically download
//! mbedtls 2.28 and compile it with lib60870. This enables secure connections via
//! `CS104_Connection_createSecure()` and related TLS functions.
//!
//! ## License
//!
//! lib60870 is dual-licensed under GPLv3 and a commercial license.
//! See <https://github.com/mz-automation/lib60870> for details.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unpredictable_function_pointer_comparisons)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_version() {
        let version = unsafe { Lib60870_getLibraryVersionInfo() };
        
        // lib60870 v2.3.x
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 3);
        assert!(version.patch >= 0);
        
        println!(
            "lib60870 version: {}.{}.{}",
            version.major, version.minor, version.patch
        );
    }

    #[test]
    fn test_cause_of_transmission_to_string() {
        // Test that we can call a function that returns a string
        let cot_str = unsafe {
            let ptr = CS101_CauseOfTransmission_toString(CS101_CauseOfTransmission_CS101_COT_SPONTANEOUS);
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        };
        
        assert_eq!(cot_str, "SPONTANEOUS");
        println!("COT_SPONTANEOUS = \"{}\"", cot_str);
    }
}

