# Plan: Optimize Git Status Cache

## Goal
Significantly reduce the time taken to determine Git status, especially for repositories with many submodules, by implementing a stat trait cache and a one-time snapshot mechanism. This will involve enhancing the `RollupLock` to store Git-specific metadata and modifying existing Git status logic to leverage this cache.

## Phase 1: Enhance `RollupLock` Schema and `FileSystemStat` for Git Metadata

### Step 1.1: Extend `FileMetadata` with Git Object Information
Modify `FileMetadata` in `src/fs_cache.rs` to include fields relevant to Git objects, such as:
- `git_object_hash: Option<String>`: Stores the SHA-1 hash of the Git object (blob or tree) if the path corresponds to a Git-tracked file or directory.
- `is_git_tracked: bool`: Indicates if the path is tracked by Git.

### Step 1.2: Update `RollupLock` to Store Git Metadata
Modify `RollupLock` in `src/repo_sync_lib/rollup_lock.rs` to:
- Update its `file_metadata_cache` to store the extended `FileMetadata`.
- Potentially add a new field, `git_tree_cache: HashMap<PathBuf, String>`, to store the tree hash for directories, allowing for quick comparison of directory contents.

### Step 1.3: Update `RealFileSystemStat` to Capture Git Object Hashes
Modify `RealFileSystemStat` in `src/fs_cache.rs` to:
- When `get_metadata` is called for a Git-tracked path, use `git2` (or `gitoxide` if integrated) to determine the Git object hash (blob for files, tree for directories).
- Set `git_object_hash` and `is_git_tracked` fields in the returned `FileMetadata`.

## Phase 2: Implement Snapshot Mechanism

### Step 2.1: Create a `GitSnapshot` Module
Create a new module, e.g., `src/repo_sync_lib/git_snapshot.rs`, with a function like `fn create_snapshot(repo_path: &Path, rollup_lock: Arc<Mutex<RollupLock>>) -> Result<()>`.

### Step 2.2: Traverse Repository and Submodules
Within `create_snapshot`:
- Use `git2` to traverse the main repository's working tree and index.
- For each file and directory, obtain its Git object hash (blob or tree).
- Recursively traverse all submodules, collecting their Git object hashes.
- For untracked files, use `RealFileSystemStat` to get their `FileMetadata`.

### Step 2.3: Populate `RollupLock` with Snapshot Data
Within `create_snapshot`:
- For each tracked file/directory, store its `FileMetadata` (including `git_object_hash`) in `rollup_lock.file_metadata_cache`.
- For each directory, store its tree hash in `rollup_lock.git_tree_cache`.
- Mark the `rollup.lock` as updated.

## Phase 3: Integrate Cache into Git Status Logic

### Step 3.1: Modify `run_submodule_status`
Update `run_submodule_status` in `src/repo_sync_lib/run_submodule_status.rs` to:
- Before performing any expensive Git operations, check the `RollupLock` for cached Git status information.
- If a valid, up-to-date snapshot exists, use it to quickly determine the status of files and submodules.
- If the cache is stale or incomplete, trigger `create_snapshot` to refresh it.

### Step 3.2: Implement Cache Invalidation Strategy
Define conditions under which the Git status cache in `RollupLock` becomes invalid and needs to be rebuilt:
- After any Git command that modifies the working tree, index, or HEAD (e.g., `git pull`, `git commit`, `git checkout`).
- After any file system changes detected by the file system monitor (from `implement_file_system_monitoring_and_hashing.md`).
- Potentially a time-based invalidation for very large repositories.

## Phase 4: Refinement and Testing

### Step 4.1: Unit and Integration Tests
Write comprehensive tests for:
- `FileMetadata` serialization/deserialization.
- `RealFileSystemStat`'s ability to capture Git object hashes.
- `create_snapshot` function.
- `run_submodule_status`'s use of the cache.

### Step 4.2: Performance Benchmarking
Measure the performance improvement of Git status operations with and without the cache.
