# TidesDB Rust Examples

This directory contains example programs demonstrating various features of the `tidesdb-rs` library.

## How to Run Examples

### Prerequisites

Make sure you have the TidesDB system dependencies installed:

```bash
# Ubuntu/Debian
sudo apt-get install liblz4-dev libzstd-dev libsnappy-dev zlib1g-dev

# Fedora/RHEL
sudo dnf install lz4-devel zstd-devel snappy-devel zlib-devel

# macOS
brew install lz4 zstd snappy
```

### Running an Example

From the root of the repository (where `Cargo.toml` is located), run:

```bash
cargo run --example <example_name>
```

For example:

```bash
cargo run --example basic
```

### Building All Examples

To build all examples without running them:

```bash
cargo build --examples
```

### Building a Specific Example

```bash
cargo build --example <example_name>
```

## Examples

### 1. Basic Operations (`basic.rs`)

Demonstrates fundamental database operations:

- Opening a database
- Creating a column family
- Writing key-value pairs
- Reading values back
- Handling non-existent keys

**Run it:**

```bash
cargo run --example basic
```

### 2. Transactions (`transactions.rs`)

Shows transactional operations with different isolation levels:

- Read Committed (default) isolation
- Serializable isolation for strong consistency
- Read-modify-write pattern
- Transaction rollback
- Delete operations within transactions

**Run it:**

```bash
cargo run --example transactions
```

### 3. Multiple Column Families (`column_families.rs`)

Demonstrates using multiple column families:

- Creating column families with different configurations
- Column families with compression (LZ4, Zstd)
- Column families with bloom filters
- Writing to different column families
- Reading from multiple column families
- Column family isolation (same keys in different CFs)

**Run it:**

```bash
cargo run --example column_families
```

### 4. Time-to-Live (TTL) (`ttl.rs`)

Shows automatic key expiration:

- Setting keys with different TTLs
- Monitoring key expiration over time
- Refreshing TTL for existing keys
- Use case: Authentication tokens

**Note:** This example takes ~20 seconds to run as it waits for keys to expire.

**Run it:**

```bash
cargo run --example ttl
```

### 5. Transaction Savepoints (`savepoints.rs`)

Demonstrates savepoint functionality:

- Creating savepoints within transactions
- Rolling back to specific savepoints
- Multiple savepoints in one transaction
- Complex multi-stage operations
- Selective rollback patterns

**Run it:**

```bash
cargo run --example savepoints
```

## Cleanup

Each example creates its own database directory. To clean up:

```bash
# Remove example databases
rm -rf example_db example_transactions example_column_families example_ttl example_savepoints

# Or remove all directories starting with "example_"
rm -rf example_*
```
