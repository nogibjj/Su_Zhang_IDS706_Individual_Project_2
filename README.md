# Su Zhang IDS706 Individual Project #2

## Project Overview

This project presents both Python and Rust implementation of an ETL (Extract, Transform, Load) process, which performs operations on an SQLite database, including extracting data from a source, loading it into the database, and providing CRUD (Create, Read, Update, Delete) functionalities. This project also includes time and memory usage tracking to evaluate the performance of Rust compared to an existing Python implementation.

## Objectives

1. **Rewrite Python Scripts in Rust**: 
   - The primary goal of this project is to rewrite existing Python scripts that perform ETL operations in Rust. This enables us to explore Rust's performance capabilities and its handling of database operations.

2. **Performance Comparison**: 
   - By measuring execution time and memory usage for both the Rust and Python implementations, we can compare their efficiencies in handling ETL processes.

3. **Rust CLI Binary with SQLite**: 
   - The project implements a command-line interface (CLI) in Rust that allows users to perform various operations on an SQLite database efficiently.

## Features

- **Data Extraction**: Extracts data from a specified URL and saves it locally.
- **Loading Data**: Loads the extracted data into an SQLite database.
- **CRUD Operations**: Provides the following functionalities:
  - **Create**: Insert new records into the database.
  - **Read**: Retrieve and display records from the database.
  - **Update**: Modify existing records in the database.
  - **Delete**: Remove records from the database.
- **Performance Tracking**: Measures and displays the execution time and memory usage for each operation.

## Project Structure

Here’s the structure of the Rust ETL-Query project:

```
SU_ZHANG_IDS706_Individual_Project_2/
├── .devcontainer/           # Development container settings
├── .github/workflows/       # GitHub Actions workflows
│   ├── python.yml          # CI configuration for Python
│   └── rust.yml            # CI configuration for Rust
├── data/                    # Directory for data files
│   └── drinks.csv          # Sample data file
├── mylib/                   # Python library for ETL
│   ├── __pycache__/        # Cached Python files
│   ├── extract.py          # Python extraction script
│   ├── query.py            # Python query script
│   └── transform_load.py    # Python loading script
├── src/                     # Source code for Rust application
│   ├── lib.rs              # Library code for ETL operations
│   └── main.rs             # Entry point for the Rust application
├── target/                  # Compiled output directory for Rust
├── .gitignore               # Git ignore file
├── Cargo.lock               # Dependency lock file for Rust
├── Cargo.toml               # Project metadata and dependencies
├── Drinks.db                # SQLite database file
├── LICENSE                  # License file
├── Makefile                 # Makefile for build automation
├── README.md                # Project documentation
├── requirements.txt         # Python dependencies
└── test_main.py             # Test script for Python
```

## Performance Comparison Summary

| Operation          | Python Execution Time | Rust Execution Time | Python Memory Consumed | Rust Memory Consumed |
|---------------------|-----------------------|---------------------|-------------------------|----------------------|
| **Extract**         | 173.66 ms             | 272.03 ms           | 4000.00 KB              | 4244 KB              |
| **Load**            | 5.32 ms               | 79.56 ms            | 800.00 KB               | 196 KB               |
| **Create**          | 0.50 ms               | 20.90 ms            | 176.00 KB               | 0 KB                 |
| **Read**            | 0.17 ms               | 17.07 ms            | 0.00 KB                 | 148 KB               |
| **Update**          | 0.51 ms               | 16.31 ms            | 0.00 KB                 | 0 KB                 |
| **Delete**          | 0.54 ms               | 16.91 ms            | 0.00 KB                 | 33 KB                |
| **Total**           | 180.70 ms             | 465.48 ms           | 4992.00 KB              | 8503 KB              |

## Build the Project

```bash
cargo build --release
```

## Running the CLI

To run the CLI binary, use the following command:

```bash
cargo run -- --url "<DATA_SOURCE_URL>" --file_path "<LOCAL_FILE_PATH>" --action <create|read|update|delete>
```

For default settings of data source (drinks.csv), could skip specifying url and file_path:

```bash
cargo run -- --action <create|read|update|delete>
```


**Example**:
```bash
cargo run -- --url "https://example.com/data.csv" --file_path "data/my_data.csv" --action create
```

### Arguments

- `--url`: The URL from which to extract data (default: a specified example URL).
- `--file_path`: The local path where the extracted data will be saved (default: `data/drinks.csv`).
- `--action`: The operation to perform on the SQLite database (`create`, `read`, `update`, or `delete`). The default action is `read`.


## Conclusion

In this project, we implemented an ETL process in both Python and Rust, measuring and comparing their performance in terms of execution time and memory usage.

- **Execution Time**: The total execution time for the Rust implementation (465.48 ms) was longer than the Python version (180.70 ms). This is likely due to the additional overhead associated with Rust's strict memory safety checks and the complexities of its ownership model.
  
- **Memory Usage**: The Rust version consumed more memory overall (8503 KB) compared to the Python implementation (4992 KB). This difference may arise from Rust's management of data ownership, which can sometimes lead to higher memory usage during operations.

- **Efficiency**: While Rust is designed for high performance and safety, its execution time was slower in this instance. This emphasizes that the effectiveness of a programming language can depend heavily on the specific use case and implementation details.

Overall, this comparison highlights the strengths and weaknesses of both languages in handling ETL processes. Python's ease of use and mature libraries allowed for faster execution, while Rust's performance characteristics could provide advantages in more complex scenarios or when optimization is more critical.

## Data Source and References

* Data source: https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/noahgift/rust-new-project-template
