use clap::Parser; // Import the Parser trait from the clap crate to parse command-line arguments

// Define a struct to hold the command-line arguments
#[derive(Parser, Debug)] // Derive the Parser trait for Args to enable parsing and Debug for printing
#[command(version = "1.0", about = "calculator")] // Metadata: version and description of the command-line tool
struct Args {
    #[command(subcommand)] // This specifies that Args contains a subcommand (like add, multiply, etc.)
    command: Commands, // Store the subcommand in the command field
}

// Define an enum to list the supported subcommands
#[derive(Parser, Debug)] // Derive the Parser and Debug traits for Commands
enum Commands {
    Add { a: i64, b: i64 },
    Multiply { a: i64, b: i64 },
    Subtract { a: i64, b: i64 },
    Divide { a: i64, b: i64 },
}

fn main() {
    let args = Args::parse(); // Parse the command-line arguments into the Args struct

    // Match the parsed subcommand and execute the appropriate code for each case
    match args.command {
        Commands::Add { a, b } => println!("Sum: {}", a + b),
        Commands::Multiply { a, b } => println!("Product: {}", a * b),
        Commands::Subtract { a, b } => println!("Subtraction: {}", a - b),
        Commands::Divide { a, b } => {
            // If 'divide' is used, check if b is zero to avoid division by zero
            if b == 0 {
                println!("Error: Division by zero is not allowed.");
            } else {
                println!("Division: {}", a / b);
            }
        }
    }
}
