// artificial-language/artificial-backend-rust/src/emitter.rs
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn write_rust_file(out_dir: &Path, file_stem: &str, code: &str) -> io::Result<PathBuf> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir)?;

    // Build file path
    let file_path = out_dir.join(format!("{}_art.rs", file_stem));

    // Write the code to file
    fs::write(&file_path, code)?;

    Ok(file_path)
}
