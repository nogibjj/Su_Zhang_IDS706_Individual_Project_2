//Defining extract, transform and load, and query functions
use tokio::task;
use reqwest::blocking::get;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use rusqlite::{params, Connection};


pub fn extract(url: &str, file_path: &str) -> Result<String, String> {
    // Create directories if they do not exist
    if let Some(parent) = Path::new(file_path).parent() {
        create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Use block_in_place for blocking operations
    task::block_in_place(|| {
        // Make a GET request to the specified URL
        let response = get(url).map_err(|e| format!("Failed to get the URL: {}", e))?;

        // Check if response status is valid
        if response.status().is_success() {
            // Save the content to the specified file path
            let mut file = File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;
            let content = response.bytes().map_err(|e| format!("Failed to read response body: {}", e))?;
            file.write_all(&content).map_err(|e| format!("Failed to write to file: {}", e))?;
            println!("File successfully downloaded to {}", file_path);
            Ok(file_path.to_string())
        } else {
            Err(format!("Failed to retrieve the file. Status Code: {}", response.status()))
        }
    })
}

pub fn load(dataset: &str) -> Result<String, String> {
    println!("Current working directory: {:?}", std::env::current_dir());

    // Run blocking operations within block_in_place
    task::block_in_place(|| {
        let mut rdr = csv::Reader::from_path(dataset).map_err(|e| format!("Failed to open CSV file: {}", e))?;
        
        let conn = rusqlite::Connection::open("Drinks.db").map_err(|e| format!("Failed to connect to database: {}", e))?;

        // Drop table if exists and create a new one
        conn.execute("DROP TABLE IF EXISTS Drinks", []).map_err(|e| format!("Failed to drop table: {}", e))?;
        conn.execute(
            "CREATE TABLE Drinks (country TEXT, beer_servings INTEGER, spirit_servings INTEGER, wine_servings INTEGER, total_litres_of_pure_alcohol REAL)",
            [],
        ).map_err(|e| format!("Failed to create table: {}", e))?;

        // Insert data into the database
        for result in rdr.records() {
            let record = result.map_err(|e| format!("Failed to read record: {}", e))?;
            conn.execute(
                "INSERT INTO Drinks (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol) VALUES (?, ?, ?, ?, ?)",
                rusqlite::params![&record[0], &record[1], &record[2], &record[3], &record[4]],
            ).map_err(|e| format!("Failed to insert record: {}", e))?;
        }

        Ok("Drinks.db".to_string())
    })
}

pub fn read_data() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("Drinks.db").map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Query data from the Drinks table
    let mut stmt = conn.prepare("SELECT * FROM Drinks").map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let _data_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, i32>(3)?,
            row.get::<_, f64>(4)?,
        ))
    }).map_err(|e| format!("Failed to query data: {}", e))?;

    Ok("Read Success".to_string())
}

pub fn create_data() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("Drinks.db").map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Insert a new record into the Drinks table
    conn.execute(
        "INSERT INTO Drinks (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol) VALUES (?, ?, ?, ?, ?)",
        params!["Country1", 90, 20, 16, 4.5],
    ).map_err(|e| format!("Failed to insert record: {}", e))?;

    // Query the newly inserted record
    let mut stmt = conn.prepare("SELECT * FROM Drinks WHERE country = ?").map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let rows = stmt.query_map(params!["Country1"], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, i32>(3)?,
            row.get::<_, f64>(4)?,
        ))
    }).map_err(|e| format!("Failed to query data: {}", e))?;

    for row in rows {
        println!("create: {:?}", row.map_err(|e| format!("Failed to read data: {}", e))?);
    }

    Ok("Create Success".to_string())
}

pub fn delete_data() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("Drinks.db").map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Delete certain rows from the Drinks table
    let rows_deleted = conn.execute("DELETE FROM Drinks WHERE country = ?", params!["Albania"]).map_err(|e| format!("Failed to delete record: {}", e))?;

    println!("rows deleted: {}", rows_deleted);
    Ok("Delete Success".to_string())
}

pub fn update_data() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("Drinks.db").map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Update certain rows in the Drinks table
    let rows_updated = conn.execute("UPDATE Drinks SET beer_servings = ? WHERE country = ?", params![100, "Yemen"]).map_err(|e| format!("Failed to update record: {}", e))?;

    println!("rows updated: {}", rows_updated);
    Ok("Update Success".to_string())
}


