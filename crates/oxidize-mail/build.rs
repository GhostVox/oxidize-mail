//! Build script for the Oxidize Mail application.
//!
//! This build script compiles GTK/GLib resources into a binary format that can be
//! embedded into the application. It uses `glib-compile-resources` to process the
//! resource definition file and generate a compiled resource bundle.
//!
//! # Resource Compilation
//!
//! The script compiles resources defined in `resources/oxidize-mail.gresource.xml`
//! into a binary `.gresource` file that gets embedded into the application at
//! compile time. This allows the application to access CSS files, UI definitions,
//! and other assets without requiring external files at runtime.
//!
//! # Requirements
//!
//! - `glib-compile-resources` must be installed on the build system
//! - Resource files must exist in the `resources/` directory
//! - Resource definition file must be valid XML
//!
//! # Error Handling
//!
//! The script will fail the build if:
//! - `glib-compile-resources` is not found
//! - Resource compilation fails
//! - Required resource files are missing

use std::env;
use std::path::Path;
use std::process::Command;

/// Main build script entry point for compiling GTK resources.
///
/// This function performs the following steps:
/// 1. Sets up Cargo rerun triggers for resource file changes
/// 2. Determines the output path for the compiled resource file
/// 3. Executes `glib-compile-resources` to compile the resources
/// 4. Validates the compilation was successful
///
/// # Panics
///
/// Panics if:
/// - `glib-compile-resources` is not installed or not in PATH
/// - Resource compilation fails for any reason
/// - Output directory cannot be determined
fn main() {
    // Tell Cargo to rerun this script if resources change
    println!("cargo:rerun-if-changed=resources/");
    println!("cargo:rerun-if-changed=resources/oxidize-mail.gresource.xml");

    // Get the output directory where Cargo puts compiled files
    let out_dir = env::var("OUT_DIR").unwrap();
    let output_path = Path::new(&out_dir).join("oxidize-mail.gresource");

    // Compile the resource file
    let output = Command::new("glib-compile-resources")
        .args(&[
            &format!("--target={}", output_path.display()),
            "--sourcedir=resources",                // Where to find the files
            "resources/oxidize-mail.gresource.xml", // The definition file
        ])
        .output()
        .expect(
            "Failed to run glib-compile-resources. Is it installed?\n\
             Try: sudo apt install libglib2.0-dev-bin (Ubuntu/Debian)\n\
             Or: sudo pacman -S glib2 (Arch)\n\
             Or: brew install glib (macOS)",
        );

    // Check if it worked
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("glib-compile-resources failed:\n{}", stderr);
    }

    println!("✅ Resources compiled successfully!");
}
