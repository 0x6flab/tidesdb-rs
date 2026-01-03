#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use libc::{c_char, c_int, c_void, size_t, time_t};

pub const TDB_SUCCESS: c_int = 0;
pub const TDB_ERR_MEMORY: c_int = -1;
pub const TDB_ERR_INVALID_ARGS: c_int = -2;
pub const TDB_ERR_NOT_FOUND: c_int = -3;
pub const TDB_ERR_IO: c_int = -4;
pub const TDB_ERR_CORRUPTION: c_int = -5;
pub const TDB_ERR_EXISTS: c_int = -6;
pub const TDB_ERR_CONFLICT: c_int = -7;
pub const TDB_ERR_TOO_LARGE: c_int = -8;
pub const TDB_ERR_MEMORY_LIMIT: c_int = -9;
pub const TDB_ERR_INVALID_DB: c_int = -10;
pub const TDB_ERR_UNKNOWN: c_int = -11;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tidesdb_log_level_t {
    TDB_LOG_DEBUG = 0,
    TDB_LOG_INFO = 1,
    TDB_LOG_WARN = 2,
    TDB_LOG_ERROR = 3,
    TDB_LOG_FATAL = 4,
    TDB_LOG_NONE = 99,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tidesdb_isolation_level_t {
    TDB_ISOLATION_READ_UNCOMMITTED = 0,
    TDB_ISOLATION_READ_COMMITTED = 1,
    TDB_ISOLATION_REPEATABLE_READ = 2,
    TDB_ISOLATION_SNAPSHOT = 3,
    TDB_ISOLATION_SERIALIZABLE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum compression_algorithm {
    TDB_COMPRESSION_NONE = 0,
    TDB_COMPRESSION_SNAPPY = 1,
    TDB_COMPRESSION_ZLIB = 2,
    TDB_COMPRESSION_ZSTD = 3,
    TDB_COMPRESSION_LZ4 = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tidesdb_config_t {
    pub db_path: *mut c_char,
    pub num_flush_threads: c_int,
    pub num_compaction_threads: c_int,
    pub log_level: tidesdb_log_level_t,
    pub block_cache_size: size_t,
    pub max_open_sstables: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tidesdb_column_family_config_t {
    pub write_buffer_size: size_t,
    pub level_size_ratio: size_t,
    pub min_levels: c_int,
    pub dividing_level_offset: c_int,
    pub klog_value_threshold: size_t,
    pub compression_algorithm: compression_algorithm,
    pub enable_bloom_filter: c_int,
    pub bloom_fpr: f64,
    pub enable_block_indexes: c_int,
    pub index_sample_ratio: c_int,
    pub block_index_prefix_len: c_int,
    pub sync_mode: c_int,
    pub sync_interval_us: u64,
    pub comparator_name: [c_char; 64],
    pub comparator_ctx_str: [c_char; 256],
    pub comparator_fn_cached: Option<skip_list_comparator_fn>,
    pub comparator_ctx_cached: *mut c_void,
    pub skip_list_max_level: c_int,
    pub skip_list_probability: f32,
    pub default_isolation_level: tidesdb_isolation_level_t,
    pub min_disk_space: u64,
    pub l1_file_count_trigger: c_int,
    pub l0_queue_stall_threshold: c_int,
}

#[repr(C)]
pub struct tidesdb_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tidesdb_column_family_t {
    pub name: *mut c_char,
    pub directory: *mut c_char,
    pub config: tidesdb_column_family_config_t,
    _private: [u8; 0],
}

#[repr(C)]
pub struct tidesdb_txn_t {
    _private: [u8; 0],
}

pub type skip_list_comparator_fn = Option<
    unsafe extern "C" fn(*const u8, size_t, *const u8, size_t, *mut c_void) -> c_int,
>;

extern "C" {
    pub fn tidesdb_default_column_family_config() -> tidesdb_column_family_config_t;
    pub fn tidesdb_default_config() -> tidesdb_config_t;

    pub fn tidesdb_open(config: *const tidesdb_config_t, db: *mut *mut tidesdb_t) -> c_int;
    pub fn tidesdb_close(db: *mut tidesdb_t) -> c_int;

    pub fn tidesdb_create_column_family(
        db: *mut tidesdb_t,
        name: *const c_char,
        config: *const tidesdb_column_family_config_t,
    ) -> c_int;

    pub fn tidesdb_drop_column_family(db: *mut tidesdb_t, name: *const c_char) -> c_int;

    pub fn tidesdb_get_column_family(
        db: *mut tidesdb_t,
        name: *const c_char,
    ) -> *mut tidesdb_column_family_t;

    pub fn tidesdb_list_column_families(
        db: *mut tidesdb_t,
        names: *mut *mut *mut c_char,
        count: *mut c_int,
    ) -> c_int;

    pub fn tidesdb_txn_begin(db: *mut tidesdb_t, txn: *mut *mut tidesdb_txn_t) -> c_int;

    pub fn tidesdb_txn_begin_with_isolation(
        db: *mut tidesdb_t,
        isolation: tidesdb_isolation_level_t,
        txn: *mut *mut tidesdb_txn_t,
    ) -> c_int;

    pub fn tidesdb_txn_put(
        txn: *mut tidesdb_txn_t,
        cf: *mut tidesdb_column_family_t,
        key: *const u8,
        key_size: size_t,
        value: *const u8,
        value_size: size_t,
        ttl: time_t,
    ) -> c_int;

    pub fn tidesdb_txn_get(
        txn: *mut tidesdb_txn_t,
        cf: *mut tidesdb_column_family_t,
        key: *const u8,
        key_size: size_t,
        value: *mut *mut u8,
        value_size: *mut size_t,
    ) -> c_int;

    pub fn tidesdb_txn_delete(
        txn: *mut tidesdb_txn_t,
        cf: *mut tidesdb_column_family_t,
        key: *const u8,
        key_size: size_t,
    ) -> c_int;

    pub fn tidesdb_txn_commit(txn: *mut tidesdb_txn_t) -> c_int;
    pub fn tidesdb_txn_rollback(txn: *mut tidesdb_txn_t) -> c_int;
    pub fn tidesdb_txn_free(txn: *mut tidesdb_txn_t);

    pub fn tidesdb_txn_savepoint(txn: *mut tidesdb_txn_t, name: *const c_char) -> c_int;
    pub fn tidesdb_txn_rollback_to_savepoint(txn: *mut tidesdb_txn_t, name: *const c_char)
        -> c_int;
    pub fn tidesdb_txn_release_savepoint(txn: *mut tidesdb_txn_t, name: *const c_char) -> c_int;

    pub fn tidesdb_compact(cf: *mut tidesdb_column_family_t) -> c_int;
    pub fn tidesdb_flush_memtable(cf: *mut tidesdb_column_family_t) -> c_int;

    pub fn tidesdb_register_comparator(
        db: *mut tidesdb_t,
        name: *const c_char,
        fn_: skip_list_comparator_fn,
        ctx_str: *const c_char,
        ctx: *mut c_void,
    ) -> c_int;

    pub fn tidesdb_get_comparator(
        db: *mut tidesdb_t,
        name: *const c_char,
        fn_: *mut skip_list_comparator_fn,
        ctx: *mut *mut c_void,
    ) -> c_int;
}
