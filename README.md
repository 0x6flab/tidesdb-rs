# tidesdb-rs

Rust wrapper for [TidesDB](https://github.com/tidesdb/tidesdb), a fast and efficient key-value storage engine library written in C.

**Note**: This repository uses [tidesdb](https://github.com/tidesdb/tidesdb) as a git submodule.

## Cloning with Submodules

When cloning this repository, use the `--recursive` flag to include the TidesDB submodule:

```bash
git clone --recursive https://github.com/0x6flab/tidesdb-rs.git
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
git checkout origin/master
cd ..
git add tidesdb
git commit -m "Update tidesdb submodule"
```

## Examples

Check out the [examples directory](./examples/) for comprehensive examples demonstrating various features:

- [basic.rs](./examples/basic.rs) - Simple put/get operations
- [transactions.rs](./examples/transactions.rs) - Different isolation levels and rollback
- [column_families.rs](./examples/column_families.rs) - Multiple column families
- [ttl.rs](./examples/ttl.rs) - Time-to-live and key expiration
- [savepoints.rs](./examples/savepoints.rs) - Transaction savepoints

### Running Examples

```bash
# Run a specific example
cargo run --example basic

# Build all examples
cargo build --examples

# See all available examples
ls examples/
```

For detailed instructions, see the [examples README](./examples/README.md).

## [Features](https://github.com/tidesdb/tidesdb#features)

The features of TidesDB-rs are the same as the [TidesDB C library](https://github.com/tidesdb/tidesdb#features).

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
use std::fs;
use tidesdb_rs::{ColumnFamilyConfig, Config, Database};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "example_db";
    let _ = fs::remove_dir_all(db_path);

    let config = Config::new(db_path)?;
    println!("Opening database at: {}", db_path);
    let db = Database::open(config)?;
    println!();

    let cf_config = ColumnFamilyConfig::new();
    db.create_column_family("users", &cf_config)?;
    println!("Created column family: users");
    println!();

    let cf = db.get_column_family("users")?;
    println!("Writing data...");

    let mut txn = db.begin_transaction()?;
    txn.put(&cf, b"user:1", b"John Doe")?;
    txn.put(&cf, b"user:2", b"Jane Smith")?;
    txn.put(&cf, b"user:3", b"Bob Johnson")?;
    txn.commit()?;

    println!("Inserted 3 users");
    println!();
    println!("Reading data...");

    let txn = db.begin_transaction()?;

    if let Some(value) = txn.get(&cf, b"user:1")? {
        println!("user:1 -> {}", String::from_utf8_lossy(&value));
    }

    if let Some(value) = txn.get(&cf, b"user:2")? {
        println!("user:2 -> {}", String::from_utf8_lossy(&value));
    }

    if let Some(value) = txn.get(&cf, b"user:3")? {
        println!("user:3 -> {}", String::from_utf8_lossy(&value));
    }

    match txn.get(&cf, b"user:999")? {
        Some(_) => println!("user:999 -> Unexpectedly found!"),
        None => println!("user:999 -> Not found (expected)"),
    }

    println!();
    println!("Successfully demonstrated basic operations");
    Ok(())
}
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

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [TidesDB C Library](https://github.com/tidesdb/tidesdb)
- [TidesDB Documentation](https://tidesdb.com/)
