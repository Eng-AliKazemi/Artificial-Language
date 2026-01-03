// artificial-language/ALC/src/bundler.rs
use artificial_vm::{serialize, BytecodeModule};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Creates a standalone executable that embeds the VM and bytecode
pub fn create_standalone(
    bytecode: &BytecodeModule,
    out_dir: &Path,
    file_stem: &str,
) -> Result<PathBuf, String> {
    // Serialize bytecode
    let bytecode_bytes = serialize(bytecode);

    // Generate the embedded Rust source
    let rust_source = generate_standalone_source(&bytecode_bytes);

    // Write the generated source file
    let rs_path = out_dir.join(format!("{}_standalone.rs", file_stem));
    let mut file = fs::File::create(&rs_path)
        .map_err(|e| format!("Failed to create standalone source file: {}", e))?;
    file.write_all(rust_source.as_bytes())
        .map_err(|e| format!("Failed to write standalone source: {}", e))?;

    // Compile with rustc
    let binary_path = if cfg!(windows) {
        out_dir.join(format!("{}_art.exe", file_stem))
    } else {
        out_dir.join(format!("{}_art", file_stem))
    };

    let output = Command::new("rustc")
        .arg(&rs_path)
        .arg("-o")
        .arg(&binary_path)
        .arg("-O") // Optimize
        .output()
        .map_err(|e| format!("Failed to run rustc: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("rustc compilation failed:\n{}", stderr));
    }

    Ok(binary_path)
}

fn generate_standalone_source(bytecode_bytes: &[u8]) -> String {
    let mut source = String::new();

    // File header
    source.push_str("// Generated standalone executable for Artificial Language\n");
    source.push_str("// DO NOT EDIT - This file is auto-generated\n\n");

    // Embedded bytecode as a const array
    source.push_str("const BYTECODE: &[u8] = &[\n    ");
    for (i, byte) in bytecode_bytes.iter().enumerate() {
        source.push_str(&format!("0x{:02X}, ", byte));
        if (i + 1) % 12 == 0 {
            source.push_str("\n    ");
        }
    }
    source.push_str("\n];\n\n");

    // Embed the minimal VM implementation
    source.push_str(EMBEDDED_VM_SOURCE);

    source
}

/// Minimal VM implementation to embed in standalone executables
const EMBEDDED_VM_SOURCE: &str = r#"
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

struct BytecodeModule {
    constants: Vec<Value>,
    instructions: Vec<u8>,
}

fn deserialize(bytes: &[u8]) -> BytecodeModule {
    let mut pos = 0;
    
    // Skip magic (4 bytes) and version (1 byte)
    pos += 5;
    
    // Read constant count
    let const_count = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
    pos += 4;
    
    let mut constants = Vec::new();
    for _ in 0..const_count {
        let type_tag = bytes[pos];
        pos += 1;
        match type_tag {
            0 => constants.push(Value::Null),
            1 => {
                let len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
                pos += 4;
                let s = String::from_utf8(bytes[pos..pos+len].to_vec()).unwrap();
                pos += len;
                constants.push(Value::String(s));
            }
            2 => {
                let n = i64::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3],
                                           bytes[pos+4], bytes[pos+5], bytes[pos+6], bytes[pos+7]]);
                pos += 8;
                constants.push(Value::Integer(n));
            }
            3 => {
                let b = bytes[pos] != 0;
                pos += 1;
                constants.push(Value::Boolean(b));
            }
            _ => panic!("Unknown type tag"),
        }
    }
    
    // Read instruction count
    let instr_count = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
    pos += 4;
    
    let instructions = bytes[pos..pos+instr_count].to_vec();
    
    BytecodeModule { constants, instructions }
}

fn execute(module: &BytecodeModule) {
    let mut stack: Vec<Value> = Vec::new();
    let mut ip = 0;
    
    while ip < module.instructions.len() {
        match module.instructions[ip] {
            0x00 => ip += 1, // Nop
            0x01 => break,   // Halt
            0x02 => {        // PushConst
                let idx = ((module.instructions[ip+1] as u16) << 8) | (module.instructions[ip+2] as u16);
                stack.push(module.constants[idx as usize].clone());
                ip += 3;
            }
            0x03 => {        // Print
                if let Some(v) = stack.pop() {
                    print!("{}", v);
                    io::stdout().flush().ok();
                }
                ip += 1;
            }
            0x04 => {        // PrintLn
                if let Some(v) = stack.pop() {
                    println!("{}", v);
                }
                ip += 1;
            }
            0x05 => {        // Pop
                stack.pop();
                ip += 1;
            }
            _ => panic!("Unknown opcode"),
        }
    }
}

fn main() {
    let module = deserialize(BYTECODE);
    execute(&module);
}
"#;
