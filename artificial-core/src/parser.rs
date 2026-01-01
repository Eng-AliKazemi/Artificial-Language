// artificial-language/artificial-core/src/parser.rs
use crate::ast::*;
use crate::lexer::{Token, TokenKind};
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse_module(&mut self) -> Result<AstModule, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_eof() {
            statements.push(self.parse_statement()?);
        }

        Ok(AstModule { statements })
    }

    fn parse_statement(&mut self) -> Result<AstStatement, ParseError> {
        if self.check(&TokenKind::Accrete) {
            self.parse_accrete_statement()
        } else {
            let token = self.current_token();
            Err(ParseError {
                message: format!("Expected statement, found {:?}", token.kind),
                line: token.line,
                column: token.column,
            })
        }
    }

    fn parse_accrete_statement(&mut self) -> Result<AstStatement, ParseError> {
        self.consume(&TokenKind::Accrete)?;
        let expr = self.parse_expression()?;
        if self.check(&TokenKind::Semicolon) {
            self.consume(&TokenKind::Semicolon)?;
        }
        Ok(AstStatement::Accrete(expr))
    }

    fn parse_expression(&mut self) -> Result<AstExpression, ParseError> {
        match &self.current_token().kind {
            TokenKind::StringLiteral(s) => {
                let string = s.clone();
                self.advance();
                Ok(AstExpression::StringLiteral(string))
            }
            _ => {
                let token = self.current_token();
                Err(ParseError {
                    message: format!("Expected expression, found {:?}", token.kind),
                    line: token.line,
                    column: token.column,
                })
            }
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens
            .get(self.position)
            .unwrap_or_else(|| &self.tokens[self.tokens.len() - 1])
    }

    fn is_at_eof(&self) -> bool {
        matches!(self.current_token().kind, TokenKind::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_eof() {
            self.position += 1;
        }
        &self.tokens[self.position - 1]
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.current_token().kind) == std::mem::discriminant(kind)
    }

    fn consume(&mut self, kind: &TokenKind) -> Result<&Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let token = self.current_token();
            Err(ParseError {
                message: format!("Expected {:?}, found {:?}", kind, token.kind),
                line: token.line,
                column: token.column,
            })
        }
    }
}
