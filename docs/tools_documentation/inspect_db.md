# Binary: inspect_db

## Description
This binary provides a generic tool for inspecting the contents of a RocksDB database. It takes a database path as an argument and allows for basic inspection of its key-value pairs, which is useful for debugging, auditing, or understanding the data stored by other tools that utilize RocksDB.

## Usage
This is an executable tool. It requires a database path as an argument.
Example: `cargo run -p rust-71-parts --bin inspect_db <db_path>` or `inspect_db <db_path>` after building.

## Dependencies
- `std::env`
- `rocksdb`

## Notes
The tool's basic functionality of iterating through a RocksDB instance makes it a versatile utility for examining various databases created within the project, especially those related to the Monster Protocol's data storage.