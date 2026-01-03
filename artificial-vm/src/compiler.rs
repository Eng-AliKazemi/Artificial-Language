// artificial-language/artificial-vm/src/compiler.rs
use crate::bytecode::{BytecodeModule, Opcode, Value};
use artificial_core::{IrExpression, IrModule, IrStatement};

/// Compiles IR to bytecode
pub struct BytecodeCompiler {
    module: BytecodeModule,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        BytecodeCompiler {
            module: BytecodeModule::new(),
        }
    }

    /// Compile an IR module to bytecode
    pub fn compile(mut self, ir: &IrModule) -> BytecodeModule {
        for statement in &ir.statements {
            self.compile_statement(statement);
        }

        // Always end with Halt
        self.module.emit(Opcode::Halt);

        self.module
    }

    fn compile_statement(&mut self, stmt: &IrStatement) {
        match stmt {
            IrStatement::Accrete(expr) => {
                self.compile_expression(expr);
                self.module.emit(Opcode::PrintLn);
            }
        }
    }

    fn compile_expression(&mut self, expr: &IrExpression) {
        match expr {
            IrExpression::StringLiteral(s) => {
                let index = self.module.add_constant(Value::String(s.clone()));
                self.module.emit_with_operand(Opcode::PushConst, index);
            }
        }
    }
}

impl Default for BytecodeCompiler {
    fn default() -> Self {
        Self::new()
    }
}
