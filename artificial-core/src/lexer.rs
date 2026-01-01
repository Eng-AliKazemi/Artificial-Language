// artificial-language/artificial-core/src/lexer.rs
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Accrete,
    StringLiteral(String),
    Semicolon,
    Eof,
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lex error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            if self.is_at_end() {
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    line: self.line,
                    column: self.column,
                });
                break;
            }

            let ch = self.current_char();

            if ch.is_alphabetic() {
                let start_col = self.column;
                let word = self.read_identifier();

                match word.as_str() {
                    "accrete" => {
                        tokens.push(Token {
                            kind: TokenKind::Accrete,
                            line: self.line,
                            column: start_col,
                        });
                    }
                    _ => {
                        return Err(LexError {
                            message: format!("Unknown identifier: {}", word),
                            line: self.line,
                            column: start_col,
                        });
                    }
                }
            } else if ch == '"' {
                let start_col = self.column;
                let string_literal = self.read_string_literal()?;
                tokens.push(Token {
                    kind: TokenKind::StringLiteral(string_literal),
                    line: self.line,
                    column: start_col,
                });
            } else if ch == ';' {
                tokens.push(Token {
                    kind: TokenKind::Semicolon,
                    line: self.line,
                    column: self.column,
                });
                self.advance();
            } else {
                return Err(LexError {
                    message: format!("Unexpected character: '{}'", ch),
                    line: self.line,
                    column: self.column,
                });
            }
        }

        Ok(tokens)
    }

    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            ident.push(self.current_char());
            self.advance();
        }
        ident
    }

    fn read_string_literal(&mut self) -> Result<String, LexError> {
        self.advance(); // Consume opening quote
        let mut string = String::new();
        let start_line = self.line;
        let start_col = self.column;

        while !self.is_at_end() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err(LexError {
                        message: "Unterminated string literal".to_string(),
                        line: start_line,
                        column: start_col,
                    });
                }
                match self.current_char() {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '\\' => string.push('\\'),
                    '"' => string.push('"'),
                    _ => {
                        string.push('\\');
                        string.push(self.current_char());
                    }
                }
                self.advance();
            } else {
                string.push(self.current_char());
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(LexError {
                message: "Unterminated string literal".to_string(),
                line: start_line,
                column: start_col,
            });
        }

        self.advance(); // Consume closing quote
        Ok(string)
    }
}
