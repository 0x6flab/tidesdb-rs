#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{ColumnFamilyConfig, CompressionAlgorithm, Config, Database, IsolationLevel};

    fn setup_test_db(name: &str) -> Database {
        let db_path = format!("/tmp/tidesdb_test_{}", name);
        let _ = fs::remove_dir_all(&db_path);

        let config = Config::new(&db_path).unwrap();
        Database::open(config).unwrap()
    }

    fn teardown_test_db(name: &str) {
        let db_path = format!("/tmp/tidesdb_test_{}", name);
        let _ = fs::remove_dir_all(&db_path);
    }

    #[test]
    fn test_database_open() {
        let _db = setup_test_db("open");
        teardown_test_db("open");
    }

    #[test]
    fn test_create_column_family() {
        let db = setup_test_db("create_cf");
        let config = ColumnFamilyConfig::new();
        db.create_column_family("test_cf", &config).unwrap();
        teardown_test_db("create_cf");
    }

    #[test]
    fn test_list_column_families() {
        let db = setup_test_db("list_cf");
        let config = ColumnFamilyConfig::new();
        db.create_column_family("cf1", &config).unwrap();
        db.create_column_family("cf2", &config).unwrap();

        let cfs = db.list_column_families().unwrap();
        assert!(cfs.contains(&"cf1".to_string()));
        assert!(cfs.contains(&"cf2".to_string()));

        teardown_test_db("list_cf");
    }

    #[test]
    fn test_put_get_delete() {
        let db = setup_test_db("put_get");
        let cf_config = ColumnFamilyConfig::new();
        db.create_column_family("test_cf", &cf_config).unwrap();
        let cf = db.get_column_family("test_cf").unwrap();

        let mut txn = db.begin_transaction().unwrap();

        txn.put(&cf, b"key1", b"value1").unwrap();
        txn.put(&cf, b"key2", b"value2").unwrap();

        txn.commit().unwrap();

        let txn2 = db.begin_transaction().unwrap();
        let value = txn2.get(&cf, b"key1").unwrap();
        assert_eq!(value, Some(b"value1".to_vec()));

        let value = txn2.get(&cf, b"key2").unwrap();
        assert_eq!(value, Some(b"value2".to_vec()));

        let value = txn2.get(&cf, b"key3").unwrap();
        assert_eq!(value, None);

        teardown_test_db("put_get");
    }

    #[test]
    fn test_delete() {
        let db = setup_test_db("delete");
        let cf_config = ColumnFamilyConfig::new();
        db.create_column_family("test_cf", &cf_config).unwrap();
        let cf = db.get_column_family("test_cf").unwrap();

        let mut txn = db.begin_transaction().unwrap();
        txn.put(&cf, b"key1", b"value1").unwrap();
        txn.commit().unwrap();

        let mut txn2 = db.begin_transaction().unwrap();
        let value = txn2.get(&cf, b"key1").unwrap();
        assert_eq!(value, Some(b"value1".to_vec()));

        txn2.delete(&cf, b"key1").unwrap();
        txn2.commit().unwrap();

        let txn3 = db.begin_transaction().unwrap();
        let value = txn3.get(&cf, b"key1").unwrap();
        assert_eq!(value, None);

        teardown_test_db("delete");
    }

    #[test]
    fn test_transaction_rollback() {
        let db = setup_test_db("rollback");
        let cf_config = ColumnFamilyConfig::new();
        db.create_column_family("test_cf", &cf_config).unwrap();
        let cf = db.get_column_family("test_cf").unwrap();

        let mut txn = db.begin_transaction().unwrap();
        txn.put(&cf, b"key1", b"value1").unwrap();
        txn.rollback().unwrap();

        let txn2 = db.begin_transaction().unwrap();
        let value = txn2.get(&cf, b"key1").unwrap();
        assert_eq!(value, None);

        teardown_test_db("rollback");
    }

    #[test]
    fn test_isolation_levels() {
        let db = setup_test_db("isolation");
        let cf_config = ColumnFamilyConfig::new();
        db.create_column_family("test_cf", &cf_config).unwrap();
        let cf = db.get_column_family("test_cf").unwrap();

        let mut txn = db
            .begin_transaction_with_isolation(IsolationLevel::Serializable)
            .unwrap();
        txn.put(&cf, b"key1", b"value1").unwrap();
        txn.commit().unwrap();

        teardown_test_db("isolation");
    }

    #[test]
    fn test_compression() {
        let db = setup_test_db("compression");
        let cf_config =
            ColumnFamilyConfig::new().with_compression(CompressionAlgorithm::Lz4);
        db.create_column_family("test_cf", &cf_config).unwrap();
        let cf = db.get_column_family("test_cf").unwrap();

        let mut txn = db.begin_transaction().unwrap();
        txn.put(&cf, b"key1", b"value1").unwrap();
        txn.commit().unwrap();

        teardown_test_db("compression");
    }

    #[test]
    fn test_bloom_filter() {
        let db = setup_test_db("bloom");
        let cf_config =
            ColumnFamilyConfig::new().with_bloom_filter(true, 0.01);
        db.create_column_family("test_cf", &cf_config).unwrap();

        teardown_test_db("bloom");
    }

    #[test]
    fn test_drop_column_family() {
        let db = setup_test_db("drop_cf");
        let cf_config = ColumnFamilyConfig::new();
        db.create_column_family("test_cf", &cf_config).unwrap();
        db.drop_column_family("test_cf").unwrap();

        let result = db.get_column_family("test_cf");
        assert!(result.is_err());

        teardown_test_db("drop_cf");
    }
}
