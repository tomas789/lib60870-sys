//! Raw FFI bindings to lib60870-C.
//!
//! This module contains the auto-generated bindgen bindings.
//! For a safer API, use the types in the parent module.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unpredictable_function_pointer_comparisons)]

// On docs.rs, use pre-generated bindings (no network access to download C source)
#[cfg(docsrs)]
include!("bindings_pregenerated.rs");

// For normal builds, use freshly generated bindings from build.rs
#[cfg(not(docsrs))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
