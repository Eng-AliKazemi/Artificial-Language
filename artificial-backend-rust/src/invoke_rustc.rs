// artificial-language/artificial-backend-rust/src/invoke_rustc.rs
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn compile(rs_file: &Path, out_dir: &Path, file_stem: &str) -> io::Result<PathBuf> {
    // Determine the binary path
    let binary_path = if cfg!(windows) {
        out_dir.join(format!("{}_art.exe", file_stem))
    } else {
        out_dir.join(format!("{}_art", file_stem))
    };

    // Run rustc
    let output = Command::new("rustc")
        .arg(rs_file)
        .arg("-o")
        .arg(&binary_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::other(
            format!("rustc compilation failed:\n{}", stderr),
        ));
    }

    Ok(binary_path)
}
