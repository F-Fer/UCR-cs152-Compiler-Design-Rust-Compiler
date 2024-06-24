# Rust Compiler for CS152: Compiler Design at UCR

This repository contains a custom handwritten compiler written in Rust as part of the Compiler Design course (CS152) at the University of California, Riverside (UCR). This project is heavily based on the repository of my instructor, which can be found [here](https://github.com/danieltan1517/teh_tarik).

## Overview

The goal of this project is to design and implement a simple compiler from scratch using the Rust programming language. This includes the development of a lexer, parser and  semantic analyzer for a subset of a hypothetical programming language.

## Features

- **Lexer**: Tokenizes the input source code into meaningful symbols.
- **Parser**: Constructs a syntax tree from the tokens provided by the lexer.
- **Semantic Analysis**: Ensures the syntax tree adheres to the language's semantic rules.
- **Code Generation**: Translates the syntax tree into an intermediate representation or target code.

## Repository Structure

- `src/`
  - `interpreter.rs`: Contains the implementation of the final-code generator from intermediate code.
  - `main.rs`: The main entry point of the compiler.
- `examples/`: Example source codes to be compiled using this compiler.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system. If not, you can download and install it from [rust-lang.org](https://www.rust-lang.org/).

### Installation

1. Clone this repository:

   ```sh
   git clone https://github.com/F-Fer/UCR-cs152-Compiler-Design-Rust-Compiler.git
   cd UCR-cs152-Compiler-Design-Rust-Compiler
