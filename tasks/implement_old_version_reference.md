name = "Implement Old Version Reference in Nix Bootstrap"
description = """
Implement a mechanism to "ref the old version from this version" within the Nix self-hosting bootstrap.
This involves:
1. Defining how the "old version" (e.g., Git commit hash, tag) is specified (e.g., CLI argument, configuration).
2. Modifying `flake.nix` (or a template) to include an input that references this old version.
3. Integrating this reference into the Nix build process, potentially using it to fetch or build the old version.
"""
status = "todo"
depends_on = ["implement_nix_build_command.toml"]