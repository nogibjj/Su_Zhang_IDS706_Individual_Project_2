### User Guide: Command-Line Calculator in Rust

#### Overview
This Rust program is a simple command-line calculator that allows you to perform basic arithmetic operations like addition, multiplication, subtraction, and division. The program uses the `clap` crate for parsing command-line arguments and supports the following operations:

- **Addition**
- **Multiplication**
- **Subtraction**
- **Division (with division by zero check)**

#### Prerequisites
Before running the program, ensure the following are set up:

1. **Step 1: Install Rust**: Ensure Rust is installed. Here are the detailed steps for installing **rust**

- Open your terminal and run 

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

- Follow the on-screen instructions to complete the installation. Typically, you'll be prompted to confirm the installation and modify your environment.

- Verify the installation by checking the Rust version:

   ```bash
   rustc --version
   ```

   If Rust is successfully installed, this command will output the version of Rust, like:
   ```
   rustc 1.60.0 (a.b.c)
   ```

2. **Clap Dependency**: The `clap` crate is used to parse command-line arguments. Ensure your `Cargo.toml` file includes:

   ```toml
   [dependencies]
   clap = { version = "4.0", features = ["derive"] }
   ```

#### Usage Instructions

1. **Navigate to the Project Directory**:
   Make sure you're in the correct directory where the `Cargo.toml` file is located. You can do this with the following command:
   
   ```bash
   cd /path/to/your/rust/project
   ```

2. **Compile the Program**:
   Run the following command to compile the program:
   
   ```bash
   cargo build
   ```

3. **Run the Program**:
   Use the `cargo run` command followed by the arithmetic operation (`add`, `multiply`, `subtract`, or `divide`) and the two numbers you want to operate on.

   ### Command Syntax:
   ```bash
   cargo run -- <operation> <num1> <num2>
   ```

   Replace `<operation>` with one of the supported operations (`add`, `multiply`, `subtract`, `divide`), and `<num1>`, `<num2>` with the numbers you want to calculate.

#### Examples of Operations

1. **Addition**:
   To add two numbers:
   
   ```bash
   cargo run -- add 5 10
   ```
   **Output**:
   ```
   Sum: 15
   ```

2. **Multiplication**:
   To multiply two numbers:
   
   ```bash
   cargo run -- multiply 6 4
   ```
   **Output**:
   ```
   Product: 24
   ```

3. **Subtraction**:
   To subtract two numbers:
   
   ```bash
   cargo run -- subtract 10 3
   ```
   **Output**:
   ```
   Subtraction: 7
   ```

4. **Division**:
   To divide two numbers:
   
   ```bash
   cargo run -- divide 20 4
   ```
   **Output**:
   ```
   Division: 5
   ```

   - **Division by Zero**: If you attempt to divide by zero, the program will handle the error gracefully:
   
     ```bash
     cargo run -- divide 10 0
     ```
     **Output**:
     ```
     Error: Division by zero is not allowed.
     ```

#### Command-Line Arguments

The calculator accepts the following arguments:

- **add**: Performs addition of two numbers.
- **multiply**: Performs multiplication of two numbers.
- **subtract**: Performs subtraction of two numbers (first minus second).
- **divide**: Performs division of two numbers (first divided by second). If the second number is zero, an error message is displayed.

#### Help and Version Information

- To display the help message, which provides an overview of how to use the calculator, run:

  ```bash
  cargo run -- --help
  ```

  **Output**:
  ```
  calculator 1.0
  Simple calculator that supports basic arithmetic operations.

  USAGE:
      cargo run -- [SUBCOMMAND]

  FLAGS:
      -h, --help       Prints help information
      -V, --version    Prints version information

  SUBCOMMANDS:
      add         Add two numbers
      divide      Divide two numbers
      multiply    Multiply two numbers
      subtract    Subtract two numbers
  ```

- To display the version information, run:

  ```bash
  cargo run -- --version
  ```

  **Output**:
  ```
  calculator 1.0
  ```

#### Error Handling

The program provides basic error handling:

- **Invalid Operation**: If you enter an invalid operation or no operation, the program will print an error message and display the available subcommands.
  
  Example:
  ```bash
  cargo run -- invalid 5 10
  ```

  **Output**:
  ```
  error: The subcommand 'invalid' wasn't recognized
  ```

- **Division by Zero**: If you attempt to divide by zero, the program will output an error message without crashing.

