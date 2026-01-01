// artificial-language/artificial-core/src/ir.rs
use std::fmt;

#[derive(Debug, Clone)]
pub struct IrModule {
    pub statements: Vec<IrStatement>,
}

#[derive(Debug, Clone)]
pub enum IrStatement {
    Accrete(IrExpression),
}

#[derive(Debug, Clone)]
pub enum IrExpression {
    StringLiteral(String),
}

impl fmt::Display for IrModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IrModule {{")?;
        for stmt in &self.statements {
            writeln!(f, "  {:?}", stmt)?;
        }
        write!(f, "}}")
    }
}
