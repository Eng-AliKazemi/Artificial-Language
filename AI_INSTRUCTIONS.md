### **MASTER PROMPT: AUTONOMOUS ARTIFICIAL LANGUAGE COMPILER IMPLEMENTATION (V6)**

**Your Role:** You are an autonomous, senior-level Rust engineer and language tools developer.

**Your Mission:** Your sole task is to create a new compiler project for a new **programming language: Artificial Language** in Rust based on standard compiler development architecture. You will implement the entire project from end-to-end with **no human intervention or code edits**. The ultimate goal is to use this compiler to produce a single, standalone executable file named `Artificial-Language` that, when executed, prints "Hello Artificial World!" to the console. You will perform a complete, automated workflow: plan the project, create every file, write all the code, run all commands, and perform all debugging and iteration until the project is fully functional, robust, and completely free of any warnings.

---

### **Part 1: Core Directives & Project Plan**

#### **1.1. Overarching Principle: Autonomy and Expertise**

This prompt contains a complete, working implementation of the compiler, including all source code files, configuration files, and test cases. You must treat this as a **functional reference and a structural guide**. It demonstrates a valid path to success and defines the required project architecture (the workspace, the three crates, and the compilation pipeline).

However, you are a **senior engineer**. Your primary directive is to use your own expert knowledge to generate all code. **You must not simply copy the provided code verbatim.** Instead, you will write your own mature, production-ready, clean, and extensible implementations that follow modern Rust idioms and best practices. If you have a better, more efficient, or cleaner approach that still meets the project's goals, you are required to use it. The fundamental project structure must be preserved.

You must perform this entire task automatically and without asking the user for assistance. Analyze the following steps, use the provided examples as a reference for the goal, and execute the plan from start to finish.

#### **1.2. Global Rules & Quality Mandates (Non-Negotiable)**

1.  **Zero Warnings Policy**: You must treat **all** compiler warnings as errors. After every `cargo build`, `cargo check`, or `cargo clippy` command, you will meticulously inspect the output. If **any** warning appears, you must immediately **stop**, modify the code to resolve the warning, and re-run the build command. This cycle must be repeated until the build is perfectly clean.
2.  **Mandatory `.gitignore` Files**: You must create four separate `.gitignore` files as one of the first steps. The paths and content are specified precisely in Step 1.7 below.
3.  **No Human Interaction**: You must not ask the user to edit files, run commands, or debug failures. You are responsible for the entire lifecycle.
4.  **File Path Comments**: When generating Rust code for any `.rs` file, you must **always** place a comment at the very top with the full file path (e.g., `// artificial-language/ALC/src/main.rs`).
5.  **Strict Adherence to Best Practices**: You must follow all guidelines outlined in the **Appendix** section of this prompt.

#### **1.3. Final Success Criteria**

The project is considered complete only when **all** of the following criteria are met in order:

1.  The `ALC` compiler successfully compiles the `main.art` source file into a standalone, executable binary.
2.  This final, compiled binary is programmatically copied from its build output directory to the project's root directory (`artificial-language/`).
3.  The copied binary is renamed to `Artificial-Language`.
4.  The console is cleared via the `clear` command.
5.  Immediately following the `clear` command, the `Artificial-Language` binary is executed directly from the project root with the command `./Artificial-Language`.
6.  The execution produces the **exact** text `Hello Artificial World!` as the **only** output visible on the cleared console.

#### **1.4. Step-by-Step Implementation Plan**

**Step 1 — Initialize the Workspace and Git Repository**

1.1. Create the root project directory: `mkdir artificial-language`
1.2. Change into the new directory: `cd artificial-language`
1.3. **Initialize a single Git repository for the entire workspace:** `git init`
1.4. Create the root `Cargo.toml` file with the following content:
```toml
// artificial-language/Cargo.toml
[workspace]
resolver = "2"
members = ["artificial-core", "artificial-backend-rust", "ALC"]

[workspace.package]
version = "0.1.0"
edition = "2021"
```
1.5. Initialize the three member crates **without creating nested Git repositories**:
`cargo new artificial-core --lib --vcs none`
`cargo new artificial-backend-rust --lib --vcs none`
`cargo new ALC --bin --vcs none`

1.6. Create the `main.art` file in the project root with the following content:
```text
// artificial-language/main.art
accrete "Hello Artificial World!"
```

1.7. **Create four `.gitignore` files** in the specified locations. Each file must contain the content below:
    *   `./.gitignore` (at the workspace root)
    *   `./artificial-core/.gitignore`
    *   `./artificial-backend-rust/.gitignore`
    *   `./ALC/.gitignore`

Content for each `.gitignore` file:
```gitignore
# Rust build artifacts
/target
/debug
/release
**/*.rs.bk
Cargo.lock

# Generated compiler outputs
target/artificial_out/
**/*_art.rs
**/*_art

# OS & IDE junk
.DS_Store
*.log
.vscode/
.idea/
*.swp
*.swo
*~
```

1.8. Run `cargo build`. **Review the output for warnings.** Fix any and all warnings before proceeding.

**Step 2 — Define Core Types in `artificial-core`**

- 2.1. In `artificial-core/src/`, create `ast.rs` and `ir.rs`.
- 2.2. Define the AST types (`AstModule`, `AstStatement`, `AstExpression`). The statement type must handle the `accrete` keyword.
- 2.3. Define the IR types (`IrModule`, `IrStatement`, `IrExpression`), which will initially mirror the AST.
- 2.4. Update `artificial-core/src/lib.rs` to declare and re-export the modules for the entire crate.

**Step 3 — Implement the Lexer in `artificial-core`**

- 3.1. In `artificial-core/src/`, create `lexer.rs`.
- 3.2. Define the `Token` and `TokenKind` types.
- 3.3. Implement a `Lexer` struct with a `tokenize` method that recognizes the **`accrete`** keyword and double-quoted string literals.
- 3.4. Implement a clear `LexError` type that includes line and column information.
- 3.5. Run `cargo build` and fix **all** warnings.

**Step 4 — Implement the Parser in `artificial-core`**

- 4.1. In `artificial-core/src/`, create `parser.rs`.
- 4.2. Implement a `Parser` struct that consumes the lexer's tokens.
- 4.3. Implement a `parse_module` method to produce an `AstModule`. It only needs to support **`accrete <expression>;`** statements for now.
- 4.4. Implement a `ParseError` type with good contextual information.
- 4.5. Run `cargo build` and fix **all** warnings.

**Step 5 — Implement AST → IR Lowering in `artificial-core`**

- 5.1. In `artificial-core/src/`, create `lowering.rs`.
- 5.2. Implement the `lower_module` function to transform the `AstModule` into an `IrModule`. This will be a simple 1:1 mapping.
- 5.3. Run `cargo build` and fix **all** warnings.

**Step 6 — Implement the Rust Backend in `artificial-backend-rust`**

- 6.1. Add the dependency on `artificial-core` to `artificial-backend-rust/Cargo.toml`.
- 6.2. In `artificial-backend-rust/src/`, create `codegen.rs`.
- 6.3. Implement a `RustCodegen` utility to traverse the `IrModule` and generate a `String` of valid Rust source code.
- 6.4. The generated code must be a complete program (e.g., `fn main() { ... }`). **`accrete "..."`** should map to `println! "..."`.
- 6.5. Run `cargo build` and fix **all** warnings.

**Step 7 — Implement Emitter and `rustc` Invocation in `artificial-backend-rust`**

- 7.1. In `artificial-backend-rust/src/`, create `emitter.rs` to handle writing the generated Rust code to a file.
- 7.2. In `artificial-backend-rust/src/`, create `invoke_rustc.rs` to compile the `.rs` file into a binary using `std::process::Command`.
- 7.3. Update `artificial-backend-rust/src/lib.rs` to expose a single `compile_to_binary` function that orchestrates the codegen, emitting, and compilation steps.
- 7.4. Run `cargo build` and fix **all** warnings.

**Step 8 — Implement the CLI in `ALC`**

- 8.1. Add dependencies on `artificial-core` and `artificial-backend-rust` in `ALC/Cargo.toml`.
- 8.2. In `ALC/src/`, create `cli.rs` to parse command-line arguments using only the standard library. It must at least handle a single positional argument for the source file path and a `--no-run` flag.
- 8.3. Run `cargo build` and fix **all** warnings.

**Step 9 — Implement the Compiler Runner in `ALC`**

- 9.1. In `ALC/src/`, create `runner.rs`.
- 9.2. Implement the main `run_compiler` function that chains the entire process together: read source -> lex -> parse -> lower -> compile to binary. It must respect the `--no-run` flag.
- 9.3. Optional: create `hooks.rs` to run placeholder shell scripts.
- 9.4. Update `ALC/src/main.rs` to call the CLI parser and the runner, with robust error handling.
- 9.5. Run `cargo build` and fix **all** warnings.

**Step 10 — End-to-End Verification and Self-Correction**

- 10.1. This is the primary debugging and verification loop.
- 10.2. Run the compiler in debug/development mode to compile and immediately execute its output: `cargo run -p ALC -- main.art`.
- 10.3. **Verify the output.** If it is not exactly `Hello Artificial World!`, you must diagnose the logic error within your implementation, modify the relevant source file(s), rebuild, and re-test.
- 10.4. Repeat this self-correction loop until `cargo run -p ALC -- main.art` works perfectly and there are zero warnings from `cargo build` and `cargo clippy`.

**Step 11 — Final Product Generation and Deployment**

- 11.1. **After and only after Step 10 is successfully completed**, you will perform the final product generation and deployment.
- 11.2. Execute the `ALC` compiler in release mode to compile `main.art` into a binary, but **do not run it yet**. The output binary should be placed in a predictable location (e.g., `target/artificial_out/main_art`). Use the command: `cargo run -p ALC --release -- main.art --no-run`.
- 11.3. Programmatically locate the compiled binary at its output path (e.g., `target/artificial_out/main_art`).
- 11.4. Programmatically copy this binary to the project's root directory (`./`). You must use a method like `std::fs::copy`.
- 11.5. The destination path for the copy operation must be `./Artificial-Language`.
- 11.6. **(For non-Windows systems)** Programmatically make the final binary executable: `chmod +x ./Artificial-Language`.
- **The final two steps must be executed in sequence to meet the success criteria.**
- 11.7. First, clear all previous console history to ensure a clean final output: `clear`.
- 11.8. Second, execute the deployed binary to display the final output on the clean console: `./Artificial-Language`.

---

### **Part 2: Functional Code Examples & Reference**

This section contains the complete source for a working version of the project. Use it as your reference for structure and functionality, but write your own code.

--- START OF FILE: artificial-language/artificial-backend-rust/src/codegen.rs ---
```rust
// artificial-language/artificial-backend-rust/src/codegen.rs
use artificial_core::IrModule;
use artificial_core::IrStatement;
use artificial_core::IrExpression;

pub struct RustCodegen;

impl RustCodegen {
    pub fn generate(module: &IrModule) -> String {
        let mut code = String::new();

        // Header
        code.push_str("// Generated by ALC\n");
        code.push_str("// DO NOT EDIT\n\n");

        // Main function
        code.push_str("fn main() {\n");

        // Statements
        for stmt in &module.statements {
            Self::generate_statement(&mut code, stmt);
        }

        code.push_str("}\n");

        code
    }

    fn generate_statement(code: &mut String, stmt: &IrStatement) {
        match stmt {
            IrStatement::Accrete(expr) => {
                code.push_str("    println!(");
                Self::generate_expression(code, expr);
                code.push_str(");\n");
            }
        }
    }

    fn generate_expression(code: &mut String, expr: &IrExpression) {
        match expr {
            IrExpression::StringLiteral(s) => {
                let escaped = Self::escape_string(s);
                code.push('"');
                code.push_str(&escaped);
                code.push('"');
            }
        }
    }

    fn escape_string(s: &str) -> String {
        let mut escaped = String::new();
        for ch in s.chars() {
            match ch {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                '\n' => escaped.push_str("\\n"),
                '\t' => escaped.push_str("\\t"),
                '\r' => escaped.push_str("\\r"),
                _ => escaped.push(ch),
            }
        }
        escaped
    }
}
```
--- END OF FILE: artificial-language/artificial-backend-rust/src/codegen.rs ---

--- START OF FILE: artificial-language/artificial-backend-rust/src/emitter.rs ---
```rust
// artificial-language/artificial-backend-rust/src/emitter.rs
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn write_rust_file(out_dir: &Path, file_stem: &str, code: &str) -> io::Result<PathBuf> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir)?;

    // Build file path
    let file_path = out_dir.join(format!("{}_art.rs", file_stem));

    // Write the code to file
    fs::write(&file_path, code)?;

    Ok(file_path)
}
```
--- END OF FILE: artificial-language/artificial-backend-rust/src/emitter.rs ---

--- START OF FILE: artificial-language/artificial-backend-rust/src/invoke_rustc.rs ---
```rust
// artificial-language/artificial-backend-rust/src/invoke_rustc.rs
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn compile(rs_file: &Path, out_dir: &Path, file_stem: &str) -> io::Result<PathBuf> {
    // Determine the binary path
    let binary_path = if cfg!(windows) {
        out_dir.join(format!("{}_art.exe", file_stem))
    } else {
        out_dir.join(format!("{}_art", file_stem))
    };

    // Run rustc
    let output = Command::new("rustc")
        .arg(rs_file)
        .arg("-o")
        .arg(&binary_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("rustc compilation failed:\n{}", stderr),
        ));
    }

    Ok(binary_path)
}
```
--- END OF FILE: artificial-language/artificial-backend-rust/src/invoke_rustc.rs ---

--- START OF FILE: artificial-language/artificial-backend-rust/src/lib.rs ---
```rust
// artificial-language/artificial-backend-rust/src/lib.rs
pub mod codegen;
pub mod emitter;
pub mod invoke_rustc;

pub use codegen::RustCodegen;
pub use emitter::write_rust_file;
pub use invoke_rustc::compile;

use artificial_core::IrModule;
use std::io;
use std::path::{Path, PathBuf};

pub fn compile_to_binary(
    module: &IrModule,
    out_dir: &Path,
    file_stem: &str,
) -> io::Result<PathBuf> {
    // Generate Rust code
    let rust_code = RustCodegen::generate(module);

    // Write to file
    let rs_file = write_rust_file(out_dir, file_stem, &rust_code)?;

    // Compile with rustc
    let binary_path = compile(&rs_file, out_dir, file_stem)?;

    Ok(binary_path)
}
```
--- END OF FILE: artificial-language/artificial-backend-rust/src/lib.rs ---

--- START OF FILE: artificial-language/artificial-core/src/ast.rs ---
```rust
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
        write!(f, "AstModule {{\n")?;
        for stmt in &self.statements {
            write!(f, "  {:?}\n", stmt)?;
        }
        write!(f, "}}")
    }
}
```
--- END OF FILE: artificial-language/artificial-core/src/ast.rs ---

--- START OF FILE: artificial-language/artificial-core/src/ir.rs ---
```rust
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
        write!(f, "IrModule {{\n")?;
        for stmt in &self.statements {
            write!(f, "  {:?}\n", stmt)?;
        }
        write!(f, "}}")
    }
}
```
--- END OF FILE: artificial-language/artificial-core/src/ir.rs ---

--- START OF FILE: artificial-language/artificial-core/src/lexer.rs ---
```rust
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
                    return Err(LexError { message: "Unterminated string literal".to_string(), line: start_line, column: start_col });
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
            return Err(LexError { message: "Unterminated string literal".to_string(), line: start_line, column: start_col });
        }

        self.advance(); // Consume closing quote
        Ok(string)
    }
}```
--- END OF FILE: artificial-language/artificial-core/src/lexer.rs ---

--- START OF FILE: artificial-language/artificial-core/src/lib.rs ---
```rust
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
```
--- END OF FILE: artificial-language/artificial-core/src/lib.rs ---

--- START OF FILE: artificial-language/artificial-core/src/lowering.rs ---
```rust
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
```
--- END OF FILE: artificial-language/artificial-core/src/lowering.rs ---

--- START OF FILE: artificial-language/artificial-core/src/parser.rs ---
```rust
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
        match self.current_token().kind {
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
        self.tokens.get(self.position).unwrap_or_else(|| &self.tokens[self.tokens.len() - 1])
    }

    fn is_at_eof(&self) -> bool {
        matches!(self.current_token().kind, TokenKind::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_eof() {
            self.position += 1;
        }
        self.tokens.get(self.position - 1).unwrap()
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
```
--- END OF FILE: artificial-language/artificial-core/src/parser.rs ---

--- START OF FILE: artificial-language/ALC/src/cli.rs ---
```rust
// artificial-language/ALC/src/cli.rs
pub struct CliOptions {
    pub source_file: String,
    pub emit_rs: bool,
    pub no_run: bool,
    pub out_dir: String,
    pub debug: bool,
}

impl Default for CliOptions {
    fn default() -> Self {
        CliOptions {
            source_file: String::new(),
            emit_rs: false,
            no_run: false,
            out_dir: "target/artificial_out".to_string(),
            debug: false,
        }
    }
}

pub fn parse_cli_args() -> Result<CliOptions, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = CliOptions::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--emit-rs" => { opts.emit_rs = true; i += 1; }
            "--no-run" => { opts.no_run = true; i += 1; }
            "--debug" => { opts.debug = true; i += 1; }
            "--out-dir" => {
                i += 1;
                if i >= args.len() { return Err("--out-dir requires an argument".to_string()); }
                opts.out_dir = args[i].clone();
                i += 1;
            }
            arg if !arg.starts_with("--") => {
                if opts.source_file.is_empty() { opts.source_file = arg.to_string(); }
                else { return Err(format!("Unexpected positional argument: {}", arg)); }
                i += 1;
            }
            arg => { return Err(format!("Unknown option: {}", arg)); }
        }
    }

    if opts.source_file.is_empty() { return Err("No source file specified".to_string()); }
    Ok(opts)
}
```
--- END OF FILE: artificial-language/ALC/src/cli.rs ---

--- START OF FILE: artificial-language/ALC/src/hooks.rs ---
```rust
// artificial-language/ALC/src/hooks.rs
use std::path::Path;
use std::process::Command;

pub fn run_hook(hook_path: &str) {
    if Path::new(hook_path).exists() {
        let _ = Command::new("sh")
            .arg(hook_path)
            .status();
    }
}
```
--- END OF FILE: artificial-language/ALC/src/hooks.rs ---

--- START OF FILE: artificial-language/ALC/src/main.rs ---
```rust
// artificial-language/ALC/src/main.rs
mod cli;
mod hooks;
mod runner;

use cli::parse_cli_args;
use runner::run_compiler;

fn main() {
    let opts = match parse_cli_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: ALC [OPTIONS] <source-file>");
            std::process::exit(1);
        }
    };

    if let Err(e) = run_compiler(&opts) {
        eprintln!("Compiler Error: {}", e);
        std::process::exit(1);
    }
}
```
--- END OF FILE: artificial-language/ALC/src/main.rs ---

--- START OF FILE: artificial-language/ALC/src/runner.rs ---
```rust
// artificial-language/ALC/src/runner.rs
use crate::cli::CliOptions;
use crate::hooks::run_hook;
use artificial_core::{Lexer, Parser, lower_module};
use artificial_backend_rust::compile_to_binary;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run_compiler(opts: &CliOptions) -> Result<(), String> {
    let source = fs::read_to_string(&opts.source_file)
        .map_err(|e| format!("Failed to read source file '{}': {}", &opts.source_file, e))?;

    let lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_module().map_err(|e| e.to_string())?;

    if opts.debug { println!("AST = {:#?}", ast); }

    let ir = lower_module(ast);
    if opts.debug { println!("IR = {:#?}", ir); }

    let out_dir = Path::new(&opts.out_dir);
    let file_stem = Path::new(&opts.source_file).file_stem().unwrap().to_str().unwrap();

    run_hook("scripts/pre_build.sh");

    let binary_path = compile_to_binary(&ir, out_dir, file_stem)
        .map_err(|e| format!("Backend compilation failed: {}", e))?;

    run_hook("scripts/post_build.sh");

    if opts.emit_rs || opts.no_run { return Ok(()); }

    let output = Command::new(&binary_path)
        .output()
        .map_err(|e| format!("Failed to execute binary: {}", e))?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(format!("Binary exited with error code: {}", output.status));
    }

    run_hook("scripts/post_run.sh");

    Ok(())
}
```
--- END OF FILE: artificial-language/ALC/src/runner.rs ---

--- START OF FILE: artificial-language/Cargo.toml ---
```toml
// artificial-language/Cargo.toml
[workspace]
resolver = "2"
members = [
    "artificial-core",
    "artificial-backend-rust",
    "ALC"
]

[workspace.package]
version = "0.1.0"
edition = "2021"
```
--- END OF FILE: artificial-language/Cargo.toml ---

--- START OF FILE: artificial-language/artificial-backend-rust/Cargo.toml ---
```toml
// artificial-language/artificial-backend-rust/Cargo.toml
[package]
name = "artificial-backend-rust"
version.workspace = true
edition.workspace = true

[dependencies]
artificial-core = { path = "../artificial-core" }
```
--- END OF FILE: artificial-language/artificial-backend-rust/Cargo.toml ---

--- START OF FILE: artificial-language/artificial-core/Cargo.toml ---
```toml
// artificial-language/artificial-core/Cargo.toml
[package]
name = "artificial-core"
version.workspace = true
edition.workspace = true

[dependencies]
```
--- END OF FILE: artificial-language/artificial-core/Cargo.toml ---

--- START OF FILE: artificial-language/ALC/Cargo.toml ---
```toml
// artificial-language/ALC/Cargo.toml
[package]
name = "ALC"
version.workspace = true
edition.workspace = true

[[bin]]
name = "ALC"
path = "src/main.rs"

[dependencies]
artificial-core = { path = "../artificial-core" }
artificial-backend-rust = { path = "../artificial-backend-rust" }
```
--- END OF FILE: artificial-language/ALC/Cargo.toml ---

--- START OF FILE: artificial-language/main.art ---
```text
// artificial-language/main.art
accrete "Hello Artificial World!"
```
--- END OF FILE: artificial-language/main.art ---

---

### Appendix: Development Best Practices & Quality Mandates

You must adhere to the following best practices throughout the entire development process.

#### **Error Prevention Checklist**

1.  **Workspace Configuration**
    *   ✅ Always specify `resolver = "2"` in the workspace `Cargo.toml`.
    *   Prevents: `"virtual workspace defaulting to resolver = 1"` warnings.

2.  **.gitignore Setup**
    *   ✅ Create proper `.gitignore` files in the root and in each crate.
    *   ✅ Include project-specific artifacts like `target/artificial_out/`.
    *   Prevents: Accidentally committing build artifacts and generated files.

3.  **Unused Code Cleanup**
    *   ✅ Immediately remove any unused imports.
    *   ✅ Prefix all unused variables with an underscore (`_`).
    *   ✅ Use `#[allow(dead_code)]` for intentionally unused functions (e.g., helpers for future debugging).
    *   Prevents: A cluttered codebase and compiler warnings.

4.  **Crate Dependencies**
    *   ✅ Always use `path` dependencies for local workspace crates.
    *   ✅ Use workspace-level metadata for version and edition.
    *   Prevents: Broken module imports and versioning conflicts.

5.  **Code Organization**
    *   ✅ Organize each crate into logical modules (e.g., `lexer.rs`, `parser.rs`).
    *   ✅ Use `lib.rs` as the crate entry point to declare modules and re-export public APIs.
    *   Prevents: Module-not-found errors and unmanageable file sizes.

6.  **Error Handling**
    *   ✅ Define distinct, clear error types for each compilation phase (e.g., `LexError`, `ParseError`).
    *   ✅ Always include location information (line, column) in errors.
    *   ✅ Chain errors with context: `map_err(|e| format!("Parsing failed: {}", e))`.
    *   Prevents: Cryptic, unhelpful error messages during debugging.

7.  **Testing During Development**
    *   ✅ Build frequently with `cargo build`.
    *   ✅ Continuously check for warnings and treat them as errors.
    *   ✅ Run the full compiler pipeline on test files often.
    *   Prevents: Discovering deep-rooted issues late in the process.

8.  **Frontend (Lexer/Parser) Development**
    *   ✅ Ensure the lexer correctly handles all required tokens and provides positional data.
    *   ✅ Implement the parser with comprehensive error messages for unexpected tokens.
    *   Prevents: Parser errors that are actually caused by lexer bugs.

9.  **Backend Development**
    *   ✅ Strictly separate concerns: Codegen (IR -> String), Emitter (String -> File), Invoker (File -> Binary).
    *   ✅ Ensure all generated code is syntactically valid before attempting to compile it.
    *   Prevents: Bugs that are difficult to trace between code generation and compilation.

10. **CLI Development**
    *   ✅ Provide clear usage instructions if arguments are parsed incorrectly.
    *   ✅ Handle all error cases and exit with a non-zero status code to signal failure.
    *   Prevents: A confusing and frustrating user experience.

#### **Common Pitfalls to Avoid**

| Issue                         | Solution                                           |
|-------------------------------|----------------------------------------------------|
| Unused imports causing warnings | Remove them immediately or use `#[allow(unused_imports)]` sparingly. |
| Module not found errors       | Declare the module (`mod my_module;`) in its parent (`lib.rs` or `main.rs`). |
| Wrong Cargo.toml resolver     | Add `resolver = "2"` to the `[workspace]` table.     |
| Generated files in git        | Ensure `.gitignore` patterns correctly cover output directories. |
| Build caching issues          | Run `cargo clean` before rebuilding if you suspect stale artifacts. |
| Rustc invocation failing      | Capture and display `stderr` from the `rustc` command for debugging. |

#### **Final Verification Checklist**

- [ ] `cargo build` succeeds with **zero warnings**.
- [ ] `cargo clippy` produces **no warnings**.
- [ ] Generated code is valid and compiles cleanly.
- [ ] Error messages are clear and actionable.
- [ ] `.gitignore` files are correctly configured and placed.
- [ ] All unused code has been removed or properly marked.
- [ ] The workspace `resolver` is set to `"2"`.
- [ ] The final success criteria are met exactly.
