// artificial-language/artificial-vm/src/serializer.rs
use crate::bytecode::{BytecodeModule, Value};
use std::io::{self, Read, Write};

/// Magic bytes for Artificial Language Bytecode files
const MAGIC: &[u8; 4] = b"ARTB";
/// Current bytecode version
const VERSION: u8 = 1;

/// Value type tags for serialization
mod type_tags {
    pub const NULL: u8 = 0;
    pub const STRING: u8 = 1;
    pub const INTEGER: u8 = 2;
    pub const BOOLEAN: u8 = 3;
}

/// Serialize a bytecode module to bytes
pub fn serialize(module: &BytecodeModule) -> Vec<u8> {
    let mut bytes = Vec::new();

    // Magic bytes
    bytes.extend_from_slice(MAGIC);

    // Version
    bytes.push(VERSION);

    // Constant pool
    let const_count = module.constants.len() as u32;
    bytes.extend_from_slice(&const_count.to_be_bytes());

    for constant in &module.constants {
        serialize_value(&mut bytes, constant);
    }

    // Instructions
    let instr_count = module.instructions.len() as u32;
    bytes.extend_from_slice(&instr_count.to_be_bytes());
    bytes.extend_from_slice(&module.instructions);

    bytes
}

fn serialize_value(bytes: &mut Vec<u8>, value: &Value) {
    match value {
        Value::Null => {
            bytes.push(type_tags::NULL);
        }
        Value::String(s) => {
            bytes.push(type_tags::STRING);
            let len = s.len() as u32;
            bytes.extend_from_slice(&len.to_be_bytes());
            bytes.extend_from_slice(s.as_bytes());
        }
        Value::Integer(n) => {
            bytes.push(type_tags::INTEGER);
            bytes.extend_from_slice(&n.to_be_bytes());
        }
        Value::Boolean(b) => {
            bytes.push(type_tags::BOOLEAN);
            bytes.push(if *b { 1 } else { 0 });
        }
    }
}

/// Deserialize bytes to a bytecode module
pub fn deserialize(bytes: &[u8]) -> io::Result<BytecodeModule> {
    let mut cursor = std::io::Cursor::new(bytes);
    deserialize_from_reader(&mut cursor)
}

pub fn deserialize_from_reader<R: Read>(reader: &mut R) -> io::Result<BytecodeModule> {
    // Magic bytes
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid bytecode file: bad magic bytes",
        ));
    }

    // Version
    let mut version = [0u8; 1];
    reader.read_exact(&mut version)?;
    if version[0] != VERSION {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unsupported bytecode version: {}", version[0]),
        ));
    }

    // Constant pool
    let mut const_count_bytes = [0u8; 4];
    reader.read_exact(&mut const_count_bytes)?;
    let const_count = u32::from_be_bytes(const_count_bytes) as usize;

    let mut constants = Vec::with_capacity(const_count);
    for _ in 0..const_count {
        constants.push(deserialize_value(reader)?);
    }

    // Instructions
    let mut instr_count_bytes = [0u8; 4];
    reader.read_exact(&mut instr_count_bytes)?;
    let instr_count = u32::from_be_bytes(instr_count_bytes) as usize;

    let mut instructions = vec![0u8; instr_count];
    reader.read_exact(&mut instructions)?;

    Ok(BytecodeModule {
        constants,
        instructions,
    })
}

fn deserialize_value<R: Read>(reader: &mut R) -> io::Result<Value> {
    let mut type_tag = [0u8; 1];
    reader.read_exact(&mut type_tag)?;

    match type_tag[0] {
        type_tags::NULL => Ok(Value::Null),
        type_tags::STRING => {
            let mut len_bytes = [0u8; 4];
            reader.read_exact(&mut len_bytes)?;
            let len = u32::from_be_bytes(len_bytes) as usize;
            let mut string_bytes = vec![0u8; len];
            reader.read_exact(&mut string_bytes)?;
            let s = String::from_utf8(string_bytes).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Invalid UTF-8: {}", e))
            })?;
            Ok(Value::String(s))
        }
        type_tags::INTEGER => {
            let mut int_bytes = [0u8; 8];
            reader.read_exact(&mut int_bytes)?;
            Ok(Value::Integer(i64::from_be_bytes(int_bytes)))
        }
        type_tags::BOOLEAN => {
            let mut bool_byte = [0u8; 1];
            reader.read_exact(&mut bool_byte)?;
            Ok(Value::Boolean(bool_byte[0] != 0))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown value type tag: {}", type_tag[0]),
        )),
    }
}

/// Write bytecode to a file
pub fn write_to_file(module: &BytecodeModule, path: &std::path::Path) -> io::Result<()> {
    let bytes = serialize(module);
    let mut file = std::fs::File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Read bytecode from a file
pub fn read_from_file(path: &std::path::Path) -> io::Result<BytecodeModule> {
    let bytes = std::fs::read(path)?;
    deserialize(&bytes)
}
