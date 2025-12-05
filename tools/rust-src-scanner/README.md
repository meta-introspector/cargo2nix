# rust-src-scanner

Advanced Rust source code analyzer with dependency graph generation and caching.

## Purpose
Comprehensive analysis of Rust codebases with focus on dependency relationships, complexity metrics, and build optimization.

## Features
- **Dependency Graph Generation**: Creates detailed dependency graphs
- **Caching System**: Efficient re-analysis with change detection
- **Syntax Analysis**: Uses `syn` for deep Rust code parsing
- **Hash-based Verification**: SHA2 hashing for integrity
- **Cargo.toml Parsing**: Full workspace analysis
- **Walkdir Integration**: Recursive directory traversal

## Dependencies
- `walkdir` - Directory traversal
- `syn` - Rust syntax parsing
- `petgraph` - Dependency graph construction
- `clap` - Command-line interface
- `toml` - Cargo.toml parsing
- `chrono` - Timestamp handling
- `sha2` - File integrity hashing

## Usage
```bash
cargo run -- --rust-src-path <path> --output-dir <output> --cache-path <cache>
```

## Output
- `dependency_graph.json` - Complete dependency graph
- `file_cache.json` - Cached analysis results
- Console progress and statistics

## Status
Production-ready scanner with comprehensive Rust analysis capabilities
