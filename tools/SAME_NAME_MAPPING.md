# Same Name Relationship Mapping Report

## Same Name Relationship Flow
git module - same name as - git module

## Same Name Groups

###  (2 modules)
#### Module 1 - rust
- **Path**: `submodules/rust`
- **URL**: `https://github.com/rust-lang/rust/`
- **Git Object**: `e9acbd99d384280874129fb7fa0da9faeae0d051`

#### Module 2 - tools/rust-bootstrap-nix
- **Path**: `tools/rust-bootstrap-nix`
- **URL**: `/data/data/com.termux.nix/files/home/rust-bootstrap-nix/`
- **Git Object**: `79cb33d7b62687dc9747af9529d4427e7f2ddcf7`

**Same Name Relationships**:
- `rust` - **same name as** - `tools/rust-bootstrap-nix`

### criterion.rs (2 modules)
#### Module 1 - criterion.rs
- **Path**: `submodules/criterion.rs`
- **URL**: `https://github.com/meta-introspector/criterion.rs`
- **Git Object**: `af5cc00ef1ad5e32b2d36a5be4d9cad8ed0c6ec9`

#### Module 2 - criterion
- **Path**: `submodules/criterion`
- **URL**: `https://github.com/meta-introspector/criterion.rs`
- **Git Object**: `-56de8c491634760910f8311911db26648c927f0c`

**Same Name Relationships**:
- `criterion.rs` - **same name as** - `criterion`

### eigenvalues (2 modules)
#### Module 1 - /data/data/com.termux.nix/files/home/experiments/pick-up-nix/vendor/nix/cargo2nix/tools/eigenvalues
- **Path**: `/data/data/com.termux.nix/files/home/experiments/pick-up-nix/vendor/nix/cargo2nix/tools/eigenvalues`
- **URL**: `https://github.com/meta-introspector/eigenvalues.git`
- **Git Object**: `unknown`

#### Module 2 - tools/eigenvalues
- **Path**: `tools/eigenvalues`
- **URL**: `https://github.com/meta-introspector/eigenvalues`
- **Git Object**: `9adc7603da14c5205a418574e31cdd21d926afa2`

**Same Name Relationships**:
- `/data/data/com.termux.nix/files/home/experiments/pick-up-nix/vendor/nix/cargo2nix/tools/eigenvalues` - **same name as** - `tools/eigenvalues`

## Statistics
- Total git modules: 537
- Unique names: 531
- Name collisions: 3
- Total name groups: 534
- Modules with name collisions: 6
- Average collision group size: 2.0

## Top Name Collision Groups
1. **** - 2 modules
2. **criterion.rs** - 2 modules
3. **eigenvalues** - 2 modules

## RocksDB Same Name Schema
```
Key: repo_name
Value: {
  name: string,
  modules: [{
    submodule_path: string,
    url: string,
    git_object: string
  }],
  relationship: "same_name_as"
}
```
