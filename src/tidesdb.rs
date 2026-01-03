use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;

use crate::error::{Error, Result};
use crate::ffi;

pub struct Config {
    inner: ffi::tidesdb_config_t,
}

impl Config {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path = CString::new(db_path.as_ref().to_str().ok_or(Error::InvalidArgs)?)?;
        Ok(Config {
            inner: ffi::tidesdb_config_t {
                db_path: db_path.into_raw(),
                num_flush_threads: 2,
                num_compaction_threads: 2,
                log_level: ffi::tidesdb_log_level_t::TDB_LOG_INFO,
                block_cache_size: 64 * 1024 * 1024,
                max_open_sstables: 512,
            },
        })
    }

    pub fn with_log_level(mut self, level: LogLevel) -> Self {
        self.inner.log_level = level.0;
        self
    }

    pub fn with_flush_threads(mut self, count: i32) -> Self {
        self.inner.num_flush_threads = count;
        self
    }

    pub fn with_compaction_threads(mut self, count: i32) -> Self {
        self.inner.num_compaction_threads = count;
        self
    }

    pub fn with_block_cache_size(mut self, size: usize) -> Self {
        self.inner.block_cache_size = size;
        self
    }

    pub fn with_max_open_sstables(mut self, count: usize) -> Self {
        self.inner.max_open_sstables = count;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new("tidesdb").unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogLevel(pub ffi::tidesdb_log_level_t);

impl LogLevel {
    pub const DEBUG: LogLevel = LogLevel(ffi::tidesdb_log_level_t::TDB_LOG_DEBUG);
    pub const INFO: LogLevel = LogLevel(ffi::tidesdb_log_level_t::TDB_LOG_INFO);
    pub const WARN: LogLevel = LogLevel(ffi::tidesdb_log_level_t::TDB_LOG_WARN);
    pub const ERROR: LogLevel = LogLevel(ffi::tidesdb_log_level_t::TDB_LOG_ERROR);
    pub const FATAL: LogLevel = LogLevel(ffi::tidesdb_log_level_t::TDB_LOG_FATAL);
    pub const NONE: LogLevel = LogLevel(ffi::tidesdb_log_level_t::TDB_LOG_NONE);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsolationLevel(pub ffi::tidesdb_isolation_level_t);

impl IsolationLevel {
    pub const READ_UNCOMMITTED: IsolationLevel =
        IsolationLevel(ffi::tidesdb_isolation_level_t::TDB_ISOLATION_READ_UNCOMMITTED);
    pub const READ_COMMITTED: IsolationLevel =
        IsolationLevel(ffi::tidesdb_isolation_level_t::TDB_ISOLATION_READ_COMMITTED);
    pub const REPEATABLE_READ: IsolationLevel =
        IsolationLevel(ffi::tidesdb_isolation_level_t::TDB_ISOLATION_REPEATABLE_READ);
    pub const SNAPSHOT: IsolationLevel =
        IsolationLevel(ffi::tidesdb_isolation_level_t::TDB_ISOLATION_SNAPSHOT);
    pub const SERIALIZABLE: IsolationLevel =
        IsolationLevel(ffi::tidesdb_isolation_level_t::TDB_ISOLATION_SERIALIZABLE);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionAlgorithm(pub i32);

impl CompressionAlgorithm {
    pub const NONE: CompressionAlgorithm =
        CompressionAlgorithm(ffi::compression_algorithm::TDB_COMPRESSION_NONE as i32);
    pub const SNAPPY: CompressionAlgorithm =
        CompressionAlgorithm(ffi::compression_algorithm::TDB_COMPRESSION_SNAPPY as i32);
    pub const ZLIB: CompressionAlgorithm =
        CompressionAlgorithm(ffi::compression_algorithm::TDB_COMPRESSION_ZLIB as i32);
    pub const ZSTD: CompressionAlgorithm =
        CompressionAlgorithm(ffi::compression_algorithm::TDB_COMPRESSION_ZSTD as i32);
    pub const LZ4: CompressionAlgorithm =
        CompressionAlgorithm(ffi::compression_algorithm::TDB_COMPRESSION_LZ4 as i32);
}

pub struct Database {
    inner: *mut ffi::tidesdb_t,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub fn open(config: Config) -> Result<Self> {
        let mut db_ptr = ptr::null_mut();
        let result = unsafe { ffi::tidesdb_open(&config.inner, &mut db_ptr) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(Database { inner: db_ptr })
    }

    pub fn get_column_family(&self, name: &str) -> Result<ColumnFamily> {
        let name = CString::new(name)?;
        let cf_ptr = unsafe { ffi::tidesdb_get_column_family(self.inner, name.as_ptr()) };

        if cf_ptr.is_null() {
            return Err(Error::NotFound);
        }

        Ok(ColumnFamily { inner: cf_ptr })
    }

    pub fn create_column_family(&self, name: &str, config: &ColumnFamilyConfig) -> Result<()> {
        let name = CString::new(name)?;
        let result =
            unsafe { ffi::tidesdb_create_column_family(self.inner, name.as_ptr(), &config.inner) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn list_column_families(&self) -> Result<Vec<String>> {
        let mut names_ptr = ptr::null_mut();
        let mut count = 0;
        let result =
            unsafe { ffi::tidesdb_list_column_families(self.inner, &mut names_ptr, &mut count) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        let names = unsafe {
            std::slice::from_raw_parts(names_ptr, count as usize)
                .iter()
                .map(|&ptr| {
                    let cstr = CStr::from_ptr(ptr);
                    cstr.to_string_lossy().into_owned()
                })
                .collect()
        };

        unsafe {
            libc::free(names_ptr as *mut libc::c_void);
        }

        Ok(names)
    }

    pub fn drop_column_family(&self, name: &str) -> Result<()> {
        let name = CString::new(name)?;
        let result = unsafe { ffi::tidesdb_drop_column_family(self.inner, name.as_ptr()) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn begin_transaction(&self) -> Result<Transaction> {
        self.begin_transaction_with_isolation(IsolationLevel::READ_COMMITTED)
    }

    pub fn begin_transaction_with_isolation(
        &self,
        isolation: IsolationLevel,
    ) -> Result<Transaction> {
        let mut txn_ptr = ptr::null_mut();
        let result =
            unsafe { ffi::tidesdb_txn_begin_with_isolation(self.inner, isolation.0, &mut txn_ptr) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(Transaction {
            inner: txn_ptr,
            committed: false,
        })
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi::tidesdb_close(self.inner);
            }
        }
    }
}

pub struct ColumnFamily {
    inner: *mut ffi::tidesdb_column_family_t,
}

unsafe impl Send for ColumnFamily {}
unsafe impl Sync for ColumnFamily {}

impl ColumnFamily {
    pub fn name(&self) -> String {
        unsafe {
            let name_ptr = (*self.inner).name;
            CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
        }
    }

    pub fn compact(&self) -> Result<()> {
        let result = unsafe { ffi::tidesdb_compact(self.inner) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        let result = unsafe { ffi::tidesdb_flush_memtable(self.inner) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }
}

pub struct ColumnFamilyConfig {
    inner: ffi::tidesdb_column_family_config_t,
}

impl ColumnFamilyConfig {
    pub fn new() -> Self {
        ColumnFamilyConfig {
            inner: unsafe { ffi::tidesdb_default_column_family_config() },
        }
    }

    pub fn with_compression(mut self, algorithm: CompressionAlgorithm) -> Self {
        self.inner.compression_algorithm =
            unsafe { std::mem::transmute::<i32, ffi::compression_algorithm>(algorithm.0) };
        self
    }

    pub fn with_bloom_filter(mut self, enabled: bool, false_positive_rate: f64) -> Self {
        self.inner.enable_bloom_filter = if enabled { 1 } else { 0 };
        self.inner.bloom_fpr = false_positive_rate;
        self
    }

    pub fn with_ttl(mut self, ttl: u64) -> Self {
        self.inner.klog_value_threshold = ttl as usize;
        self
    }
}

impl Default for ColumnFamilyConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Transaction {
    inner: *mut ffi::tidesdb_txn_t,
    committed: bool,
}

unsafe impl Send for Transaction {}

impl Transaction {
    pub fn put(&mut self, cf: &ColumnFamily, key: &[u8], value: &[u8]) -> Result<()> {
        let result = unsafe {
            ffi::tidesdb_txn_put(
                self.inner,
                cf.inner,
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
                0,
            )
        };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn put_with_ttl(
        &mut self,
        cf: &ColumnFamily,
        key: &[u8],
        value: &[u8],
        ttl: u64,
    ) -> Result<()> {
        let result = unsafe {
            ffi::tidesdb_txn_put(
                self.inner,
                cf.inner,
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
                ttl as i64,
            )
        };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn get(&self, cf: &ColumnFamily, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let mut value_ptr = ptr::null_mut();
        let mut value_size = 0;

        let result = unsafe {
            ffi::tidesdb_txn_get(
                self.inner,
                cf.inner,
                key.as_ptr(),
                key.len(),
                &mut value_ptr,
                &mut value_size,
            )
        };

        if result == ffi::TDB_ERR_NOT_FOUND {
            return Ok(None);
        }

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        let value = unsafe { Vec::from_raw_parts(value_ptr, value_size, value_size) };
        Ok(Some(value))
    }

    pub fn delete(&mut self, cf: &ColumnFamily, key: &[u8]) -> Result<()> {
        let result =
            unsafe { ffi::tidesdb_txn_delete(self.inner, cf.inner, key.as_ptr(), key.len()) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn commit(mut self) -> Result<()> {
        let result = unsafe { ffi::tidesdb_txn_commit(self.inner) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        self.committed = true;
        Ok(())
    }

    pub fn rollback(self) -> Result<()> {
        let result = unsafe { ffi::tidesdb_txn_rollback(self.inner) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn savepoint(&mut self, name: &str) -> Result<()> {
        let name = CString::new(name)?;
        let result = unsafe { ffi::tidesdb_txn_savepoint(self.inner, name.as_ptr()) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn rollback_to_savepoint(&mut self, name: &str) -> Result<()> {
        let name = CString::new(name)?;
        let result = unsafe { ffi::tidesdb_txn_rollback_to_savepoint(self.inner, name.as_ptr()) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }

    pub fn release_savepoint(&mut self, name: &str) -> Result<()> {
        let name = CString::new(name)?;
        let result = unsafe { ffi::tidesdb_txn_release_savepoint(self.inner, name.as_ptr()) };

        if result != ffi::TDB_SUCCESS {
            return Err(Error::from_code(result));
        }

        Ok(())
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if !self.inner.is_null() && !self.committed {
            unsafe {
                ffi::tidesdb_txn_free(self.inner);
            }
        }
    }
}
