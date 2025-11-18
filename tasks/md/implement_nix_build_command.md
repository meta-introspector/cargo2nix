description = "Create a new `NixBuildCommand` that implements `CargoCommand`. Its `needs_execution` should return `true` if `Cargo.nix` has changed or if the previous `nix build` failed (e.g., by checking for the existence and timestamp of a `result` symlink or a build log). Its `execute` method should perform a `nix build`."
status = "in-progress"
steps = [
    "Define `struct NixBuildCommand`.",
    "Implement `CargoCommand` for `NixBuildCommand`, checking `Cargo.nix` timestamp against build output.",
    "Implement `execute` to run `nix build`."
]
