use clap::Parser;
use su_zhang_sqlite::{extract, load, read_data, create_data, delete_data, update_data}; //import functions defined in lib.rs
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt}; 
use tokio;

#[derive(Parser)]
#[command(name = "example")]
#[command(about = "A Rust program for SQLite ETL and query pipeline with memory and runtime tracking", long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv"
    )]
    url: String,

    #[arg(short, long, default_value = "data/drinks.csv")]
    file_path: String,

    #[arg(short, long, default_value = "read")] // Default action is "read"
    action: String,
}

async fn run_main(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("URL: {}", args.url);
    println!("File path: {}", args.file_path);
    
    // Start timing for the entire process
    let start_time = Instant::now();
    let mut system = System::new_all();
    system.refresh_all();

    // Measure initial memory usage
    let pid = sysinfo::get_current_pid().expect("Failed to get process ID");
    let initial_memory = system.process(pid).map_or(0, |process| process.memory());

    // Perform data extraction and track time and memory
    track_memory_and_time("Extract", || {
        extract(&args.url, &args.file_path)
    })?;

    // Load data into the database and track time and memory
    track_memory_and_time("Load", || {
        load(&args.file_path)
    })?;

    // Perform CRUD operations based on user input and track them
    let operations = vec!["create", "read", "update", "delete"];
    for operation in operations {
        match operation {
            "read" => {
                track_memory_and_time("Read", || {
                    read_data()
                })?;
            }
            "create" => {
                track_memory_and_time("Create", || {
                    create_data()
                })?;
            }
            "delete" => {
                track_memory_and_time("Delete", || {
                    delete_data()
                })?;
            }
            "update" => {
                track_memory_and_time("Update", || {
                    update_data()
                })?;
            }
            _ => {}
        }
    }

    // Measure final memory usage
    system.refresh_all();
    let final_memory = system.process(pid).map_or(0, |process| process.memory());

    // Calculate total elapsed time and memory usage
    let elapsed_time = start_time.elapsed();
    let memory_used = final_memory.saturating_sub(initial_memory);

    // Print total performance metrics
    println!("Total Process completed in: {:.2?}", elapsed_time);
    println!("Total Memory used: {} KB", memory_used);

    Ok(())
}


fn track_memory_and_time<F>(operation_name: &str, operation: F) -> Result<(), String>
where
    F: FnOnce() -> Result<String, String>,
{
    let start_time = Instant::now();
    let mut system = System::new_all();
    system.refresh_all();
    let pid = sysinfo::get_current_pid().expect("Failed to get process ID");
    let initial_memory = system.process(pid).map_or(0, |process| process.memory());

    // Execute the operation
    let result = operation();

    // Measure final memory usage
    system.refresh_all();
    let final_memory = system.process(pid).map_or(0, |process| process.memory());

    // Calculate elapsed time and memory usage
    let elapsed_time = start_time.elapsed();
    let memory_used = final_memory.saturating_sub(initial_memory);

    // Print metrics for the operation
    println!("{} completed in: {:.2?}", operation_name, elapsed_time);
    println!("Memory used during {}: {} KB", operation_name, memory_used);

    result.map(|_| ())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Run the main logic asynchronously
    run_main(&args).await?;
    Ok(())
}

//test main goes here
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_main() {
        let args = Args {
            url: String::from("https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv"),
            file_path: String::from("data/drinks.csv"),
            action: String::from("read"), // Set default action to "read" for the test
        };

        let result = run_main(&args).await;
        println!("{:?}", result);
        assert!(result.is_ok(), "Expected Ok result, got Err: {:?}", result);
    }
}
