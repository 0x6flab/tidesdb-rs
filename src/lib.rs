//! TidesDB Rust Wrapper
//!
//! TidesDB is a fast and efficient key-value storage engine library written in C.
//! This crate provides safe Rust bindings to the TidesDB C API.
//!
//! # Features
//!
//! - ACID transactions with MVCC supporting 5 isolation levels
//! - Multi-column family support
//! - Compression (LZ4, Zstd, Snappy, Zlib)
//! - Bloom filters for efficient key existence checks
//! - TTL support for automatic key expiration
//! - Custom comparators
//!
//! # Example
//!
//! ```no_run
//!use std::fs;
//!use tidesdb_rs::{ColumnFamilyConfig, Config, Database};
//!
//!fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let db_path = "example_db";
//!    let _ = fs::remove_dir_all(db_path);
//!
//!    let config = Config::new(db_path)?;
//!    println!("Opening database at: {}", db_path);
//!    let db = Database::open(config)?;
//!    println!();
//!
//!    let cf_config = ColumnFamilyConfig::new();
//!    db.create_column_family("users", &cf_config)?;
//!    println!("Created column family: users");
//!    println!();
//!
//!    let cf = db.get_column_family("users")?;
//!    println!("Writing data...");
//!
//!    let mut txn = db.begin_transaction()?;
//!    txn.put(&cf, b"user:1", b"John Doe")?;
//!    txn.put(&cf, b"user:2", b"Jane Smith")?;
//!    txn.put(&cf, b"user:3", b"Bob Johnson")?;
//!    txn.commit()?;
//!
//!    println!("Inserted 3 users");
//!    println!();
//!    println!("Reading data...");
//!
//!    let txn = db.begin_transaction()?;
//!
//!    if let Some(value) = txn.get(&cf, b"user:1")? {
//!        println!("user:1 -> {}", String::from_utf8_lossy(&value));
//!    }
//!
//!    if let Some(value) = txn.get(&cf, b"user:2")? {
//!        println!("user:2 -> {}", String::from_utf8_lossy(&value));
//!    }
//!
//!    if let Some(value) = txn.get(&cf, b"user:3")? {
//!        println!("user:3 -> {}", String::from_utf8_lossy(&value));
//!    }
//!
//!    match txn.get(&cf, b"user:999")? {
//!        Some(_) => println!("user:999 -> Unexpectedly found!"),
//!        None => println!("user:999 -> Not found (expected)"),
//!    }
//!
//!    println!();
//!    println!("Successfully demonstrated basic operations");
//!    Ok(())
//!}
//! ```

pub mod error;
mod ffi;
mod tidesdb;

#[cfg(test)]
mod tests;

pub use error::{Error, Result};
pub use tidesdb::{
    ColumnFamily, ColumnFamilyConfig, CompressionAlgorithm, Config, Database, IsolationLevel,
    LogLevel, Transaction,
};
