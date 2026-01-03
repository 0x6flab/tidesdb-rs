use std::fs;

use tidesdb_rs::{ColumnFamilyConfig, Config, Database, IsolationLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "example_transactions";
    let _ = fs::remove_dir_all(db_path);

    let config = Config::new(db_path)?;
    let db = Database::open(config)?;
    println!();

    let cf_config = ColumnFamilyConfig::new();
    db.create_column_family("accounts", &cf_config)?;
    let cf = db.get_column_family("accounts")?;

    println!("Example 1: Read Committed isolation");
    let mut txn = db.begin_transaction()?;
    txn.put(&cf, b"account:1", b"balance:1000")?;
    txn.put(&cf, b"account:2", b"balance:2000")?;
    txn.commit()?;
    println!();

    println!("Example 2: Serializable isolation");
    let mut txn = db.begin_transaction_with_isolation(IsolationLevel::SERIALIZABLE)?;
    txn.put(&cf, b"account:3", b"balance:3000")?;
    txn.put(&cf, b"account:4", b"balance:4000")?;
    txn.commit()?;
    println!();

    println!("Example 3: Read-Modify-Write pattern");
    let mut txn = db.begin_transaction()?;

    let new_balance = if let Some(bytes) = txn.get(&cf, b"account:1")? {
        let current_str = String::from_utf8_lossy(&bytes);
        if let Some(num_str) = current_str.strip_prefix("balance:") {
            if let Ok(num) = num_str.parse::<i32>() {
                format!("balance:{}", num + 500)
            } else {
                "balance:0".to_string()
            }
        } else {
            "balance:0".to_string()
        }
    } else {
        "balance:0".to_string()
    };

    txn.put(&cf, b"account:1", new_balance.as_bytes())?;
    txn.commit()?;
    println!("Updated account:1 to: {}", new_balance);
    println!();

    println!("Example 4: Transaction rollback");
    let mut txn = db.begin_transaction()?;
    txn.put(&cf, b"account:5", b"balance:5000")?;
    txn.put(&cf, b"account:6", b"balance:6000")?;
    txn.rollback()?;
    println!();

    let before = db.begin_transaction()?.get(&cf, b"account:5")?;
    println!(
        "In-transaction, account:5 = {:?}",
        before.as_ref().map(|v| String::from_utf8_lossy(&v))
    );

    let after = db.begin_transaction()?.get(&cf, b"account:5")?;
    println!(
        "After rollback, account:5 = {:?}",
        after.as_ref().map(|v| String::from_utf8_lossy(&v))
    );
    println!();

    println!("Example 5: Delete operation");
    let mut txn = db.begin_transaction()?;
    txn.delete(&cf, b"account:2")?;
    txn.commit()?;

    let deleted = db.begin_transaction()?.get(&cf, b"account:2")?;
    println!(
        "After deletion, account:2 = {:?}",
        deleted.as_ref().map(|v| String::from_utf8_lossy(&v))
    );
    println!();

    println!("Successfully demonstrated transaction features");
    Ok(())
}
