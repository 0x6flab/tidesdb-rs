use std::fs;
use std::thread;
use std::time::Duration;

use tidesdb_rs::{ColumnFamilyConfig, Config, Database};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "example_ttl";
    let _ = fs::remove_dir_all(db_path);

    let config = Config::new(db_path)?;
    let db = Database::open(config)?;
    println!();

    let cf_config = ColumnFamilyConfig::new();
    db.create_column_family("sessions", &cf_config)?;
    let cf = db.get_column_family("sessions")?;
    println!();

    println!("Example 1: Keys with different TTLs");
    let mut txn = db.begin_transaction()?;

    txn.put_with_ttl(&cf, b"session:guest", b"guest_session_data", 5)?;
    println!("Set session:guest (TTL: 5 seconds)");

    txn.put_with_ttl(&cf, b"session:temp", b"temp_session_data", 10)?;
    println!("Set session:temp (TTL: 10 seconds)");

    txn.put_with_ttl(&cf, b"session:premium", b"premium_session_data", 15)?;
    println!("Set session:premium (TTL: 15 seconds)");

    txn.put(&cf, b"session:admin", b"admin_session_data")?;
    println!("Set session:admin (no TTL - permanent)");

    txn.commit()?;
    println!();

    println!("Example 2: Immediate availability");
    let txn = db.begin_transaction()?;

    let sessions = [
        "session:guest",
        "session:temp",
        "session:premium",
        "session:admin",
    ];
    for session in &sessions {
        if let Some(value) = txn.get(&cf, session.as_bytes())? {
            println!("{} -> {}", session, String::from_utf8_lossy(&value));
        } else {
            println!("{} -> Not found", session);
        }
    }
    println!();

    println!("Example 3: Watching expiration");
    println!("Waiting for keys to expire...");
    println!();

    thread::sleep(Duration::from_secs(6));
    println!("After 6 seconds:");
    let txn = db.begin_transaction()?;
    for session in &sessions {
        if let Some(value) = txn.get(&cf, session.as_bytes())? {
            println!("{} -> {}", session, String::from_utf8_lossy(&value));
        } else {
            println!("{} -> Expired", session);
        }
    }
    println!();

    thread::sleep(Duration::from_secs(6));
    println!("After 12 seconds:");
    let txn = db.begin_transaction()?;
    for session in &sessions {
        if let Some(value) = txn.get(&cf, session.as_bytes())? {
            println!("{} -> {}", session, String::from_utf8_lossy(&value));
        } else {
            println!("{} -> Expired", session);
        }
    }
    println!();

    thread::sleep(Duration::from_secs(5));
    println!("After 17 seconds:");
    let txn = db.begin_transaction()?;
    for session in &sessions {
        if let Some(value) = txn.get(&cf, session.as_bytes())? {
            println!("{} -> {}", session, String::from_utf8_lossy(&value));
        } else {
            println!("{} -> Expired", session);
        }
    }
    println!();

    println!("Example 4: Refreshing TTL");
    println!("Creating a new session and refreshing its TTL");
    println!();

    let mut txn = db.begin_transaction()?;
    txn.put_with_ttl(&cf, b"session:refreshable", b"original_data", 3)?;
    txn.commit()?;
    println!("Created session:refreshable (TTL: 3 seconds)");

    thread::sleep(Duration::from_secs(2));
    let txn = db.begin_transaction()?;
    if let Some(value) = txn.get(&cf, b"session:refreshable")? {
        println!(
            "After 2s: Still exists -> {}",
            String::from_utf8_lossy(&value)
        );
    }
    println!();

    let mut txn2 = db.begin_transaction()?;
    txn2.put_with_ttl(&cf, b"session:refreshable", b"refreshed_data", 10)?;
    txn2.commit()?;
    println!("Refreshed session:refreshable (new TTL: 10 seconds)");
    println!();

    thread::sleep(Duration::from_secs(2));
    let txn3 = db.begin_transaction()?;
    if let Some(value) = txn3.get(&cf, b"session:refreshable")? {
        println!(
            "After 2s refresh: Still exists -> {}",
            String::from_utf8_lossy(&value)
        );
    } else {
        println!("After 2s refresh: Expired");
    }
    println!();

    println!("Example 5: Use case - Authentication tokens");
    let mut txn = db.begin_transaction()?;

    let token = format!("auth_token_{}", uuid::Uuid::new_v4());
    println!("Generated auth token: {}", token);

    txn.put_with_ttl(&cf, b"auth:active", token.as_bytes(), 30)?;
    txn.commit()?;
    println!("Token stored (TTL: 30 seconds)");
    println!();

    let txn = db.begin_transaction()?;
    if let Some(stored_token) = txn.get(&cf, b"auth:active")? {
        let stored_str = String::from_utf8_lossy(&stored_token);
        if stored_str == token.as_str() {
            println!("Token validation: Valid");
        } else {
            println!("Token validation: Invalid");
        }
    } else {
        println!("Token validation: Expired");
    }
    println!();

    println!("Successfully demonstrated TTL features");
    Ok(())
}
