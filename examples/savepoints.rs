use std::fs;

use tidesdb_rs::{ColumnFamilyConfig, Config, Database};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define database path
    let db_path = "example_savepoints";
    let _ = fs::remove_dir_all(db_path);

    // Open database
    let config = Config::new(db_path)?;
    let db = Database::open(config)?;
    println!("Database opened\n");

    // Create column family
    let cf_config = ColumnFamilyConfig::new();
    db.create_column_family("game_state", &cf_config)?;
    let cf = db.get_column_family("game_state")?;
    println!("Column family created\n");

    // Example 1: Basic savepoint usage
    println!("--- Example 1: Basic Savepoint ---");
    let mut txn = db.begin_transaction()?;

    // Set initial state
    txn.put(&cf, b"score", b"0")?;
    txn.put(&cf, b"level", b"1")?;
    txn.put(&cf, b"coins", b"100")?;
    println!("  Initial state: score=0, level=1, coins=100");

    // Create savepoint
    txn.savepoint("checkpoint1")?;
    println!("  Created savepoint: checkpoint1\n");

    // Make changes
    txn.put(&cf, b"score", b"500")?;
    txn.put(&cf, b"level", b"2")?;
    txn.put(&cf, b"coins", b"50")?;
    println!("  After changes: score=500, level=2, coins=50");

    // Rollback to savepoint
    txn.rollback_to_savepoint("checkpoint1")?;
    println!("  Rolled back to checkpoint1\n");

    // Verify rollback
    let txn = db.begin_transaction()?;
    let score = txn.get(&cf, b"score")?;
    let level = txn.get(&cf, b"level")?;
    let coins = txn.get(&cf, b"coins")?;
    println!(
        "  Current state: score={:?}, level={:?}, coins={:?}",
        score.as_ref().map(|v| String::from_utf8_lossy(v.as_slice())),
        level.as_ref().map(|v| String::from_utf8_lossy(v.as_slice())),
        coins.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!();

    // Example 2: Multiple savepoints
    println!("--- Example 2: Multiple Savepoints ---");
    let mut txn = db.begin_transaction()?;

    // Initial state
    txn.put(&cf, b"position", b"0,0")?;
    txn.savepoint("start")?;
    println!("  Savepoint: start (position: 0,0)");

    // Move to checkpoint 1
    txn.put(&cf, b"position", b"10,5")?;
    txn.put(&cf, b"health", b"100")?;
    txn.savepoint("checkpoint_1")?;
    println!("  Savepoint: checkpoint_1 (position: 10,5, health: 100)");

    // Move to checkpoint 2
    txn.put(&cf, b"position", b"25,12")?;
    txn.put(&cf, b"health", b"75")?;
    txn.savepoint("checkpoint_2")?;
    println!("  Savepoint: checkpoint_2 (position: 25,12, health: 75)");

    // Move to checkpoint 3
    txn.put(&cf, b"position", b"40,8")?;
    txn.put(&cf, b"health", b"50")?;
    txn.savepoint("checkpoint_3")?;
    println!("  Savepoint: checkpoint_3 (position: 40,8, health: 50)");

    // Current position
    txn.put(&cf, b"position", b"50,15")?;
    txn.put(&cf, b"health", b"30")?;
    println!("  Current: position: 50,15, health: 30");

    // Rollback to checkpoint_2
    println!("\n  Rolling back to checkpoint_2...");
    txn.rollback_to_savepoint("checkpoint_2")?;

    // Verify
    let position = txn.get(&cf, b"position")?;
    let health = txn.get(&cf, b"health")?;
    println!(
        "  After rollback: position={:?}, health={:?}",
        position.as_ref().map(|v| String::from_utf8_lossy(v.as_slice())),
        health.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!();

    txn.commit()?;
    println!("Transaction committed\n");

    // Example 3: Savepoints with partial operations
    println!("--- Example 3: Complex Multi-Stage Operation ---");
    let mut txn = db.begin_transaction()?;

    // Stage 1: Initialize
    txn.put(&cf, b"stage", b"1")?;
    txn.put(&cf, b"data:1", b"value_1")?;
    txn.savepoint("stage_1_complete")?;
    println!("  Stage 1 complete, created savepoint");

    // Stage 2: Add more data
    txn.put(&cf, b"stage", b"2")?;
    txn.put(&cf, b"data:1", b"value_1_updated")?;
    txn.put(&cf, b"data:2", b"value_2")?;
    txn.savepoint("stage_2_complete")?;
    println!("  Stage 2 complete, created savepoint");

    // Stage 3: Finalize (but something goes wrong)
    txn.put(&cf, b"stage", b"3")?;
    txn.put(&cf, b"data:3", b"value_3")?;
    println!("  Stage 3 complete (simulated error)");

    // Rollback to stage 2
    println!("  Rolling back to stage_2...");
    txn.rollback_to_savepoint("stage_2_complete")?;

    // Continue from stage 2
    txn.put(&cf, b"recovery", b"true")?;
    txn.commit()?;
    println!("Transaction committed from stage 2\n");

    // Verify final state
    let txn2 = db.begin_transaction()?;
    let stage = txn2.get(&cf, b"stage")?;
    let data1 = txn2.get(&cf, b"data:1")?;
    let data2 = txn2.get(&cf, b"data:2")?;
    let data3 = txn2.get(&cf, b"data:3")?;
    let recovery = txn2.get(&cf, b"recovery")?;

    println!("  Final state:");
    println!(
        "    stage: {:?}",
        stage.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "    data:1: {:?}",
        data1.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "    data:2: {:?}",
        data2.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "    data:3: {:?} (should be None - rolled back)",
        data3.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "    recovery: {:?}",
        recovery.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!();

    // Example 4: Releasing savepoints
    println!("--- Example 4: Releasing Savepoints ---");
    let mut txn = db.begin_transaction()?;

    txn.put(&cf, b"temp", b"temporary_value")?;
    txn.savepoint("temp_savepoint")?;
    println!("  Created savepoint: temp_savepoint");

    // After no longer needed, release the savepoint
    txn.release_savepoint("temp_savepoint")?;
    println!("  Released savepoint: temp_savepoint");

    // Note: releasing a savepoint doesn't rollback, just removes the checkpoint
    txn.commit()?;
    println!("Transaction committed\n");

    // Example 5: Nested savepoints with selective rollback
    println!("--- Example 5: Selective Rollback Pattern ---");
    let mut txn = db.begin_transaction()?;

    // Operation A
    txn.put(&cf, b"op_a", b"result_a")?;
    txn.savepoint("after_a")?;
    println!("  Operation A complete");

    // Operation B
    txn.put(&cf, b"op_b", b"result_b")?;
    txn.savepoint("after_b")?;
    println!("  Operation B complete");

    // Operation C (fails)
    txn.put(&cf, b"op_c", b"result_c")?;
    println!("  Operation C complete (but we'll rollback)");

    // Rollback to after_b, keeping A and B
    println!("  Rolling back to after_b (keeps A and B)...");
    txn.rollback_to_savepoint("after_b")?;
    txn.commit()?;
    println!("Committed A and B, rolled back C\n");

    // Verify
    let txn3 = db.begin_transaction()?;
    let op_a = txn3.get(&cf, b"op_a")?;
    let op_b = txn3.get(&cf, b"op_b")?;
    let op_c = txn3.get(&cf, b"op_c")?;

    println!("  Final state:");
    println!(
        "    op_a: {:?} (should exist)",
        op_a.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "    op_b: {:?} (should exist)",
        op_b.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!(
        "    op_c: {:?} (should be None)",
        op_c.as_ref().map(|v| String::from_utf8_lossy(v.as_slice()))
    );
    println!();

    println!("Successfully demonstrated savepoint features");
    Ok(())
}
