# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3](https://github.com/0x6flab/tidesdb-rs/compare/v0.1.2...v0.1.3) - 2026-05-04

### Added

- *(ffi)* add config fields and commit hook, include C sources

### Fixed

- *(tidesdb)* copy C buffer to Vec and free original
- *(tidesdb)* free db_path CString on Config drop

### Other

- *(cargo)* bump version to 0.1.3
- *(tidesdb)* add helper to copy and free C buffer
- *(deps)* bump the rs-dependencies group with 4 updates
- release v0.1.2

## [0.1.2](https://github.com/0x6flab/tidesdb-rs/releases/tag/v0.1.2) - 2026-01-03

### Fixed

- *(ci)* add dependecies when running release
- *(ci)* skip release when version unchanged

### Other

- update version
- fix version check in release workflow to exit gracefully
- *(ci)* add release workflows
- Complete TidesDB Rust wrapper implementation
- Initial commit: Add TidesDB Rust wrapper with git submodule
- Initial commit

## [0.1.1](https://github.com/0x6flab/tidesdb-rs/releases/tag/v0.1.1) - 2026-01-03

### Other

- fix version check in release workflow to exit gracefully
- *(ci)* add release workflows
- Complete TidesDB Rust wrapper implementation
- Initial commit: Add TidesDB Rust wrapper with git submodule
- Initial commit
