// artificial-language/ALC/src/cli.rs
pub struct CliOptions {
    pub source_file: String,
    pub emit_rs: bool,
    pub no_run: bool,
    pub out_dir: String,
    pub debug: bool,
}

impl Default for CliOptions {
    fn default() -> Self {
        CliOptions {
            source_file: String::new(),
            emit_rs: false,
            no_run: false,
            out_dir: "target/artificial_out".to_string(),
            debug: false,
        }
    }
}

pub fn parse_cli_args() -> Result<CliOptions, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = CliOptions::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--emit-rs" => {
                opts.emit_rs = true;
                i += 1;
            }
            "--no-run" => {
                opts.no_run = true;
                i += 1;
            }
            "--debug" => {
                opts.debug = true;
                i += 1;
            }
            "--out-dir" => {
                i += 1;
                if i >= args.len() {
                    return Err("--out-dir requires an argument".to_string());
                }
                opts.out_dir = args[i].clone();
                i += 1;
            }
            arg if !arg.starts_with("--") => {
                if opts.source_file.is_empty() {
                    opts.source_file = arg.to_string();
                } else {
                    return Err(format!("Unexpected positional argument: {}", arg));
                }
                i += 1;
            }
            arg => {
                return Err(format!("Unknown option: {}", arg));
            }
        }
    }

    if opts.source_file.is_empty() {
        return Err("No source file specified".to_string());
    }
    Ok(opts)
}
