[![CI](https://github.com/nogibjj/Su_Zhang_IDS706_Week7/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/Su_Zhang_IDS706_Week7/actions/workflows/rust.yml)

# Su Zhang IDS706 Project 7 - Rust Command Line

## Project Purpose
This project demonstrates the creation of a simple calculator using the Rust programming language. The calculator supports basic arithmetic operations (addition, subtraction, multiplication, and division) and is designed to be run as a command-line tool. 

## Project Structure

The project is structured as follows:

```
.
├── Cargo.toml             # Rust project manifest file (dependencies and metadata)
├── src
│   └── main.rs            # Main Rust source code (calculator logic and command-line argument parsing)
└── README.md              # Project documentation (this file)

```

### File Descriptions:

- **Cargo.toml**: This file contains the dependencies and project metadata for the Rust project. It defines which crates are used, including the `clap` crate for parsing command-line arguments.
  
- **src/main.rs**: The main source file written in Rust that implements the calculator logic. It includes functions for performing addition, subtraction, multiplication, and division based on user input via command-line arguments.

## Command-Line Tool Functionality

This command-line tool allows you to perform four basic arithmetic operations: **addition**, **subtraction**, **multiplication**, and **division**. It takes two numbers as input and prints the result to the console.

### Available Commands:
The tool supports the following operations:
- `add`: Adds two numbers.
- `subtract`: Subtracts the second number from the first.
- `multiply`: Multiplies two numbers.
- `divide`: Divides the first number by the second (with error handling for division by zero).

### Command Syntax:

```bash
cargo run -- <operation> <num1> <num2>
```

Where:
- `<operation>`: The arithmetic operation to perform (add, subtract, multiply, divide).
- `<num1>` and `<num2>`: The two numbers you want to calculate.

### Example Usage:

1. **Addition**:
   ```bash
   cargo run -- add 5 3
   ```
   **Output**:
   ```
   Sum: 8
   ```

2. **Subtraction**:
   ```bash
   cargo run -- subtract 10 4
   ```
   **Output**:
   ```
   Subtraction: 6
   ```

3. **Multiplication**:
   ```bash
   cargo run -- multiply 6 7
   ```
   **Output**:
   ```
   Product: 42
   ```

4. **Division**:
   ```bash
   cargo run -- divide 20 4
   ```
   **Output**:
   ```
   Division: 5
   ```

   **Division by Zero Handling**:
   ```bash
   cargo run -- divide 10 0
   ```
   **Output**:
   ```
   Error: Division by zero is not allowed.
   ```

## User Guide
Detailed user guide could be found [here](https://github.com/nogibjj/Su_Zhang_IDS706_Week7/blob/main/user_guide.md).

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/noahgift/rust-new-project-template
