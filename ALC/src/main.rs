// artificial-language/ALC/src/main.rs
mod cli;
mod hooks;
mod runner;

use cli::parse_cli_args;
use runner::run_compiler;

fn main() {
    let opts = match parse_cli_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: ALC [OPTIONS] <source-file>");
            std::process::exit(1);
        }
    };

    if let Err(e) = run_compiler(&opts) {
        eprintln!("Compiler Error: {}", e);
        std::process::exit(1);
    }
}
