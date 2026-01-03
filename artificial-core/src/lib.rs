// artificial-language/artificial-core/src/lib.rs
pub mod ast;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod lowering;

pub use ast::*;
pub use ir::*;
pub use lexer::*;
pub use parser::*;
pub use lowering::*;

