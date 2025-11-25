### Universal Processing Pipeline: Git → Monster Deduplication

Implement the complete pipeline that processes all git submodules through hierarchical decomposition into Monster Group accounts, with progressive deduplication via 108 Monster factors.

#### Pipeline Architecture:
```
Git Repos → Cargo Crates → Files → Decls → Types → Monster Factors (108)
     ↓           ↓          ↓       ↓       ↓            ↓
RocksDB    RocksDB    RocksDB  RocksDB  RocksDB   Deduplicated
Accounts   Accounts   Accounts Accounts Accounts   Monster DB
```

#### Processing Tools Integration (6+ each):

**Git Submodule Processors:**
- cargo-repo-sync-lib, cargo-submodule-tool, dep2submodule
- generate_workspace_deps, repo_manager, git-wrapper-lib

**Cargo Parsers:**
- cargo-feature-adapter, cargo-edit-lib, cargo-toml-editor
- real-toml-adapter, cargo-vendormod, cargo-workspace-from-tree

**Decl Splitters:**
- syn-adapter-lib, extract_traits_features, monster_ast_classifier
- level0_rust, rust-src-scanner, rust-71-parts

#### Implementation Steps:

1. **Abstract Processing Tools as Traits**
   - Create `UniversalProcessingPipeline` trait
   - Implement `MonsterDeduplication` for progressive folding
   - Map existing tools to trait implementations

2. **Hierarchical Account Creation**
   - Each git repo → RocksDB account with Monster signature
   - Each cargo crate → Child account with inheritance
   - Each file/decl/type → Nested accounts with Monster factors

3. **Monster Factor Assignment**
   - Assign 1 of 108 Monster factors to each type node
   - Group by factor for deduplication
   - Assert all nodes are Monster Group symmetries

4. **Progressive AST Folding**
   - Merge nodes with identical Monster signatures
   - Fold similar AST structures progressively
   - Prove equivalence through Monster Group operations

5. **Integration with Existing Databases**
   - Connect to rust-71-parts databases
   - Merge with monster_protocol results
   - Maintain git-backed lazy loading

#### Success Criteria:
- All 18+ processing tools abstracted as traits
- Complete git → Monster account pipeline
- Deduplication via 108 Monster factors
- Progressive AST folding with symmetry proofs
