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
//! use tidesdb_rs::{Config, Database, ColumnFamilyConfig, IsolationLevel};
//!
//! let config = Config::new("/path/to/db");
//! let db = Database::open(config).unwrap();
//!
//! let cf_config = ColumnFamilyConfig::new();
//! db.create_column_family("my_cf", &cf_config).unwrap();
//!
//! let cf = db.get_column_family("my_cf").unwrap();
//!
//! let mut txn = db.begin_transaction().unwrap();
//! txn.put(&cf, b"key", b"value").unwrap();
//! txn.commit().unwrap();
//!
//! let txn2 = db.begin_transaction().unwrap();
//! let value = txn2.get(&cf, b"key").unwrap();
//! assert_eq!(value, Some(b"value".to_vec()));
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
