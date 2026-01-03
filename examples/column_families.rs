use std::fs;

use tidesdb_rs::{ColumnFamilyConfig, Config, Database};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "example_column_families";
    let _ = fs::remove_dir_all(db_path);

    let config = Config::new(db_path)?;
    let db = Database::open(config)?;
    println!();

    println!("Creating column families...");

    let users_cf = ColumnFamilyConfig::new();
    db.create_column_family("users", &users_cf)?;
    println!("Created 'users' (no compression)");

    let products_cf =
        ColumnFamilyConfig::new().with_compression(tidesdb_rs::CompressionAlgorithm::LZ4);
    db.create_column_family("products", &products_cf)?;
    println!("Created 'products' (LZ4 compression)");

    let logs_cf = ColumnFamilyConfig::new().with_bloom_filter(true, 0.01);
    db.create_column_family("logs", &logs_cf)?;
    println!("Created 'logs' (bloom filter, 1% FPR)");

    let cache_cf = ColumnFamilyConfig::new()
        .with_compression(tidesdb_rs::CompressionAlgorithm::ZSTD)
        .with_bloom_filter(true, 0.001);
    db.create_column_family("cache", &cache_cf)?;
    println!("Created 'cache' (Zstd + bloom filter, 0.1% FPR)");
    println!();

    println!("Listing all column families");
    let cfs = db.list_column_families()?;
    for cf_name in &cfs {
        println!("  - {}", cf_name);
    }
    println!();

    println!("Writing to different column families");

    let users = db.get_column_family("users")?;
    let products = db.get_column_family("products")?;
    let logs = db.get_column_family("logs")?;
    let cache = db.get_column_family("cache")?;

    let mut txn = db.begin_transaction()?;

    txn.put(&users, b"user:1001", b"John Doe|john@example.com|active")?;
    txn.put(&users, b"user:1002", b"Jane Smith|jane@example.com|active")?;
    println!("Added 2 users to 'users' CF");

    txn.put(&products, b"product:sku001", b"Laptop|Dell|1299.99")?;
    txn.put(&products, b"product:sku002", b"Mouse|Logitech|29.99")?;
    txn.put(&products, b"product:sku003", b"Keyboard|Corsair|89.99")?;
    println!("Added 3 products to 'products' CF");

    txn.put(&logs, b"log:2024-01-01", b"INFO: System started")?;
    txn.put(&logs, b"log:2024-01-02", b"WARNING: High memory usage")?;
    println!("Added 2 log entries to 'logs' CF");

    txn.put(
        &cache,
        b"cache:session:123",
        b"session_data|expires_in_3600",
    )?;
    txn.put(&cache, b"cache:api:weather", b"temp:72,humidity:45")?;
    println!("Added 2 cache entries to 'cache' CF");

    txn.commit()?;
    println!();

    println!("Reading from different column families");
    let txn = db.begin_transaction()?;

    if let Some(user) = txn.get(&users, b"user:1001")? {
        println!("users CF: {}", String::from_utf8_lossy(&user));
    }

    if let Some(product) = txn.get(&products, b"product:sku002")? {
        println!("products CF: {}", String::from_utf8_lossy(&product));
    }

    if let Some(log) = txn.get(&logs, b"log:2024-01-02")? {
        println!("logs CF: {}", String::from_utf8_lossy(&log));
    }

    if let Some(cache) = txn.get(&cache, b"cache:api:weather")? {
        println!("cache CF: {}", String::from_utf8_lossy(&cache));
    }
    println!();

    println!("Demonstrating column family isolation");
    let mut txn = db.begin_transaction()?;

    txn.put(&users, b"shared_key", b"data_in_users")?;
    txn.put(&products, b"shared_key", b"data_in_products")?;
    txn.put(&cache, b"shared_key", b"data_in_cache")?;
    txn.commit()?;

    let txn2 = db.begin_transaction()?;

    let users_data = txn2.get(&users, b"shared_key")?;
    let products_data = txn2.get(&products, b"shared_key")?;
    let cache_data = txn2.get(&cache, b"shared_key")?;

    println!(
        "'shared_key' in users CF: {:?}",
        users_data
            .as_ref()
            .map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "'shared_key' in products CF: {:?}",
        products_data
            .as_ref()
            .map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "'shared_key' in cache CF: {:?}",
        cache_data
            .as_ref()
            .map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!();

    println!("Successfully demonstrated column family features");
    Ok(())
}
