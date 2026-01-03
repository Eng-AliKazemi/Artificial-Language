// artificial-language/artificial-vm/src/bytecode.rs
use std::fmt;

/// Opcodes for the Artificial Language Virtual Machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    /// No operation
    Nop = 0x00,
    /// Halt execution
    Halt = 0x01,
    /// Push constant from pool onto stack (followed by u16 index)
    PushConst = 0x02,
    /// Pop value and print without newline
    Print = 0x03,
    /// Pop value and print with newline
    PrintLn = 0x04,
    /// Pop top of stack and discard
    Pop = 0x05,
}

impl Opcode {
    pub fn from_u8(byte: u8) -> Option<Opcode> {
        match byte {
            0x00 => Some(Opcode::Nop),
            0x01 => Some(Opcode::Halt),
            0x02 => Some(Opcode::PushConst),
            0x03 => Some(Opcode::Print),
            0x04 => Some(Opcode::PrintLn),
            0x05 => Some(Opcode::Pop),
            _ => None,
        }
    }
}

/// Runtime values in the VM
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

/// Compiled bytecode module
#[derive(Debug, Clone)]
pub struct BytecodeModule {
    /// Constant pool containing literal values
    pub constants: Vec<Value>,
    /// Raw bytecode instructions
    pub instructions: Vec<u8>,
}

impl BytecodeModule {
    pub fn new() -> Self {
        BytecodeModule {
            constants: Vec::new(),
            instructions: Vec::new(),
        }
    }

    /// Add a constant to the pool and return its index
    pub fn add_constant(&mut self, value: Value) -> u16 {
        // Check if constant already exists
        for (i, existing) in self.constants.iter().enumerate() {
            if existing == &value {
                return i as u16;
            }
        }
        let index = self.constants.len() as u16;
        self.constants.push(value);
        index
    }

    /// Emit a single-byte opcode
    pub fn emit(&mut self, opcode: Opcode) {
        self.instructions.push(opcode as u8);
    }

    /// Emit an opcode with a u16 operand
    pub fn emit_with_operand(&mut self, opcode: Opcode, operand: u16) {
        self.instructions.push(opcode as u8);
        self.instructions.push((operand >> 8) as u8);
        self.instructions.push((operand & 0xFF) as u8);
    }
}

impl Default for BytecodeModule {
    fn default() -> Self {
        Self::new()
    }
}
