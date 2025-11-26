# Content-Addressable Cargo.toml Mapping Report

## Content Addressing Flow
git obj -> cargo.toml -> name and version -> content hash -> identical content across repos

## Identical Content Across Repositories

## Content Statistics
- Total unique content hashes: 24
- Identical content across repos: 0
- Unique content (single repo): 24
- Total Cargo.toml files: 24

## Most Duplicated Content

## Content-Addressable Storage Schema
```
Key: content_hash
Value: {
  name: string,
  version: string,
  locations: [{
    git_repo: string,
    git_object: string,
    cargo_toml_path: string,
    git_log: [string]
  }]
}
```
