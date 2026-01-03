# tidesdb-rs

Rust wrapper for [TidesDB](https://github.com/tidesdb/tidesdb), a fast and efficient key-value storage engine library written in C.

**Note**: This repository uses [tidesdb](https://github.com/tidesdb/tidesdb) as a git submodule.

## Cloning with Submodules

When cloning this repository, use the `--recursive` flag to include the TidesDB submodule:

```bash
git clone --recursive https://github.com/yourusername/tidesdb-rs.git
```

If you've already cloned without the `--recursive` flag:

```bash
git submodule update --init --recursive
```

## Updating the Submodule

To update the TidesDB submodule to the latest version:

```bash
cd tidesdb
git fetch origin
git checkout origin/main
cd ..
git add tidesdb
git commit -m "Update tidesdb submodule"
```

## Features

- **ACID Transactions**: Full transactional support with MVCC and 5 isolation levels (READ_UNCOMMITTED, READ_COMMITTED, REPEATABLE_READ, SNAPSHOT, SERIALIZABLE)
- **Multi-Column Family**: Support for multiple column families with independent configurations
- **Compression**: LZ4, Zstd, Snappy, and Zlib compression algorithms
- **Bloom Filters**: Configurable bloom filters for efficient key existence checks
- **TTL Support**: Automatic key-value expiration with TTL
- **Thread-Safe**: Safe Rust API with Send + Sync traits for concurrent use
- **Memory Safety**: Proper resource management with RAII patterns

## Installation

### System Dependencies

You need to following C libraries installed on your system:

- `liblz4-dev` (or `lz4-devel` on some systems)
- `libzstd-dev` (or `zstd-devel`)
- `libsnappy-dev` (or `snappy-devel`)
- `zlib1g-dev` (or `zlib-devel`)

#### Ubuntu/Debian
```bash
sudo apt-get install liblz4-dev libzstd-dev libsnappy-dev zlib1g-dev
```

#### Fedora/RHEL
```bash
sudo dnf install lz4-devel zstd-devel snappy-devel zlib-devel
```

#### macOS
```bash
brew install lz4 zstd snappy
```

## Usage

### Basic Example

```rust
use tidesdb_rs::{Config, Database, ColumnFamilyConfig};

// Open or create a database
let config = Config::new("/path/to/db");
let db = Database::open(config)?;

// Create a column family
let cf_config = ColumnFamilyConfig::new();
db.create_column_family("my_cf", &cf_config)?;

// Get of column family
let cf = db.get_column_family("my_cf")?;

// Write data
let mut txn = db.begin_transaction()?;
txn.put(&cf, b"key1", b"value1")?;
txn.put(&cf, b"key2", b"value2")?;
txn.commit()?;

// Read data
let txn = db.begin_transaction()?;
let value = txn.get(&cf, b"key1")?;
assert_eq!(value, Some(b"value1".to_vec()));

# Ok::<(), Box<dyn std::error::Error>>(())
```

### Transactions with Isolation Levels

```rust
use tidesdb_rs::{IsolationLevel};

// Serializable isolation for strong consistency
let txn = db.begin_transaction_with_isolation(IsolationLevel::Serializable)?;
txn.put(&cf, b"account", b"balance:100")?;
txn.commit()?;
```

### Compression and Bloom Filters

```rust
use tidesdb_rs::{ColumnFamilyConfig, CompressionAlgorithm};

let cf_config = ColumnFamilyConfig::new()
    .with_compression(CompressionAlgorithm::Lz4)
    .with_bloom_filter(true, 0.01); // 1% false positive rate

db.create_column_family("compressed_cf", &cf_config)?;
```

### TTL Support

```rust
// Set a key that expires in 60 seconds
let mut txn = db.begin_transaction()?;
txn.put_with_ttl(&cf, b"temp_key", b"temp_value", 60)?;
txn.commit()?;
```

### Savepoints

```rust
let mut txn = db.begin_transaction()?;

txn.put(&cf, b"key1", b"value1")?;
txn.savepoint("checkpoint1")?;

txn.put(&cf, b"key2", b"value2")?;

// Rollback to checkpoint
txn.rollback_to_savepoint("checkpoint1")?;
txn.commit()?;
```

### Column Family Management

```rust
// List all column families
let cfs = db.list_column_families()?;
for cf_name in &cfs {
    println!("CF: {}", cf_name);
}

// Drop a column family
db.drop_column_family("old_cf")?;
```

### Manual Operations

```rust
// Manually trigger compaction
let cf = db.get_column_family("my_cf")?;
cf.compact()?;

// Manually flush memtable to disk
cf.flush()?;
```

## API Reference

### Main Types

- `Database` - Main database handle
- `ColumnFamily` - Column family handle
- `Transaction` - Transaction handle
- `Config` - Database configuration
- `ColumnFamilyConfig` - Column family configuration
- `IsolationLevel` - Transaction isolation levels
- `CompressionAlgorithm` - Compression algorithms
- `Error` - Error type

### Configuration Options

#### Database Config
- `new(path)` - Create config with database path
- `with_log_level(level)` - Set logging level
- `with_flush_threads(count)` - Set number of flush threads
- `with_compaction_threads(count)` - Set number of compaction threads
- `with_block_cache_size(size)` - Set block cache size
- `with_max_open_sstables(count)` - Set max open SSTables

#### Column Family Config
- `new()` - Create default config
- `with_compression(algo)` - Set compression algorithm
- `with_bloom_filter(enabled, fpr)` - Enable bloom filter with false positive rate
- `with_ttl(ttl)` - Set default TTL

## Error Handling

All functions return a `Result<T, Error>`. The `Error` type includes:

- `Memory` - Memory allocation failure
- `InvalidArgs` - Invalid arguments
- `NotFound` - Key not found
- `Io` - I/O error
- `Corruption` - Data corruption detected
- `Exists` - Resource already exists
- `Conflict` - Transaction conflict
- `TooLarge` - Value too large
- `MemoryLimit` - Memory limit exceeded
- `InvalidDb` - Invalid database state
- `Unknown` - Unknown error

## Safety

This crate provides safe Rust wrappers around TidesDB C API:

- **Memory Safety**: All C pointers are managed properly with RAII
- **Thread Safety**: Database and ColumnFamily implement Send + Sync
- **Error Handling**: All C errors are properly converted to Rust Result
- **Resource Cleanup**: Drop traits ensure proper cleanup of resources

## Performance Tips

1. **Use column families** to separate data with different access patterns
2. **Enable compression** for large values (LZ4 offers good speed/compression ratio)
3. **Use bloom filters** to reduce disk I/O for non-existent keys
4. **Batch operations** in transactions to reduce overhead
5. **Choose appropriate isolation level** - READ_COMMITTED is usually sufficient

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running Specific Tests

```bash
cargo test test_database_open
```

### Cloning for Development

When cloning for development, always use `--recursive`:

```bash
git clone --recursive https://github.com/yourusername/tidesdb-rs.git
cd tidesdb-rs
cargo build
```

Or if you already cloned without `--recursive`:

```bash
git submodule update --init --recursive
```

## License

This crate is licensed under a Mozilla Public License Version 2.0 (MPL-2.0), consistent with TidesDB C library.

The underlying TidesDB library uses multiple licenses:

- Mozilla Public License Version 2.0 (TidesDB)
- BSD 3 Clause (Snappy)
- BSD 2 (LZ4)
- BSD 2 (xxHash)
- BSD (Zstandard)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [TidesDB C Library](https://github.com/tidesdb/tidesdb)
- [TidesDB Documentation](https://tidesdb.com/)
- [TidesDB Discord](https://discord.gg/tWEmjR66cy)
