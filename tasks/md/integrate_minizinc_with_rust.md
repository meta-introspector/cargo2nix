description = "Develop Rust code within `cargo-git-manage` (or a new utility) to invoke MiniZinc models, pass input data, and parse the output for 'optimal placement' solutions."
steps = [
    "Identify the Rust crates suitable for executing external commands and handling process I/O (e.g., `std::process::Command`).",
    "Implement a Rust function that takes paths to a MiniZinc model and data file, executes MiniZinc, and captures its stdout/stderr.",
    "Develop parsing logic in Rust to extract the 'optimal placement' solution from MiniZinc's output format.",
    "Consider how input data for MiniZinc (e.g., representing the elliptic fiber, torus point, Monster stabilizer) would be generated or formatted by the Rust tooling."
]
