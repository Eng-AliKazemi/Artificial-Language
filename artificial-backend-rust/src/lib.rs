// artificial-language/artificial-backend-rust/src/lib.rs
pub mod codegen;
pub mod emitter;
pub mod invoke_rustc;

pub use codegen::RustCodegen;
pub use emitter::write_rust_file;
pub use invoke_rustc::compile;

use artificial_core::IrModule;
use std::io;
use std::path::{Path, PathBuf};

pub fn compile_to_binary(
    module: &IrModule,
    out_dir: &Path,
    file_stem: &str,
) -> io::Result<PathBuf> {
    // Generate Rust code
    let rust_code = RustCodegen::generate(module);

    // Write to file
    let rs_file = write_rust_file(out_dir, file_stem, &rust_code)?;

    // Compile with rustc
    let binary_path = compile(&rs_file, out_dir, file_stem)?;

    Ok(binary_path)
}

