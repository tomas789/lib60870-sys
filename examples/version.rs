use lib60870_sys::*;
use std::ffi::CString;

fn main() {
    // Get library version
    let version = unsafe { Lib60870_getLibraryVersionInfo() };
    println!(
        "lib60870 v{}.{}.{}",
        version.major, version.minor, version.patch
    );

    // Create a connection
    let ip = CString::new("127.0.0.1").unwrap();
    let port = 2404;
    let conn = unsafe { CS104_Connection_create(ip.as_ptr(), port) };

    if !conn.is_null() {
        println!("Connection created successfully");

        // Clean up
        unsafe { CS104_Connection_destroy(conn) };
        println!("Connection destroyed");
    } else {
        println!("Failed to create connection");
    }
}
