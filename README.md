# CSV to SQLite Converter

A high-performance Rust CLI tool that converts CSV files to SQLite databases with automatic type inference.

## Features

- Fast CSV parsing and SQLite insertion
- Smart type inference (INTEGER, REAL, TEXT)
- Transaction-based bulk inserts
- Memory safe with Rust's type system
- Professional CLI interface

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Basic conversion
csv_to_sqlite input.csv output.db --table users

# With automatic type inference
csv_to_sqlite input.csv output.db --table users --infer-types

# Custom table name
csv_to_sqlite data.csv database.db --table employees --infer-types
```

## Example

Convert this CSV:
```csv
name,age,city,salary
John Doe,28,New York,75000
Jane Smith,32,San Francisco,95000
```

To a SQLite table with inferred types:
```sql
CREATE TABLE users (
    "name" TEXT,
    "age" INTEGER, 
    "city" TEXT,
    "salary" REAL
)
```

## Tech Stack

- **Rust** - Systems programming language
- **csv** - CSV parsing library
- **rusqlite** - SQLite database bindings
- **clap** - CLI argument parsing
- **anyhow** - Error handling

## Performance

- Handles large CSV files efficiently
- Uses database transactions for bulk inserts
- Memory-efficient streaming parser

## License

MIT
