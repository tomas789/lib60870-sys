//! Time types for IEC 60870-5-101/104.

use crate::sys;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A 7-byte time value (CP56Time2a) used in IEC 60870-5.
///
/// This represents a timestamp with millisecond precision.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Timestamp(pub(crate) sys::sCP56Time2a);

impl Timestamp {
    /// Create a new timestamp from a Unix timestamp in milliseconds.
    pub fn from_ms(timestamp_ms: u64) -> Self {
        let mut time = sys::sCP56Time2a::default();
        unsafe {
            sys::CP56Time2a_setFromMsTimestamp(&mut time, timestamp_ms);
        }
        Self(time)
    }

    /// Create a timestamp representing the current time.
    pub fn now() -> Self {
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() as u64;
        Self::from_ms(ms)
    }

    /// Convert to Unix timestamp in milliseconds.
    pub fn as_ms(&self) -> u64 {
        unsafe { sys::CP56Time2a_toMsTimestamp(&self.0 as *const _ as *mut _) }
    }

    /// Get the milliseconds part (0-999).
    pub fn millisecond(&self) -> u16 {
        unsafe { sys::CP56Time2a_getMillisecond(&self.0 as *const _ as *mut _) as u16 }
    }

    /// Get the seconds part (0-59).
    pub fn second(&self) -> u8 {
        unsafe { sys::CP56Time2a_getSecond(&self.0 as *const _ as *mut _) as u8 }
    }

    /// Get the minutes part (0-59).
    pub fn minute(&self) -> u8 {
        unsafe { sys::CP56Time2a_getMinute(&self.0 as *const _ as *mut _) as u8 }
    }

    /// Get the hour (0-23).
    pub fn hour(&self) -> u8 {
        unsafe { sys::CP56Time2a_getHour(&self.0 as *const _ as *mut _) as u8 }
    }

    /// Get the day of month (1-31).
    pub fn day(&self) -> u8 {
        unsafe { sys::CP56Time2a_getDayOfMonth(&self.0 as *const _ as *mut _) as u8 }
    }

    /// Get the month (1-12).
    pub fn month(&self) -> u8 {
        unsafe { sys::CP56Time2a_getMonth(&self.0 as *const _ as *mut _) as u8 }
    }

    /// Get the year (0-99, representing 2000-2099).
    pub fn year(&self) -> u16 {
        let raw = unsafe { sys::CP56Time2a_getYear(&self.0 as *const _ as *mut _) as u16 };
        2000 + raw
    }

    /// Check if the invalid flag is set.
    pub fn is_invalid(&self) -> bool {
        unsafe { sys::CP56Time2a_isInvalid(&self.0 as *const _ as *mut _) }
    }

    /// Check if the substituted flag is set.
    pub fn is_substituted(&self) -> bool {
        unsafe { sys::CP56Time2a_isSubstituted(&self.0 as *const _ as *mut _) }
    }

    /// Check if summer time is active.
    pub fn is_summer_time(&self) -> bool {
        unsafe { sys::CP56Time2a_isSummerTime(&self.0 as *const _ as *mut _) }
    }

    /// Get the raw C struct (for FFI interop).
    pub fn as_raw(&self) -> &sys::sCP56Time2a {
        &self.0
    }

    /// Get a mutable reference to the raw C struct (for FFI interop).
    pub fn as_raw_mut(&mut self) -> &mut sys::sCP56Time2a {
        &mut self.0
    }
}

impl std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            self.millisecond()
        )
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl From<u64> for Timestamp {
    fn from(ms: u64) -> Self {
        Self::from_ms(ms)
    }
}

impl From<Timestamp> for u64 {
    fn from(ts: Timestamp) -> u64 {
        ts.as_ms()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_roundtrip() {
        let original_ms: u64 = 1701705600000; // 2023-12-04 16:00:00 UTC
        let ts = Timestamp::from_ms(original_ms);
        let recovered = ts.as_ms();
        // Allow small rounding differences due to year representation
        assert!((original_ms as i64 - recovered as i64).abs() < 1000);
    }

    #[test]
    fn test_timestamp_now() {
        let ts = Timestamp::now();
        assert!(ts.year() >= 2024);
    }
}
