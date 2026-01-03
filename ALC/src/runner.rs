// artificial-language/ALC/src/runner.rs
use crate::bundler;
use crate::cli::CliOptions;
use artificial_core::{lower_module, Lexer, Parser};
use artificial_vm::{BytecodeCompiler, VirtualMachine, write_to_file};
use std::fs;
use std::path::Path;

pub fn run_compiler(opts: &CliOptions) -> Result<(), String> {
    // Read source file
    let source = fs::read_to_string(&opts.source_file)
        .map_err(|e| format!("Failed to read source file '{}': {}", &opts.source_file, e))?;

    // Lexical analysis
    let lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    if opts.debug {
        println!("[DEBUG] Tokens: {:#?}", tokens);
    }

    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_module().map_err(|e| e.to_string())?;

    if opts.debug {
        println!("[DEBUG] AST: {:#?}", ast);
    }

    // Lowering to IR
    let ir = lower_module(ast);

    if opts.debug {
        println!("[DEBUG] IR: {:#?}", ir);
    }

    // Compile to bytecode
    let compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile(&ir);

    if opts.debug {
        println!("[DEBUG] Bytecode constants: {:?}", bytecode.constants);
        println!("[DEBUG] Bytecode instructions: {:?}", bytecode.instructions);
    }

    // Ensure output directory exists
    let out_dir = Path::new(&opts.out_dir);
    fs::create_dir_all(out_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Get file stem for output naming
    let file_stem = Path::new(&opts.source_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // Optionally emit bytecode to file
    if opts.emit_bytecode || opts.bundle {
        let bytecode_path = out_dir.join(format!("{}.artb", file_stem));
        write_to_file(&bytecode, &bytecode_path)
            .map_err(|e| format!("Failed to write bytecode file: {}", e))?;

        if opts.debug {
            println!("[DEBUG] Bytecode written to: {:?}", bytecode_path);
        }
    }

    // Create standalone bundle if requested
    if opts.bundle {
        let binary_path = bundler::create_standalone(&bytecode, out_dir, file_stem)?;

        if opts.debug {
            println!("[DEBUG] Standalone binary created at: {:?}", binary_path);
        }
    }

    // Execute unless --no-run is specified
    if !opts.no_run && !opts.bundle {
        let mut vm = VirtualMachine::new();
        vm.execute(&bytecode).map_err(|e| e.to_string())?;
    }

    Ok(())
}
