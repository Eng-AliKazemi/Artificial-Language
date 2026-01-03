// artificial-language/ALC/src/main.rs
mod cli;
mod runner;
mod bundler;

use cli::parse_cli_args;
use runner::run_compiler;

fn main() {
    let opts = match parse_cli_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: ALC [OPTIONS] <source-file>");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  --emit-bytecode    Save bytecode to .artb file");
            eprintln!("  --no-run           Compile without executing");
            eprintln!("  --bundle           Create standalone executable");
            eprintln!("  --out-dir <DIR>    Output directory (default: target/artificial_out)");
            eprintln!("  --debug            Enable debug output");
            std::process::exit(1);
        }
    };

    if let Err(e) = run_compiler(&opts) {
        eprintln!("Compiler Error: {}", e);
        std::process::exit(1);
    }
}
