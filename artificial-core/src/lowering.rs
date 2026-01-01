// artificial-language/artificial-core/src/lowering.rs
use crate::ast::*;
use crate::ir::*;

pub fn lower_module(ast: AstModule) -> IrModule {
    let statements = ast
        .statements
        .into_iter()
        .map(lower_statement)
        .collect();

    IrModule { statements }
}

fn lower_statement(stmt: AstStatement) -> IrStatement {
    match stmt {
        AstStatement::Accrete(expr) => IrStatement::Accrete(lower_expression(expr)),
    }
}

fn lower_expression(expr: AstExpression) -> IrExpression {
    match expr {
        AstExpression::StringLiteral(s) => IrExpression::StringLiteral(s),
    }
}
