use std::process::Command;

pub fn build_dylib(dylib_path: &str) {
    let output = Command::new("cargo")
        .arg("build")
        .current_dir(dylib_path)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Cargo build successful");
    } else {
        // If the command didn't succeed, you can inspect the output for details
        eprintln!("Cargo build failed");
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
