use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let web_dir = Path::new(&manifest_dir).join("../../web");
    let build_dir = web_dir.join("build");

    // Skip npm build if web/build already exists (Docker case)
    if build_dir.exists() {
        println!("Web build directory already exists, skipping npm build");
        return;
    }

    let output = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(&web_dir)
        .output()
        .expect("Failed to run npm build");

    if !output.status.success() {
        panic!(
            "npm build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
