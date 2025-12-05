# Binary: simple_ingest

## Description
This binary provides a simple ingestion mechanism for "Level 0 blocks" into a RocksDB instance. It processes content, calculates its SHA256 hash, and stores the hash, content, and file path as a `Level0Block` in the database. This tool is a basic building block for populating a content-addressable storage system.

## Usage
This is an executable tool. Its usage would typically involve running `cargo run -p rust-71-parts --bin simple_ingest` or `simple_ingest` after building.

## Dependencies
- `std::env`
- `std::fs`
- `rocksdb`
- `serde` (with `derive` feature)
- `sha2`

## Notes
The tool uses SHA256 hashing for content addressing, ensuring data integrity. It provides a straightforward way to load fundamental code or data blocks into a persistent store for further processing or analysis.