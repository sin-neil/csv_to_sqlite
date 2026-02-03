use anyhow::Result;
use clap::Parser;
use csv::ReaderBuilder;
use rusqlite::Connection;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "csv_to_sqlite")]
#[command(about = "Convert CSV files to SQLite database")]
struct Args {
    #[arg(help = "Input CSV file path")]
    input: PathBuf,
    
    #[arg(help = "Output SQLite database path")]
    output: PathBuf,
    
    #[arg(short, long, default_value = "data", help = "Table name in the database")]
    table: String,
    
    #[arg(long, help = "Automatically infer column types")]
    infer_types: bool,
}

fn infer_column_type(values: &[String]) -> String {
    let mut int_count = 0;
    let mut float_count = 0;
    let mut empty_count = 0;
    
    for value in values {
        if value.trim().is_empty() {
            empty_count += 1;
            continue;
        }
        
        if value.parse::<i64>().is_ok() {
            int_count += 1;
        } else if value.parse::<f64>().is_ok() {
            float_count += 1;
        }
    }
    
    let non_empty = values.len() - empty_count;
    if non_empty == 0 {
        return "TEXT".to_string();
    }
    
    if int_count as f64 / non_empty as f64 > 0.8 {
        "INTEGER".to_string()
    } else if (int_count + float_count) as f64 / non_empty as f64 > 0.8 {
        "REAL".to_string()
    } else {
        "TEXT".to_string()
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Reading CSV file: {}", args.input.display());
    
    // Read CSV file
    let file = File::open(&args.input)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    let headers = rdr.headers()?.clone();
    let mut records: Vec<Vec<String>> = Vec::new();
    
    for result in rdr.records() {
        let record = result?;
        records.push(record.iter().map(|s| s.to_string()).collect());
    }
    
    println!("Found {} columns and {} rows", headers.len(), records.len());
    
    // Determine column types
    let column_types: Vec<String> = if args.infer_types {
        println!("Inferring column types...");
        headers.iter().enumerate().map(|(i, _)| {
            let column_values: Vec<String> = records.iter()
                .filter_map(|row| row.get(i).cloned())
                .collect();
            infer_column_type(&column_values)
        }).collect()
    } else {
        vec!["TEXT".to_string(); headers.len()]
    };
    
    // Create SQLite database
    println!("Creating SQLite database: {}", args.output.display());
    let conn = Connection::open(&args.output)?;
    
    // Create table
    let create_sql = format!(
        "CREATE TABLE {} ({})",
        args.table,
        headers.iter().zip(column_types.iter())
            .map(|(header, col_type)| format!("\"{}\" {}", header, col_type))
            .collect::<Vec<_>>()
            .join(", ")
    );
    
    conn.execute(&create_sql, [])?;
    println!("Created table: {}", args.table);
    
    // Insert data
    let insert_sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        args.table,
        headers.iter().map(|h| format!("\"{}\"", h)).collect::<Vec<_>>().join(", "),
        (0..headers.len()).map(|_| "?").collect::<Vec<_>>().join(", ")
    );
    
    let tx = conn.transaction()?;
    let mut stmt = tx.prepare(&insert_sql)?;
    
    for (i, record) in records.iter().enumerate() {
        let params: Vec<&dyn rusqlite::ToSql> = record.iter()
            .map(|s| s as &dyn rusqlite::ToSql)
            .collect();
        
        stmt.execute(&*params)?;
        
        if (i + 1) % 1000 == 0 {
            println!("Inserted {} rows...", i + 1);
        }
    }
    
    tx.commit()?;
    
    println!("✅ Successfully converted CSV to SQLite!");
    println!("📊 Table: {} in {}", args.table, args.output.display());
    println!("📈 Rows inserted: {}", records.len());
    
    Ok(())
}