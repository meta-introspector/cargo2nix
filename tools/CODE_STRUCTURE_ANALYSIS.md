# Code Structure Analysis Report

## Extended Pipeline Flow
rustc → crate → cargo metadata → repo → forks → branches → submodules → cargo.toml → package name → git object → **libs → decls → exports → signatures** → database entry → deps

## Code Structure Analysis

### diff
- **Git Object**: `36e19c9527fc66d1d2e9c6ba1615680d818447c4`
- **Libs**: 0 files
- **Declarations**: 1
  - `fn: main`
- **Exports**: 0
- **Signatures**: 1
  - `fn main()`

### dummy_crate
- **Git Object**: `unknown`
- **Libs**: 1 files
  - `dummy_crate/src/lib.rs`
- **Declarations**: 1
  - `fn: hello_dummy`
- **Exports**: 1
  - `pub fn hello_dummy`
- **Signatures**: 1
  - `fn hello_dummy() -> &'static str`

### BLAKE3
- **Git Object**: `eae9bf376a1c4797df7be6e49e735c0a5d91dcb0`
- **Libs**: 0 files
- **Declarations**: 94
  - `struct: RandomInput`
  - `fn: new`
  - `fn: get`
  - `fn: bench_single_compression_fn`
  - `fn: bench_single_compression_portable`
  - ... and 89 more
- **Exports**: 3
  - `pub struct RandomInput`
  - `pub fn new`
  - `pub fn get`
- **Signatures**: 93
  - `fn new(b: &mut Bencher, len: usize) -> Self`
  - `fn get(&mut self) -> &[u8]`
  - `fn bench_single_compression_fn(b: &mut Bencher, platform: Platform)`
  - ... and 90 more

### cast
- **Git Object**: `052288097de1846b938e854e27845a93a6f4b59d`
- **Libs**: 1 files
  - `cast/src/lib.rs`
- **Declarations**: 22
  - `enum: Error`
  - `fn: description_helper`
  - `fn: fmt`
  - `fn: description`
  - `trait: From`
  - ... and 17 more
- **Exports**: 3
  - `pub enum Error`
  - `pub trait From`
  - `pub fn $ty<T>`
- **Signatures**: 19
  - `fn description_helper(&self) -> &str`
  - `fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`
  - `fn description(&self) -> &str`
  - ... and 16 more

### juniper
- **Git Object**: `0e14736ad1acbb8a25f142ec426203c8797c4755`
- **Libs**: 0 files
- **Declarations**: 1
  - `fn: bench_sync_vs_async_users_flat_instant`
- **Exports**: 0
- **Signatures**: 1
  - `fn bench_sync_vs_async_users_flat_instant(c: &mut Criterion)`

## Code Structure Statistics
- Analyzed crates: 5
- Total lib files: 2
- Total declarations: 119
- Total exports: 7
- Total signatures: 115

## RocksDB Schema
```
Key: crate_name:git_object
Value: {
  libs: [lib_paths],
  decls: [declarations],
  exports: [public_items],
  signatures: [function_signatures],
  dependencies: [dep_list]
}
```
