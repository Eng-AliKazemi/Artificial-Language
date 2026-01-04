## **MASTER PROMPT: AUTONOMOUS ARTIFICIAL LANGUAGE COMPILER WITH VIRTUAL MACHINE**

**Your Role:** You are an autonomous, senior-level Rust engineer and language tools developer.

**Your Mission:** Your sole task is to create a new compiler project for a new **programming language: Artificial Language** in Rust based on standard compiler development architecture. This compiler will include a **custom bytecode format** and a **stack-based Virtual Machine (VM)** to execute the compiled bytecode. You will implement the entire project from end-to-end with **no human intervention or code edits**. The ultimate goal is to use this compiler to produce a single, standalone executable file named `Artificial-Language` that, when executed, prints "Hello Artificial World!" to the console. You will perform a complete, automated workflow: plan the project, create every file, write all the code, run all commands, and perform all debugging and iteration until the project is fully functional, robust, and completely free of any warnings.

---

### **Part 1: Core Directives & Project Plan**

#### **1.1. Overarching Principle: Autonomy and Expertise**

This prompt contains a complete, working implementation of the compiler, including all source code files, configuration files, and test cases. You must treat this as a **functional reference and a structural guide**. It demonstrates a valid path to success and defines the required project architecture (the workspace, the three crates, and the compilation pipeline).

However, you are a **senior engineer**. Your primary directive is to use your own expert knowledge to generate all code. **You must not simply copy the provided code verbatim.** Instead, you will write your own mature, production-ready, clean, and extensible implementations that follow modern Rust idioms and best practices. If you have a better, more efficient, or cleaner approach that still meets the project's goals, you are required to use it. The fundamental project structure must be preserved.

You must perform this entire task automatically and without asking the user for assistance. Analyze the following steps, use the provided examples as a reference for the goal, and execute the plan from start to finish.

#### **1.2. Compilation Pipeline Overview**

The compiler follows this pipeline:

```
Source Code (.art)
       ↓
    [Lexer]         → Tokens
       ↓
    [Parser]        → AST (Abstract Syntax Tree)
       ↓
    [Lowering]      → IR (Intermediate Representation)
       ↓
    [Bytecode       → Bytecode Module (.artb)
     Compiler]
       ↓
    [Virtual        → Execution Output
     Machine]
```

#### **1.3. Global Rules & Quality Mandates (Non-Negotiable)**

1.  **Zero Warnings Policy**: You must treat **all** compiler warnings as errors. After every `cargo build`, `cargo check`, or `cargo clippy` command, you will meticulously inspect the output. If **any** warning appears, you must immediately **stop**, modify the code to resolve the warning, and re-run the build command. This cycle must be repeated until the build is perfectly clean.
2.  **Mandatory `.gitignore` Files**: You must create four separate `.gitignore` files as one of the first steps. The paths and content are specified precisely in Step 1.7 below.
3.  **No Human Interaction**: You must not ask the user to edit files, run commands, or debug failures. You are responsible for the entire lifecycle.
4.  **File Path Comments**: When generating Rust code for any `.rs` file, you must **always** place a comment at the very top with the full file path (e.g., `// artificial-language/ALC/src/main.rs`).
5.  **Strict Adherence to Best Practices**: You must follow all guidelines outlined in the **Appendix** section of this prompt.

#### **1.4. Final Success Criteria**

The project is considered complete only when **all** of the following criteria are met in order:

1.  The `ALC` compiler successfully compiles the `main.art` source file into bytecode.
2.  The bytecode is executed by the Virtual Machine embedded in `ALC`.
3.  A standalone executable binary is created that embeds both the VM runtime and the compiled bytecode.
4.  This final, compiled binary is programmatically copied from its build output directory to the project's root directory (`artificial-language/`).
5.  The copied binary is renamed to `Artificial-Language`.
6.  The console is cleared via the `clear` command.
7.  Immediately following the `clear` command, the `Artificial-Language` binary is executed directly from the project root with the command `./Artificial-Language`.
8.  The execution produces the **exact** text `Hello Artificial World!` as the **only** output visible on the cleared console.

#### **1.5. Project Structure**

```
artificial-language/
├── Cargo.toml                    # Workspace root
├── main.art                      # Sample source file
├── .gitignore
│
├── artificial-core/              # Frontend: Lexer, Parser, AST, IR
│   ├── Cargo.toml
│   ├── .gitignore
│   └── src/
│       ├── lib.rs
│       ├── ast.rs
│       ├── ir.rs
│       ├── lexer.rs
│       ├── parser.rs
│       └── lowering.rs
│
├── artificial-vm/                # Backend: Bytecode & Virtual Machine
│   ├── Cargo.toml
│   ├── .gitignore
│   └── src/
│       ├── lib.rs
│       ├── bytecode.rs           # Opcode & Bytecode definitions
│       ├── compiler.rs           # IR → Bytecode compiler
│       ├── vm.rs                 # Stack-based Virtual Machine
│       └── serializer.rs         # Bytecode serialization/deserialization
│
└── ALC/                          # CLI & Compiler Driver
    ├── Cargo.toml
    ├── .gitignore
    └── src/
        ├── main.rs
        ├── cli.rs
        ├── runner.rs
        └── bundler.rs            # Creates standalone executable
```

#### **1.6. Step-by-Step Implementation Plan**

**Step 1 — Initialize the Workspace and Git Repository**

1.1. Create the root project directory: `mkdir artificial-language`
1.2. Change into the new directory: `cd artificial-language`
1.3. **Initialize a single Git repository for the entire workspace:** `git init`
1.4. Create the root `Cargo.toml` file with the following content:
```toml
# artificial-language/Cargo.toml
[workspace]
resolver = "2"
members = ["artificial-core", "artificial-vm", "ALC"]

[workspace.package]
version = "0.1.0"
edition = "2021"
```
1.5. Initialize the three member crates **without creating nested Git repositories**:
```bash
cargo new artificial-core --lib --vcs none
cargo new artificial-vm --lib --vcs none
cargo new ALC --bin --vcs none
```

1.6. Create the `main.art` file in the project root with the following content:
```text
// artificial-language/main.art
accrete "Hello Artificial World!"
```

1.7. **Create four `.gitignore` files** in the specified locations. Each file must contain the content below:
    *   `./.gitignore` (at the workspace root)
    *   `./artificial-core/.gitignore`
    *   `./artificial-vm/.gitignore`
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
**/*.artb
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

**Step 6 — Define Bytecode Format in `artificial-vm`**

- 6.1. Add the dependency on `artificial-core` to `artificial-vm/Cargo.toml`.
- 6.2. In `artificial-vm/src/`, create `bytecode.rs`.
- 6.3. Define the `Opcode` enum with the following opcodes:
  - `Nop` (0x00) - No operation
  - `Halt` (0x01) - Stop execution
  - `PushConst` (0x02) - Push constant from pool onto stack
  - `Print` (0x03) - Pop value from stack and print to stdout
  - `PrintLn` (0x04) - Pop value from stack and print with newline
- 6.4. Define the `Value` enum to represent runtime values (start with `StringVal(String)`).
- 6.5. Define the `BytecodeModule` struct containing:
  - `constants: Vec<Value>` - Constant pool
  - `instructions: Vec<u8>` - Raw bytecode bytes
- 6.6. Run `cargo build` and fix **all** warnings.

**Step 7 — Implement Bytecode Compiler in `artificial-vm`**

- 7.1. In `artificial-vm/src/`, create `compiler.rs`.
- 7.2. Implement a `BytecodeCompiler` struct that traverses the `IrModule`.
- 7.3. Implement the compilation logic:
  - For `IrStatement::Accrete(expr)`:
    - Add the string to the constant pool, get its index
    - Emit `PushConst <index>` instruction
    - Emit `PrintLn` instruction
  - At the end, emit `Halt` instruction
- 7.4. The compiler should produce a `BytecodeModule`.
- 7.5. Run `cargo build` and fix **all** warnings.

**Step 8 — Implement the Virtual Machine in `artificial-vm`**

- 8.1. In `artificial-vm/src/`, create `vm.rs`.
- 8.2. Implement a `VirtualMachine` struct with:
  - `stack: Vec<Value>` - Operand stack
  - `ip: usize` - Instruction pointer
  - `output: String` - Captured output (for testing/bundling)
- 8.3. Implement the `execute` method that runs a `BytecodeModule`:
  - Fetch-decode-execute loop
  - Handle each opcode appropriately
  - `PushConst`: Push constant from pool onto stack
  - `PrintLn`: Pop value, append to output with newline
  - `Halt`: Stop execution
- 8.4. Implement proper error handling with a `VmError` type.
- 8.5. Run `cargo build` and fix **all** warnings.

**Step 9 — Implement Bytecode Serialization in `artificial-vm`**

- 9.1. In `artificial-vm/src/`, create `serializer.rs`.
- 9.2. Implement `serialize` function to convert `BytecodeModule` to `Vec<u8>`.
- 9.3. Implement `deserialize` function to convert `Vec<u8>` back to `BytecodeModule`.
- 9.4. Use a simple binary format:
  - Magic bytes: `0x41 0x52 0x54 0x42` ("ARTB")
  - Version: `u8`
  - Constant pool length: `u32`
  - For each constant: type tag + length + data
  - Instructions length: `u32`
  - Raw instruction bytes
- 9.5. Update `artificial-vm/src/lib.rs` to expose all public APIs.
- 9.6. Run `cargo build` and fix **all** warnings.

**Step 10 — Implement the CLI in `ALC`**

- 10.1. Add dependencies on `artificial-core` and `artificial-vm` in `ALC/Cargo.toml`.
- 10.2. In `ALC/src/`, create `cli.rs` to parse command-line arguments using only the standard library. It must handle:
  - A single positional argument for the source file path
  - `--emit-bytecode` flag to save bytecode to file
  - `--no-run` flag to compile without executing
  - `--bundle` flag to create standalone executable
  - `--out-dir` option for output directory
  - `--debug` flag for verbose output
- 10.3. Run `cargo build` and fix **all** warnings.

**Step 11 — Implement the Compiler Runner in `ALC`**

- 11.1. In `ALC/src/`, create `runner.rs`.
- 11.2. Implement the main `run_compiler` function that chains the entire process:
  1. Read source file
  2. Lex → tokens
  3. Parse → AST
  4. Lower → IR
  5. Compile → Bytecode
  6. (Optional) Serialize and save bytecode to `.artb` file
  7. Execute bytecode in VM (unless `--no-run`)
- 11.3. Update `ALC/src/main.rs` to call the CLI parser and the runner, with robust error handling.
- 11.4. Run `cargo build` and fix **all** warnings.

**Step 12 — Implement the Standalone Bundler in `ALC`**

- 12.1. In `ALC/src/`, create `bundler.rs`.
- 12.2. Implement the `create_standalone` function that:
  - Takes the compiled `BytecodeModule`
  - Serializes it to bytes
  - Generates a minimal Rust source file that:
    - Embeds the bytecode as a `const` byte array
    - Includes a minimal VM implementation (or uses the full VM via include)
    - Has a `main()` function that deserializes and executes the bytecode
  - Compiles the generated Rust file using `rustc`
  - Outputs the standalone binary
- 12.3. Run `cargo build` and fix **all** warnings.

**Step 13 — End-to-End Verification and Self-Correction**

- 13.1. This is the primary debugging and verification loop.
- 13.2. Run the compiler in debug/development mode to compile and immediately execute its output: `cargo run -p ALC -- main.art`.
- 13.3. **Verify the output.** If it is not exactly `Hello Artificial World!`, you must diagnose the logic error within your implementation, modify the relevant source file(s), rebuild, and re-test.
- 13.4. Repeat this self-correction loop until `cargo run -p ALC -- main.art` works perfectly and there are zero warnings from `cargo build` and `cargo clippy`.

**Step 14 — Final Product Generation and Deployment**

- 14.1. **After and only after Step 13 is successfully completed**, you will perform the final product generation and deployment.
- 14.2. Execute the `ALC` compiler with the `--bundle` flag to create a standalone executable:
  ```bash
  cargo run -p ALC --release -- main.art --bundle --no-run
  ```
- 14.3. The bundler should output the standalone binary to `target/artificial_out/main_art` (or similar).
- 14.4. Programmatically copy this binary to the project's root directory (`./`).
- 14.5. The destination path for the copy operation must be `./Artificial-Language`.
- 14.6. **(For non-Windows systems)** Programmatically make the final binary executable: `chmod +x ./Artificial-Language`.
- **The final two steps must be executed in sequence to meet the success criteria.**
- 14.7. First, clear all previous console history to ensure a clean final output: `clear`.
- 14.8. Second, execute the deployed binary to display the final output on the clean console: `./Artificial-Language`.

---

### **Part 2: Functional Code Examples & Reference**

This section contains the complete source for a working version of the project. Use it as your reference for structure and functionality, but write your own code.

--- START OF FILE: artificial-language/Cargo.toml ---
```toml
# artificial-language/Cargo.toml
[workspace]
resolver = "2"
members = [
    "artificial-core",
    "artificial-vm",
    "ALC"
]

[workspace.package]
version = "0.1.0"
edition = "2021"
```
--- END OF FILE: artificial-language/Cargo.toml ---

--- START OF FILE: artificial-language/artificial-core/Cargo.toml ---
```toml
# artificial-language/artificial-core/Cargo.toml
[package]
name = "artificial-core"
version.workspace = true
edition.workspace = true

[dependencies]
```
--- END OF FILE: artificial-language/artificial-core/Cargo.toml ---

--- START OF FILE: artificial-language/artificial-vm/Cargo.toml ---
```toml
# artificial-language/artificial-vm/Cargo.toml
[package]
name = "artificial-vm"
version.workspace = true
edition.workspace = true

[dependencies]
artificial-core = { path = "../artificial-core" }
```
--- END OF FILE: artificial-language/artificial-vm/Cargo.toml ---

--- START OF FILE: artificial-language/ALC/Cargo.toml ---
```toml
# artificial-language/ALC/Cargo.toml
[package]
name = "ALC"
version.workspace = true
edition.workspace = true

[[bin]]
name = "ALC"
path = "src/main.rs"

[dependencies]
artificial-core = { path = "../artificial-core" }
artificial-vm = { path = "../artificial-vm" }
```
--- END OF FILE: artificial-language/ALC/Cargo.toml ---

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
            self.skip_whitespace_and_comments();

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

    fn peek_char(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.position + 1]
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

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_char() == '/' {
                // Line comment
                while !self.is_at_end() && self.current_char() != '\n' {
                    self.advance();
                }
            } else {
                break;
            }
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
```
--- END OF FILE: artificial-language/artificial-core/src/lexer.rs ---

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
        // Semicolon is optional
        if self.check(&TokenKind::Semicolon) {
            self.consume(&TokenKind::Semicolon)?;
        }
        Ok(AstStatement::Accrete(expr))
    }

    fn parse_expression(&mut self) -> Result<AstExpression, ParseError> {
        let token = self.current_token().clone();
        match &token.kind {
            TokenKind::StringLiteral(s) => {
                let string = s.clone();
                self.advance();
                Ok(AstExpression::StringLiteral(string))
            }
            _ => Err(ParseError {
                message: format!("Expected expression, found {:?}", token.kind),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens
            .get(self.position)
            .unwrap_or_else(|| self.tokens.last().unwrap())
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
```
--- END OF FILE: artificial-language/artificial-core/src/parser.rs ---

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

--- START OF FILE: artificial-language/artificial-vm/src/lib.rs ---
```rust
// artificial-language/artificial-vm/src/lib.rs
pub mod bytecode;
pub mod compiler;
pub mod vm;
pub mod serializer;

pub use bytecode::*;
pub use compiler::*;
pub use vm::*;
pub use serializer::*;
```
--- END OF FILE: artificial-language/artificial-vm/src/lib.rs ---

--- START OF FILE: artificial-language/artificial-vm/src/bytecode.rs ---
```rust
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
```
--- END OF FILE: artificial-language/artificial-vm/src/bytecode.rs ---

--- START OF FILE: artificial-language/artificial-vm/src/compiler.rs ---
```rust
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
```
--- END OF FILE: artificial-language/artificial-vm/src/compiler.rs ---

--- START OF FILE: artificial-language/artificial-vm/src/vm.rs ---
```rust
// artificial-language/artificial-vm/src/vm.rs
use crate::bytecode::{BytecodeModule, Opcode, Value};
use std::fmt;
use std::io::{self, Write};

/// Virtual Machine execution errors
#[derive(Debug)]
pub struct VmError {
    pub message: String,
    pub ip: usize,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VM Error at instruction {}: {}", self.ip, self.message)
    }
}

impl std::error::Error for VmError {}

/// Stack-based Virtual Machine for Artificial Language
pub struct VirtualMachine {
    /// Operand stack
    stack: Vec<Value>,
    /// Instruction pointer
    ip: usize,
    /// Captured output (used for bundled execution)
    output: String,
    /// Whether to capture output or print directly
    capture_output: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            stack: Vec::with_capacity(256),
            ip: 0,
            output: String::new(),
            capture_output: false,
        }
    }

    /// Create a VM that captures output instead of printing
    pub fn with_captured_output() -> Self {
        VirtualMachine {
            stack: Vec::with_capacity(256),
            ip: 0,
            output: String::new(),
            capture_output: true,
        }
    }

    /// Execute a bytecode module
    pub fn execute(&mut self, module: &BytecodeModule) -> Result<(), VmError> {
        self.ip = 0;
        self.stack.clear();
        self.output.clear();

        while self.ip < module.instructions.len() {
            let opcode_byte = module.instructions[self.ip];
            let opcode = Opcode::from_u8(opcode_byte).ok_or_else(|| VmError {
                message: format!("Unknown opcode: 0x{:02X}", opcode_byte),
                ip: self.ip,
            })?;

            match opcode {
                Opcode::Nop => {
                    self.ip += 1;
                }

                Opcode::Halt => {
                    break;
                }

                Opcode::PushConst => {
                    let index = self.read_u16(module)?;
                    let value = module.constants.get(index as usize).ok_or_else(|| VmError {
                        message: format!("Constant index {} out of bounds", index),
                        ip: self.ip - 3,
                    })?;
                    self.stack.push(value.clone());
                }

                Opcode::Print => {
                    let value = self.pop()?;
                    self.print_value(&value, false)?;
                    self.ip += 1;
                }

                Opcode::PrintLn => {
                    let value = self.pop()?;
                    self.print_value(&value, true)?;
                    self.ip += 1;
                }

                Opcode::Pop => {
                    self.pop()?;
                    self.ip += 1;
                }
            }
        }

        Ok(())
    }

    /// Get captured output
    pub fn get_output(&self) -> &str {
        &self.output
    }

    fn read_u16(&mut self, module: &BytecodeModule) -> Result<u16, VmError> {
        if self.ip + 2 >= module.instructions.len() {
            return Err(VmError {
                message: "Unexpected end of bytecode while reading operand".to_string(),
                ip: self.ip,
            });
        }
        self.ip += 1;
        let high = module.instructions[self.ip] as u16;
        self.ip += 1;
        let low = module.instructions[self.ip] as u16;
        self.ip += 1;
        Ok((high << 8) | low)
    }

    fn pop(&mut self) -> Result<Value, VmError> {
        self.stack.pop().ok_or_else(|| VmError {
            message: "Stack underflow".to_string(),
            ip: self.ip,
        })
    }

    fn print_value(&mut self, value: &Value, newline: bool) -> Result<(), VmError> {
        let text = value.to_string();

        if self.capture_output {
            self.output.push_str(&text);
            if newline {
                self.output.push('\n');
            }
        } else {
            if newline {
                println!("{}", text);
            } else {
                print!("{}", text);
            }
            io::stdout().flush().ok();
        }

        Ok(())
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
```
--- END OF FILE: artificial-language/artificial-vm/src/vm.rs ---

--- START OF FILE: artificial-language/artificial-vm/src/serializer.rs ---
```rust
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
```
--- END OF FILE: artificial-language/artificial-vm/src/serializer.rs ---

--- START OF FILE: artificial-language/ALC/src/main.rs ---
```rust
// artificial-language/ALC/src/main.rs
mod cli;
mod runner;
mod bundler;

use cli::parse_cli_args;
use runner::run_compiler;

fn main() {
    let opts = match parse_cli_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: ALC [OPTIONS] <source-file>");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  --emit-bytecode    Save bytecode to .artb file");
            eprintln!("  --no-run           Compile without executing");
            eprintln!("  --bundle           Create standalone executable");
            eprintln!("  --out-dir <DIR>    Output directory (default: target/artificial_out)");
            eprintln!("  --debug            Enable debug output");
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

--- START OF FILE: artificial-language/ALC/src/cli.rs ---
```rust
// artificial-language/ALC/src/cli.rs

pub struct CliOptions {
    pub source_file: String,
    pub emit_bytecode: bool,
    pub no_run: bool,
    pub bundle: bool,
    pub out_dir: String,
    pub debug: bool,
}

impl Default for CliOptions {
    fn default() -> Self {
        CliOptions {
            source_file: String::new(),
            emit_bytecode: false,
            no_run: false,
            bundle: false,
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
            "--emit-bytecode" => {
                opts.emit_bytecode = true;
                i += 1;
            }
            "--no-run" => {
                opts.no_run = true;
                i += 1;
            }
            "--bundle" => {
                opts.bundle = true;
                i += 1;
            }
            "--debug" => {
                opts.debug = true;
                i += 1;
            }
            "--out-dir" => {
                i += 1;
                if i >= args.len() {
                    return Err("--out-dir requires an argument".to_string());
                }
                opts.out_dir = args[i].clone();
                i += 1;
            }
            arg if !arg.starts_with("--") => {
                if opts.source_file.is_empty() {
                    opts.source_file = arg.to_string();
                } else {
                    return Err(format!("Unexpected positional argument: {}", arg));
                }
                i += 1;
            }
            arg => {
                return Err(format!("Unknown option: {}", arg));
            }
        }
    }

    if opts.source_file.is_empty() {
        return Err("No source file specified".to_string());
    }

    Ok(opts)
}
```
--- END OF FILE: artificial-language/ALC/src/cli.rs ---

--- START OF FILE: artificial-language/ALC/src/runner.rs ---
```rust
// artificial-language/ALC/src/runner.rs
use crate::bundler;
use crate::cli::CliOptions;
use artificial_core::{lower_module, Lexer, Parser};
use artificial_vm::{BytecodeCompiler, VirtualMachine, serialize, write_to_file};
use std::fs;
use std::path::Path;

pub fn run_compiler(opts: &CliOptions) -> Result<(), String> {
    // Read source file
    let source = fs::read_to_string(&opts.source_file)
        .map_err(|e| format!("Failed to read source file '{}': {}", &opts.source_file, e))?;

    // Lexical analysis
    let lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    if opts.debug {
        println!("[DEBUG] Tokens: {:#?}", tokens);
    }

    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_module().map_err(|e| e.to_string())?;

    if opts.debug {
        println!("[DEBUG] AST: {:#?}", ast);
    }

    // Lowering to IR
    let ir = lower_module(ast);

    if opts.debug {
        println!("[DEBUG] IR: {:#?}", ir);
    }

    // Compile to bytecode
    let compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile(&ir);

    if opts.debug {
        println!("[DEBUG] Bytecode constants: {:?}", bytecode.constants);
        println!("[DEBUG] Bytecode instructions: {:?}", bytecode.instructions);
    }

    // Ensure output directory exists
    let out_dir = Path::new(&opts.out_dir);
    fs::create_dir_all(out_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Get file stem for output naming
    let file_stem = Path::new(&opts.source_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // Optionally emit bytecode to file
    if opts.emit_bytecode || opts.bundle {
        let bytecode_path = out_dir.join(format!("{}.artb", file_stem));
        write_to_file(&bytecode, &bytecode_path)
            .map_err(|e| format!("Failed to write bytecode file: {}", e))?;

        if opts.debug {
            println!("[DEBUG] Bytecode written to: {:?}", bytecode_path);
        }
    }

    // Create standalone bundle if requested
    if opts.bundle {
        let binary_path = bundler::create_standalone(&bytecode, out_dir, file_stem)?;

        if opts.debug {
            println!("[DEBUG] Standalone binary created at: {:?}", binary_path);
        }
    }

    // Execute unless --no-run is specified
    if !opts.no_run && !opts.bundle {
        let mut vm = VirtualMachine::new();
        vm.execute(&bytecode).map_err(|e| e.to_string())?;
    }

    Ok(())
}
```
--- END OF FILE: artificial-language/ALC/src/runner.rs ---

--- START OF FILE: artificial-language/ALC/src/bundler.rs ---
```rust
// artificial-language/ALC/src/bundler.rs
use artificial_vm::{serialize, BytecodeModule};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Creates a standalone executable that embeds the VM and bytecode
pub fn create_standalone(
    bytecode: &BytecodeModule,
    out_dir: &Path,
    file_stem: &str,
) -> Result<PathBuf, String> {
    // Serialize bytecode
    let bytecode_bytes = serialize(bytecode);

    // Generate the embedded Rust source
    let rust_source = generate_standalone_source(&bytecode_bytes);

    // Write the generated source file
    let rs_path = out_dir.join(format!("{}_standalone.rs", file_stem));
    let mut file = fs::File::create(&rs_path)
        .map_err(|e| format!("Failed to create standalone source file: {}", e))?;
    file.write_all(rust_source.as_bytes())
        .map_err(|e| format!("Failed to write standalone source: {}", e))?;

    // Compile with rustc
    let binary_path = if cfg!(windows) {
        out_dir.join(format!("{}_art.exe", file_stem))
    } else {
        out_dir.join(format!("{}_art", file_stem))
    };

    let output = Command::new("rustc")
        .arg(&rs_path)
        .arg("-o")
        .arg(&binary_path)
        .arg("-O") // Optimize
        .output()
        .map_err(|e| format!("Failed to run rustc: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("rustc compilation failed:\n{}", stderr));
    }

    Ok(binary_path)
}

fn generate_standalone_source(bytecode_bytes: &[u8]) -> String {
    let mut source = String::new();

    // File header
    source.push_str("// Generated standalone executable for Artificial Language\n");
    source.push_str("// DO NOT EDIT - This file is auto-generated\n\n");

    // Embedded bytecode as a const array
    source.push_str("const BYTECODE: &[u8] = &[\n    ");
    for (i, byte) in bytecode_bytes.iter().enumerate() {
        source.push_str(&format!("0x{:02X}, ", byte));
        if (i + 1) % 12 == 0 {
            source.push_str("\n    ");
        }
    }
    source.push_str("\n];\n\n");

    // Embed the minimal VM implementation
    source.push_str(EMBEDDED_VM_SOURCE);

    source
}

/// Minimal VM implementation to embed in standalone executables
const EMBEDDED_VM_SOURCE: &str = r#"
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

struct BytecodeModule {
    constants: Vec<Value>,
    instructions: Vec<u8>,
}

fn deserialize(bytes: &[u8]) -> BytecodeModule {
    let mut pos = 0;
    
    // Skip magic (4 bytes) and version (1 byte)
    pos += 5;
    
    // Read constant count
    let const_count = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
    pos += 4;
    
    let mut constants = Vec::new();
    for _ in 0..const_count {
        let type_tag = bytes[pos];
        pos += 1;
        match type_tag {
            0 => constants.push(Value::Null),
            1 => {
                let len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
                pos += 4;
                let s = String::from_utf8(bytes[pos..pos+len].to_vec()).unwrap();
                pos += len;
                constants.push(Value::String(s));
            }
            2 => {
                let n = i64::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3],
                                           bytes[pos+4], bytes[pos+5], bytes[pos+6], bytes[pos+7]]);
                pos += 8;
                constants.push(Value::Integer(n));
            }
            3 => {
                let b = bytes[pos] != 0;
                pos += 1;
                constants.push(Value::Boolean(b));
            }
            _ => panic!("Unknown type tag"),
        }
    }
    
    // Read instruction count
    let instr_count = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
    pos += 4;
    
    let instructions = bytes[pos..pos+instr_count].to_vec();
    
    BytecodeModule { constants, instructions }
}

fn execute(module: &BytecodeModule) {
    let mut stack: Vec<Value> = Vec::new();
    let mut ip = 0;
    
    while ip < module.instructions.len() {
        match module.instructions[ip] {
            0x00 => ip += 1, // Nop
            0x01 => break,   // Halt
            0x02 => {        // PushConst
                let idx = ((module.instructions[ip+1] as u16) << 8) | (module.instructions[ip+2] as u16);
                stack.push(module.constants[idx as usize].clone());
                ip += 3;
            }
            0x03 => {        // Print
                if let Some(v) = stack.pop() {
                    print!("{}", v);
                    io::stdout().flush().ok();
                }
                ip += 1;
            }
            0x04 => {        // PrintLn
                if let Some(v) = stack.pop() {
                    println!("{}", v);
                }
                ip += 1;
            }
            0x05 => {        // Pop
                stack.pop();
                ip += 1;
            }
            _ => panic!("Unknown opcode"),
        }
    }
}

fn main() {
    let module = deserialize(BYTECODE);
    execute(&module);
}
"#;
```
--- END OF FILE: artificial-language/ALC/src/bundler.rs ---

--- START OF FILE: artificial-language/main.art ---
```text
// artificial-language/main.art
accrete "Hello Artificial World!"
```
--- END OF FILE: artificial-language/main.art ---

---

### **Part 3: Appendix - Development Best Practices & Quality Mandates**

You must adhere to the following best practices throughout the entire development process.

#### **Error Prevention Checklist**

1.  **Workspace Configuration**
    *   ✅ Always specify `resolver = "2"` in the workspace `Cargo.toml`.
    *   Prevents: `"virtual workspace defaulting to resolver = 1"` warnings.

2.  **.gitignore Setup**
    *   ✅ Create proper `.gitignore` files in the root and in each crate.
    *   ✅ Include project-specific artifacts like `target/artificial_out/` and `*.artb`.
    *   Prevents: Accidentally committing build artifacts and generated files.

3.  **Unused Code Cleanup**
    *   ✅ Immediately remove any unused imports.
    *   ✅ Prefix all unused variables with an underscore (`_`).
    *   ✅ Use `#[allow(dead_code)]` for intentionally unused functions.
    *   Prevents: A cluttered codebase and compiler warnings.

4.  **Crate Dependencies**
    *   ✅ Always use `path` dependencies for local workspace crates.
    *   ✅ Use workspace-level metadata for version and edition.
    *   Prevents: Broken module imports and versioning conflicts.

5.  **Bytecode Design**
    *   ✅ Use clear, well-documented opcode definitions.
    *   ✅ Include magic bytes and version in serialized bytecode for validation.
    *   ✅ Use big-endian encoding for multi-byte values for consistency.
    *   Prevents: Deserialization errors and version incompatibilities.

6.  **Virtual Machine Implementation**
    *   ✅ Implement proper stack bounds checking.
    *   ✅ Handle all opcodes exhaustively.
    *   ✅ Provide clear error messages with instruction pointer location.
    *   Prevents: Silent failures and difficult debugging.

7.  **Error Handling**
    *   ✅ Define distinct, clear error types for each phase (LexError, ParseError, VmError).
    *   ✅ Always include location information where applicable.
    *   ✅ Chain errors with context.
    *   Prevents: Cryptic, unhelpful error messages during debugging.

8.  **Testing During Development**
    *   ✅ Build frequently with `cargo build`.
    *   ✅ Continuously check for warnings and treat them as errors.
    *   ✅ Run the full compiler pipeline on test files often.
    *   Prevents: Discovering deep-rooted issues late in the process.

#### **Common Pitfalls to Avoid**

| Issue                                  | Solution                                                              |
|----------------------------------------|-----------------------------------------------------------------------|
| Unused imports causing warnings        | Remove them immediately or use `#[allow(unused_imports)]` sparingly.  |
| Module not found errors                | Declare the module (`mod my_module;`) in its parent file.             |
| Wrong Cargo.toml resolver              | Add `resolver = "2"` to the `[workspace]` table.                      |
| Generated files in git                 | Ensure `.gitignore` patterns correctly cover output directories.      |
| Stack underflow in VM                  | Always check stack before popping.                                    |
| Bytecode deserialization failures      | Validate magic bytes and version before parsing.                      |
| Instruction pointer out of bounds      | Check bounds before reading operands.                                 |

#### **Final Verification Checklist**

- [ ] `cargo build` succeeds with **zero warnings**.
- [ ] `cargo clippy` produces **no warnings**.
- [ ] Bytecode is correctly generated and can be deserialized.
- [ ] VM correctly executes all opcodes.
- [ ] Error messages are clear and actionable.
- [ ] `.gitignore` files are correctly configured and placed.
- [ ] All unused code has been removed or properly marked.
- [ ] The workspace `resolver` is set to `"2"`.
- [ ] Standalone bundled executables work correctly.
- [ ] The final success criteria are met exactly.

---

**END OF INSTRUCTION**

