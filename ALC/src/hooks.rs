// artificial-language/ALC/src/hooks.rs
use std::path::Path;
use std::process::Command;

pub fn run_hook(hook_path: &str) {
    if Path::new(hook_path).exists() {
        let _ = Command::new("sh")
            .arg(hook_path)
            .status();
    }
}
