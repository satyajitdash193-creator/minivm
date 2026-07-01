# MiniVM

A stack-based virtual machine written in Rust.

This project was developed as part of a Rust Internship Minor Project.

---

## Features

- Stack-based Virtual Machine
- Custom Assembly Language
- Bytecode Generator
- Bytecode Execution
- Disassembler
- Trace Mode
- Arithmetic Instructions
- Stack Instructions
- Memory Instructions

---

## Project Structure

```
minivm
│
├── src
│   ├── main.rs
│   ├── isa.rs
│   ├── assembler.rs
│   ├── vm.rs
│   └── disassembler.rs
│
├── Cargo.toml
├── README.md
├── test.tasm
└── test.tbc
```

---

## Instruction Set

| Instruction | Description |
|-------------|-------------|
| PUSH n | Push value onto stack |
| POP | Remove top value |
| DUP | Duplicate top value |
| SWAP | Swap top two values |
| ADD | Addition |
| SUB | Subtraction |
| MUL | Multiplication |
| DIV | Division |
| MOD | Modulus |
| NEG | Negation |
| LOAD n | Load from memory |
| STORE n | Store into memory |
| PRINT | Print top value |
| HALT | Stop execution |

---

## Build

```bash
cargo build
```

---

## Assemble

```bash
cargo run -- asm test.tasm -o test.tbc
```

---

## Run

```bash
cargo run -- run test.tbc
```

---

## Disassemble

```bash
cargo run -- dis test.tbc
```

---

## Trace Mode

```bash
cargo run -- run --trace test.tbc
```

---

## Example Program

```asm
PUSH 7
PUSH 3
ADD
PRINT
HALT
```

Output

```
10
Program Finished
```

---

## Acceptance Tests

- Arithmetic Operations
- Stack Operations
- Memory Operations
- Bytecode Generation
- Virtual Machine Execution
- Disassembler
- Trace Mode

---

## Known Limitations

- Labels and branching are not implemented.
- Floating-point instructions are not supported.
- Only integer operations are supported.

---

## Author

SATYAJIT DASH

Rust Internship Minor Project