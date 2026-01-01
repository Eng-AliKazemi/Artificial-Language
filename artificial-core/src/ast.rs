// artificial-language/artificial-core/src/ast.rs
use std::fmt;

#[derive(Debug, Clone)]
pub struct AstModule {
    pub statements: Vec<AstStatement>,
}

#[derive(Debug, Clone)]
pub enum AstStatement {
    Accrete(AstExpression),
}

#[derive(Debug, Clone)]
pub enum AstExpression {
    StringLiteral(String),
}

impl fmt::Display for AstModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AstModule {{")?;
        for stmt in &self.statements {
            writeln!(f, "  {:?}", stmt)?;
        }
        write!(f, "}}")
    }
}
