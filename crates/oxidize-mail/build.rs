use std::env;
use std::path::Path;
use std::process::Command;

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
            "--sourcedir=resources",  // Where to find the files
            "resources/oxidize-mail.gresource.xml",  // The definition file
        ])
        .output()
        .expect(
            "Failed to run glib-compile-resources. Is it installed?\n\
             Try: sudo apt install libglib2.0-dev-bin (Ubuntu/Debian)\n\
             Or: sudo pacman -S glib2 (Arch)\n\
             Or: brew install glib (macOS)"
        );

    // Check if it worked
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("glib-compile-resources failed:\n{}", stderr);
    }

    println!("✅ Resources compiled successfully!");
}
