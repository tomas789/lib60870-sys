use std::env;
use std::io::Read;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;

/// Custom ParseCallbacks to convert Doxygen documentation to Rustdoc
#[derive(Debug)]
struct DoxygenCallbacks;

impl bindgen::callbacks::ParseCallbacks for DoxygenCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(comment))
    }
}

const LIB60870_VERSION: &str = "2.3.6";
const LIB60870_URL: &str =
    "https://github.com/mz-automation/lib60870/archive/refs/tags/v2.3.6.tar.gz";

const MBEDTLS_VERSION: &str = "2.28.9";
const MBEDTLS_URL: &str = "https://github.com/Mbed-TLS/mbedtls/archive/refs/tags/v2.28.9.tar.gz";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib60870_dir = out_dir.join(format!("lib60870-{}", LIB60870_VERSION));

    // Download and extract lib60870 if not already present
    if !lib60870_dir.exists() {
        download_and_extract_lib60870(&out_dir, &lib60870_dir);
    }

    let lib60870_c_dir = lib60870_dir.join("lib60870-C");
    let tls_enabled = env::var("CARGO_FEATURE_TLS").is_ok();

    // Download and setup mbedtls if TLS feature is enabled
    if tls_enabled {
        let mbedtls_target = lib60870_c_dir.join("dependencies/mbedtls-2.28");
        if !mbedtls_target.exists() {
            download_and_extract_mbedtls(&out_dir, &mbedtls_target);
        }
    }

    // Build lib60870 with cmake
    let dst = build_lib60870(&lib60870_c_dir, tls_enabled);

    // Generate bindings
    generate_bindings(&lib60870_c_dir, &out_dir, tls_enabled);

    // Link instructions
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=lib60870");

    // Platform-specific link libraries
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=rt");
            println!("cargo:rustc-link-lib=m");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=pthread");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=iphlpapi");
            println!("cargo:rustc-link-lib=bcrypt");
        }
        _ => {}
    }

    // Rerun if build.rs or wrapper.h changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
}

fn download_and_extract_tarball(url: &str, out_dir: &Path) {
    let tarball_data = ureq::get(url)
        .call()
        .expect("Failed to download tarball")
        .into_body()
        .read_to_vec()
        .expect("Failed to read response");

    // Decompress gzip
    let tar_data = {
        let mut decoder = GzDecoder::new(&tarball_data[..]);
        let mut tar_data = Vec::new();
        decoder
            .read_to_end(&mut tar_data)
            .expect("Failed to decompress gzip");
        tar_data
    };

    // Extract tar archive
    let mut archive = tar::Archive::new(&tar_data[..]);
    archive
        .unpack(out_dir)
        .expect("Failed to extract tar archive");
}

fn download_and_extract_lib60870(out_dir: &Path, lib60870_dir: &Path) {
    println!("cargo:warning=Downloading lib60870 v{}", LIB60870_VERSION);
    download_and_extract_tarball(LIB60870_URL, out_dir);

    assert!(
        lib60870_dir.exists(),
        "Expected lib60870 directory not found after extraction"
    );
}

fn download_and_extract_mbedtls(out_dir: &Path, mbedtls_target: &Path) {
    println!("cargo:warning=Downloading mbedtls v{}", MBEDTLS_VERSION);
    download_and_extract_tarball(MBEDTLS_URL, out_dir);

    // mbedtls extracts as mbedtls-2.28.9, but lib60870 expects mbedtls-2.28
    let mbedtls_extracted = out_dir.join(format!("mbedtls-{}", MBEDTLS_VERSION));
    assert!(
        mbedtls_extracted.exists(),
        "Expected mbedtls directory not found after extraction"
    );

    // Create dependencies directory if it doesn't exist
    if let Some(parent) = mbedtls_target.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create dependencies directory");
    }

    // Move/rename to the expected location
    std::fs::rename(&mbedtls_extracted, mbedtls_target)
        .expect("Failed to move mbedtls to dependencies directory");
}

fn build_lib60870(lib60870_c_dir: &Path, tls_enabled: bool) -> PathBuf {
    let mut config = cmake::Config::new(lib60870_c_dir);

    // Disable examples and tests
    config.define("BUILD_EXAMPLES", "OFF");
    config.define("BUILD_TESTS", "OFF");

    // Fix compatibility with newer CMake versions (3.30+)
    // lib60870's CMakeLists.txt uses an old cmake_minimum_required
    config.define("CMAKE_POLICY_VERSION_MINIMUM", "3.5");

    // Feature: debug output
    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        config.define("CONFIG_DEBUG_OUTPUT", "1");
    }

    // Feature: no-threads
    if env::var("CARGO_FEATURE_NO_THREADS").is_ok() {
        config.define("CONFIG_USE_THREADS", "0");
        config.define("CONFIG_USE_SEMAPHORES", "0");
    }

    // Feature: tcp-keepalive
    if env::var("CARGO_FEATURE_TCP_KEEPALIVE").is_ok() {
        config.define("CONFIG_ACTIVATE_TCP_KEEPALIVE", "1");
    }

    // Feature: TLS support
    // Note: cmake will auto-detect mbedtls in dependencies/mbedtls-2.28
    if tls_enabled {
        println!("cargo:warning=Building lib60870 with TLS support");
    }

    config.build()
}

fn generate_bindings(lib60870_c_dir: &Path, out_dir: &Path, tls_enabled: bool) {
    let api_inc = lib60870_c_dir.join("src/inc/api");
    let hal_inc = lib60870_c_dir.join("src/hal/inc");
    let common_inc = lib60870_c_dir.join("src/common/inc");
    let config_dir = lib60870_c_dir.join("config");

    let wrapper_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("wrapper.h");

    let mut builder = bindgen::Builder::default()
        .header(wrapper_path.to_str().unwrap())
        .clang_arg(format!("-I{}", api_inc.display()))
        .clang_arg(format!("-I{}", hal_inc.display()))
        .clang_arg(format!("-I{}", common_inc.display()))
        .clang_arg(format!("-I{}", config_dir.display()));

    // Add TLS-related include paths and defines
    if tls_enabled {
        let tls_mbedtls_inc = lib60870_c_dir.join("src/hal/tls/mbedtls");
        let mbedtls_inc = lib60870_c_dir.join("dependencies/mbedtls-2.28/include");

        builder = builder
            .clang_arg(format!("-I{}", tls_mbedtls_inc.display()))
            .clang_arg(format!("-I{}", mbedtls_inc.display()))
            .clang_arg("-DCONFIG_CS104_SUPPORT_TLS=1");
    }

    let bindings = builder
        // Parse inline functions
        .generate_inline_functions(true)
        // Generate comments from C headers
        .generate_comments(true)
        // Parse all comments (not just doc comments in /** */ format)
        .clang_arg("-fparse-all-comments")
        // Convert Doxygen to Rustdoc
        .parse_callbacks(Box::new(DoxygenCallbacks))
        // Derive common traits
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        // Blocklist va_list related items that can cause issues on different platforms
        .blocklist_item("__va_list_tag")
        .blocklist_item("va_list")
        .blocklist_item("__builtin_va_list")
        .blocklist_item("__darwin_va_list")
        .blocklist_item("__gnuc_va_list")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
