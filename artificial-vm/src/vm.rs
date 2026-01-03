// artificial-language/artificial-vm/src/vm.rs
use crate::bytecode::{BytecodeModule, Opcode, Value};
use std::fmt;
use std::io::{self, Write};

/// Virtual Machine execution errors
#[derive(Debug)]
pub struct VmError {
    pub message: String,
    pub ip: usize,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VM Error at instruction {}: {}", self.ip, self.message)
    }
}

impl std::error::Error for VmError {}

/// Stack-based Virtual Machine for Artificial Language
pub struct VirtualMachine {
    /// Operand stack
    stack: Vec<Value>,
    /// Instruction pointer
    ip: usize,
    /// Captured output (used for bundled execution)
    output: String,
    /// Whether to capture output or print directly
    capture_output: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            stack: Vec::with_capacity(256),
            ip: 0,
            output: String::new(),
            capture_output: false,
        }
    }

    /// Create a VM that captures output instead of printing
    pub fn with_captured_output() -> Self {
        VirtualMachine {
            stack: Vec::with_capacity(256),
            ip: 0,
            output: String::new(),
            capture_output: true,
        }
    }

    /// Execute a bytecode module
    pub fn execute(&mut self, module: &BytecodeModule) -> Result<(), VmError> {
        self.ip = 0;
        self.stack.clear();
        self.output.clear();

        while self.ip < module.instructions.len() {
            let opcode_byte = module.instructions[self.ip];
            let opcode = Opcode::from_u8(opcode_byte).ok_or_else(|| VmError {
                message: format!("Unknown opcode: 0x{:02X}", opcode_byte),
                ip: self.ip,
            })?;

            match opcode {
                Opcode::Nop => {
                    self.ip += 1;
                }

                Opcode::Halt => {
                    break;
                }

                Opcode::PushConst => {
                    let index = self.read_u16(module)?;
                    let value = module.constants.get(index as usize).ok_or_else(|| VmError {
                        message: format!("Constant index {} out of bounds", index),
                        ip: self.ip - 3,
                    })?;
                    self.stack.push(value.clone());
                }

                Opcode::Print => {
                    let value = self.pop()?;
                    self.print_value(&value, false)?;
                    self.ip += 1;
                }

                Opcode::PrintLn => {
                    let value = self.pop()?;
                    self.print_value(&value, true)?;
                    self.ip += 1;
                }

                Opcode::Pop => {
                    self.pop()?;
                    self.ip += 1;
                }
            }
        }

        Ok(())
    }

    /// Get captured output
    pub fn get_output(&self) -> &str {
        &self.output
    }

    fn read_u16(&mut self, module: &BytecodeModule) -> Result<u16, VmError> {
        if self.ip + 2 >= module.instructions.len() {
            return Err(VmError {
                message: "Unexpected end of bytecode while reading operand".to_string(),
                ip: self.ip,
            });
        }
        self.ip += 1;
        let high = module.instructions[self.ip] as u16;
        self.ip += 1;
        let low = module.instructions[self.ip] as u16;
        self.ip += 1;
        Ok((high << 8) | low)
    }

    fn pop(&mut self) -> Result<Value, VmError> {
        self.stack.pop().ok_or_else(|| VmError {
            message: "Stack underflow".to_string(),
            ip: self.ip,
        })
    }

    fn print_value(&mut self, value: &Value, newline: bool) -> Result<(), VmError> {
        let text = value.to_string();

        if self.capture_output {
            self.output.push_str(&text);
            if newline {
                self.output.push('\n');
            }
        } else {
            if newline {
                println!("{}", text);
            } else {
                print!("{}", text);
            }
            io::stdout().flush().ok();
        }

        Ok(())
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
