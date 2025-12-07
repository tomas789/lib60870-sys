use lib60870_sys::*;

fn main() {
    // Get library version
    let version = unsafe { Lib60870_getLibraryVersionInfo() };
    println!(
        "lib60870 v{}.{}.{}",
        version.major, version.minor, version.patch
    );

    // Create a connection
    let ip = "127.0.0.1";
    let port = 2404;
    let conn = unsafe { CS104_Connection_create(ip.as_ptr() as *const std::os::raw::c_char, port) };

    if !conn.is_null() {
        println!("Connection created successfully");

        // Clean up
        unsafe { CS104_Connection_destroy(conn) };
        println!("Connection destroyed");
    } else {
        println!("Failed to create connection");
    }
}
