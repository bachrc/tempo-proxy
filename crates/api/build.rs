use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let web_dir = Path::new(&manifest_dir).join("../../web");
    
    println!("cargo:rerun-if-changed=../../web/src");
    println!("cargo:rerun-if-changed=../../web/package.json");
    println!("cargo:rerun-if-changed=../../web/svelte.config.js");
    println!("cargo:rerun-if-changed=../../web/vite.config.js");
    
    if web_dir.exists() {
        let output = Command::new("npm")
            .args(&["run", "build"])
            .current_dir(&web_dir)
            .output()
            .expect("Failed to run npm build");
            
        if !output.status.success() {
            panic!("npm build failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}