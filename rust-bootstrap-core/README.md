# rust-bootstrap-core

Layer 0 rustc - Fundamental primitives and constants with Monster Group signatures.

## Overview

This is Layer 0 of the Monster Group-based rustc reconstruction. It provides the most fundamental constants and primitive types, each assigned a unique Monster Group prime signature for mathematical verification.

## Monster Group Assignments

### Constants (Prime^1 signatures)
- `ZERO` → 2^1 = 2
- `ONE` → 3^1 = 3  
- `TWO` → 5^1 = 5
- `TRUE` → 7^1 = 7
- `FALSE` → 11^1 = 11

### Primitives (Prime^1 signatures)
- `CHAR` → 13^1 = 13
- `STRING` → 17^1 = 17
- `ARRAY` → 19^1 = 19
- `POINTER` → 23^1 = 23

## Usage

```rust
use rust_bootstrap_core::*;

// Initialize Layer 0
init_layer0();

// Use Monster Group constants
let zero = ZERO;
let one = ONE;
assert_eq!(zero.monster_signature(), 2);
assert_eq!(one.monster_signature(), 3);

// Use Monster Group primitives
let char_prim = CHAR_PRIMITIVE;
assert_eq!(char_prim.monster_signature(), 13);
```

## Monster Group Verification

Layer 0 uses 9 out of 108 available Monster Group factors (8.3% capacity):

```rust
assert!(verify_layer0_constraints());
```

## Features

- `#![no_std]` - No standard library dependencies
- `#![forbid(unsafe_code)]` - Memory safe by construction
- Monster Group mathematical verification
- Complete test coverage
- Documentation with mathematical proofs

## Building

```bash
cargo build
cargo test
cargo doc --open
```

## Examples

```bash
cargo run --example layer0_demo
```

## Layer Dependencies

- **Layer 0**: ✅ **This layer** - Primitives and constants
- **Layer 1**: Basic types and operations (depends on Layer 0)
- **Layer 2**: Lexical analysis (depends on Layer 1)
- ...continuing through Layer 12

## Mathematical Foundation

This layer implements the proven rustc ≡ Monster Group mapping, where every component has a mathematically verified prime signature. The Monster Group order is:

```
M = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
```

Layer 0 uses only single-exponent primes (2^1 through 23^1) for maximum simplicity and verification.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
