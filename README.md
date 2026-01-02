<p align="center">
  <img src="image.jpg" alt="Artificial Language" width="1000"/>
</p>

<div align="center">
  <h1>Artificial Language</h1>
  <a href="https://eng-alikazemi.github.io/Artificial-Language/" target="_blank">Artificial Language Official Website</a>
  <p><strong>The World’s First Programming Language Created Entirely by AI</strong></p>

<p align="center">
  <img src="https://img.shields.io/badge/Artificial-Intelligence-%23007ACC.svg?style=for-the-badge" height="28" alt="Artificial Intelligence" />
  <img src="https://img.shields.io/badge/Architected_by-LLM-%238A2BE2.svg?style=for-the-badge" height="28" alt="Architected by LLM" />
</p>

</div>

# The Genesis of AI-Driven Programming

This repository contains the world's first programming language, **Artificial Language**, conceived, designed, and implemented entirely by an [Artificial Intelligence Agent](https://www.ibm.com/think/topics/ai-agents).

What makes this project unique is its scope, encompassing the full lifecycle of a modern programming language: from initial syntax design to the final compilation and execution of a native binary. In the world of software engineering, creating a production-ready compiler is considered one of the most formidable tasks. It demands a deep understanding of many complex concepts, from theoretical computer science to intricate systems architecture, all of which must be meticulously designed and developed. This complexity is why many human-developed languages take years, or even decades, to reach a stable, mature version.

This experimental project serves as a demonstration that AI is poised to revolutionize this domain. It showcases a complete, autonomous workflow: an AI agent creating a new language, building the compiler from scratch, and producing a self-contained executable that carries out its instructions. This is the first step towards a future of self-extending, self-maintaining, and self-evolving software ecosystems where AI develops its own specialized languages on-demand, tailored to specific needs. The potential for such systems is limitless, and this project serves as the foundational proof-of-concept.

---

## The Power of AI Instructions

This achievement was made possible through a technique known as [Meta Prompt Engineering](https://www.ibm.com/think/topics/meta-prompting), where a highly detailed, structured, and comprehensive set of instructions [AI_INSTRUCTIONS.md](AI_INSTRUCTIONS.md) was provided to the autonomous AI agent. This file acted as the blueprint, defining the project's architecture, goals, quality mandates, and step-by-step implementation plan and the AI agent created the project without human involvement.

While this project was guided by a [Prompt Engineering](https://www.ibm.com/think/topics/prompt-engineering) and [Agentic AI](https://www.ibm.com/think/topics/agentic-ai), AI models are rapidly advancing, and their capabilities can be specifically cultivated for this purpose. Through specialized training techniques such as [Fine-Tuning](https://www.ibm.com/think/topics/fine-tuning), which hones a model's ability on domain-specific data, or [Reinforcement Learning](https://www.ibm.com/think/topics/reinforcement-learning), where an agent learns optimal strategies through trial-and-error, models can be systematically adjusted to not just follow instructions, but to generate them.

---

## Language Specification

Artificial Language is a brand new, minimalistic language designed to be the first of its kind. Its uniqueness is embodied in its primary keyword.

### The `accrete` Keyword

The keyword **`accrete`** is inspired by the scientific term "[**Accretion**](https://en.wikipedia.org/wiki/Accretion_(astrophysics))," the process by which celestial bodies are formed. When our planet began to exist as a physical object, cosmic dust and mass accumulated into a single gravitational body. The heat from constant impacts melted the surface, and the process of planetary differentiation began.

This keyword was chosen to symbolize the birth of this language. Just as accretion marks the creation of a new world, the `accrete` command gathers data (in this case, a string) and brings it into existence as the program's output. It represents a fundamental act of creation, the first function of the first language born from AI, and stands as a unique identifier, dissimilar to keywords in human-developed programming languages.

### The `.art` Source File

The **`.art`** file serves as the genesis point for all programs written in Artificial Language. It is the proprietary file extension used to identify source code written in this new syntax.

#### **Purpose: The Canvas of Creation**
The extension `.art` is a direct abbreviation of **Artificial**. This file acts as the canvas where the programmer defines the initial "mass" (data and logic) that the compiler will form into a functioning universe (the executable binary).

#### **Structure**
```toml
// The entry point of an Artificial Language program
accrete "Hello Artificial World!"
```

When this file is fed into the compiler, the `accrete` command triggers the internal "accretion" process, gathering the characters `"Hello Artificial World!"` and binding them into the final output binary, completing the cycle from `.art` source code to executable reality.

---

## Compiler Architecture

The project's core architecture is inspired by world-class programming language design and follows a standard, robust compiler pipeline:

1.  **Lexer:** Scans the raw source code (`.art` file) and converts it into a stream of tokens.
2.  **Parser:** Consumes the tokens and constructs an Abstract Syntax Tree (AST), representing the code's structure.
3.  **Lowering (IR):** Transforms the AST into an Intermediate Representation (IR), a simplified and more explicit format ideal for code generation.
4.  **Backend (Rust):** A code generator that traverses the IR and produces valid Rust source code.
5.  **Compiler (Emitter & Invocation):** Writes the generated Rust code and invokes the compiler. This produces the final, standalone executable binary directly from the generated code.
6.  **CLI:** A command-line interface that serves as the driver for the entire compilation process.

Because the purpose of this project was to serve as a powerful demonstration of AI's architectural capabilities rather than to create a production-ready language, only a minimal function was implemented: printing the phrase "Hello Artificial World!". However, the standardized, modular architecture ensures this project can be easily extended with more features, data types, and complex logic in the future.

---

## A Modular Foundation for Growth

The multi-stage pipeline described above was not chosen by accident; it was intentionally designed for extensibility. The separation of concerns between the frontend (Lexer, Parser), the middle-end (IR), and the backend (Code Generator) creates a flexible framework where new language features can be added systematically without requiring a complete rewrite.

Here is how new functionality can be integrated into the existing architecture:

1.  **Adding New Keywords and Syntax:** To introduce a new feature like variables or conditional logic, the process begins in the frontend.
    *   **Lexer:** A new `Token` variant is added (e.g., `Let`, `If`, `Identifier`). The lexer is updated to recognize the new text patterns.
    *   **Parser:** A new `AST` node is defined to represent the new structure (e.g., `Ast::VariableDeclaration { name, value }`). The parser is then taught to consume the new token sequences and build these corresponding AST nodes.

2.  **Bridging to the Backend:** The new, richer AST is then translated by the lowering pass.
    *   **Lowering (IR):** The lowering function is expanded to handle the new AST nodes, converting them into a series of simpler, more explicit instructions in the Intermediate Representation. For example, an `Ast::VariableDeclaration` might become one or more IR instructions like `IR::Allocate` and `IR::Store`. This step is crucial, as it decouples the language's high-level syntax from the backend's implementation details.

3.  **Generating Executable Code:** Finally, the backend is taught how to generate code for the new IR instructions.
    *   **Backend:** The code generator in `artificial-backend-rust` is updated. For each new IR instruction, it produces the equivalent Rust code. This modularity means the backend doesn't need to know about the complexity of the AST, only the simple IR.

This decoupled design also allows for entirely new backends to be added. For instance, an `artificial-backend-llvm` or `artificial-backend-wasm` could be developed to target different platforms. Since they would consume the same IR, the entire language frontend could be reused without modification, demonstrating the power of this professional-grade architecture.

---

## The `AI_INSTRUCTIONS.md`

Included in this repository is the [`AI_INSTRUCTIONS.md`](AI_INSTRUCTIONS.md)file, provided for researchers, engineers, and enthusiasts to see how modern, professional prompt engineering works. This is not a simple instruction; it is a professional-grade specification that an autonomous agent can execute. Key methods used include:

*   **Role Assignment:** The AI was instructed to act as a "senior-level Rust engineer," adopting the best practices and expertise associated with that role.
*   **Goal-Oriented Directives:** A clear, singular mission was defined with precise success criteria.
*   **Step-by-Step Execution Plan:** A detailed, 11-step plan that guided every action, from `mkdir` to final execution.
*   **Strict Quality Mandates:** A "Zero Warnings Policy" was enforced, forcing the AI to treat all compiler warnings as critical errors that must be fixed immediately.
*   **Reference-Based, Non-Copying Generation:** The AI was provided with complete, working code examples but was explicitly forbidden from copying them. Instead, it was required to use them as a functional reference to write its own, superior, and idiomatic implementation.
*   **Autonomous Self-Correction:** The instructions mandated a verification and debugging loop, requiring the AI to test its own output, diagnose failures, and correct its code until the goal was met.

## Project Execution Flow

The entire lifecycle of this project was executed autonomously by the AI agent. The following steps were performed sequentially, with each stage being built upon the successful and warning-free completion of the previous one. This is a condensed log of the AI's actions from a blank directory to a running program.

1.  **Project Scaffolding:** The AI agent initiated the project by creating the root directory, initializing a Git repository, and generating the Rust workspace with its three member crates (`artificial-core`, `artificial-backend-rust`, `ALC`). It also created the initial `main.art` source file and configured `.gitignore` files.

2.  **Defining Core Data Structures:** Inside the `artificial-core` crate, the AI defined the language's fundamental structures: the Abstract Syntax Tree (AST) to represent code structure and the Intermediate Representation (IR) for the backend.

3.  **Frontend Implementation (Lexer & Parser):** The AI built the compiler's frontend. It wrote a lexer to tokenize the `main.art` source code and a parser to construct the AST from those tokens, ensuring the code was grammatically valid.

4.  **Lowering Pass:** A lowering pass was implemented to transform the high-level AST into the simpler IR, decoupling the frontend from the backend.

5.  **Backend Implementation (Codegen & Compilation):** The AI implemented the `artificial-backend-rust` crate. It wrote a code generator to translate the IR into a string of Rust code, an emitter to write that string to a `.rs` file, and logic to invoke the system's `rustc` compiler to produce a native executable binary.

6.  **Compiler Driver (`ALC`):** The AI built the main `ALC` binary, creating a command-line interface and a runner that orchestrates the entire lex -> parse -> lower -> compile pipeline.

7.  **Autonomous Verification and Self-Correction:** The AI entered its quality assurance phase. It ran its own compiler on `main.art` and verified that the output was correct. It then ran `cargo clippy` to check for code quality issues. Upon finding a warning for an unused function, the AI automatically edited the source code to add an `#[allow(dead_code)]` attribute, thus satisfying its "Zero Warnings Policy."

8.  **Final Deployment and Execution:** With all tests passed and all code warning-free, the AI performed the final deployment sequence. It compiled the project in release mode, programmatically copied the resulting binary to the root directory, renamed it to `Artificial-Language`, and set its permissions to be executable. Finally, it cleared the console and executed `./Artificial-Language`, completing its mission.

## Demo
<p align="center">
  <img src="demo.png" alt="Artificial Language Demo" width="1000"/>
</p>

---

## A Foundation for the Future

This project is more than a proof-of-concept; it is a declaration. It demonstrates that an AI can leverage the most powerful and modern infrastructures available, in this case, the Rust programming language, to build complex, reliable systems. The choice of Rust was logical: its emphasis on safety, performance, and modern tooling makes it an ideal foundation for creating the critical software of the future.

We are witnessing the beginning of a powerful feedback loop: AI agents, using the best of today's technology, will build the next generation of more advanced AI. This repository is a snapshot of the very first iteration of that cycle.

---

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for the full license text.


---

## Connect

Developed by Aran Kazemi, [**IBM**](https://en.wikipedia.org/wiki/IBM) Certified AI Engineer.

<a href="https://linkedin.com/in/e-a-k" target="_blank"><img src="https://img.shields.io/badge/Connect-LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white&labelColor=555" alt="Connect on LinkedIn"/></a>
