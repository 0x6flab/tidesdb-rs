fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=tidesdb/src");

    let mut build = cc::Build::new();

    build
        .file("tidesdb/src/tidesdb.c")
        .file("tidesdb/src/block_manager.c")
        .file("tidesdb/src/bloom_filter.c")
        .file("tidesdb/src/buffer.c")
        .file("tidesdb/src/clock_cache.c")
        .file("tidesdb/src/compress.c")
        .file("tidesdb/src/manifest.c")
        .file("tidesdb/src/queue.c")
        .file("tidesdb/src/skip_list.c")
        .file("tidesdb/external/ini.c")
        .file("tidesdb/external/xxhash.c")
        .include("tidesdb/src")
        .include("tidesdb/external")
        .warnings(true);

    #[cfg(target_os = "linux")]
    {
        build.flag("-pthread");
    }

    build.compile("tidesdb");

    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=lz4");
    println!("cargo:rustc-link-lib=snappy");
    println!("cargo:rustc-link-lib=zstd");
}
