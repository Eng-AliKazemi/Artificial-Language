// artificial-language/ALC/src/runner.rs
use crate::cli::CliOptions;
use crate::hooks::run_hook;
use artificial_core::{Lexer, Parser, lower_module};
use artificial_backend_rust::compile_to_binary;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run_compiler(opts: &CliOptions) -> Result<(), String> {
    let source = fs::read_to_string(&opts.source_file)
        .map_err(|e| format!("Failed to read source file '{}': {}", &opts.source_file, e))?;

    let lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_module().map_err(|e| e.to_string())?;

    if opts.debug {
        println!("AST = {:#?}", ast);
    }

    let ir = lower_module(ast);
    if opts.debug {
        println!("IR = {:#?}", ir);
    }

    let out_dir = Path::new(&opts.out_dir);
    let file_stem = Path::new(&opts.source_file)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    run_hook("scripts/pre_build.sh");

    let binary_path = compile_to_binary(&ir, out_dir, file_stem)
        .map_err(|e| format!("Backend compilation failed: {}", e))?;

    run_hook("scripts/post_build.sh");

    if opts.emit_rs || opts.no_run {
        return Ok(());
    }

    let output = Command::new(&binary_path)
        .output()
        .map_err(|e| format!("Failed to execute binary: {}", e))?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(format!("Binary exited with error code: {}", output.status));
    }

    run_hook("scripts/post_run.sh");

    Ok(())
}
