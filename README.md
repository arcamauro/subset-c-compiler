# subset-c-compiler

A compiler for a subset of the C programming language, written in Rust targeting RISC-V.
## Project Status
This project is currently under active development. You can check the [`dev`](./tree/dev) branch for the latest ongoing work.
- [x] **Lexer**: Finite State Automaton (FSA) approach
- [x] **Parser & AST**: Top-Down Recursive Descent Parser (LL(1)) generating a strongly-typed Abstract Syntax Tree
- [x] **AST Generation**: Full AST modeling using Rust algebraic data types (`enum`) and smart pointers (`Box`)
- [ ] **Intermediate code**: to lower AST constructs into three-address code (TAC).
- [ ] **Code Generation**: Target code emission for RISC-V architecture
