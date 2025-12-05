name = "Implement fork and patch submodule logic"
description = """
Implement the core logic for 'cargo git-manage submodule fork-and-patch'. This involves:
1. Extracting repository URLs for crates in 'vendor/'.
2. Checking if repositories are submodules and cloning them if not.
3. Renaming remotes to 'upstream' and adding 'origin' to 'meta-introspector'.
4. Forking repositories to 'meta-introspector' using 'gh repo fork'.
5. Checking out the 'feature/CRQ-016-nixify' branch.
6. Updating 'Cargo.toml' dependencies to point to the forked repositories and branch.
"""
status = "done"
depends_on = ["define_cli_structure.toml"]
