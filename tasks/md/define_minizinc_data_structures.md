description = "Define Rust data structures that can represent the inputs required by MiniZinc models (e.g., parameters related to elliptic fibers, torus points, Monster stabilizers) and the outputs produced by MiniZinc (e.g., optimal placement solutions)."
steps = [
    "Analyze the existing MiniZinc models to understand their input data format (`.dzn`).",
    "Create Rust structs that mirror these input data structures.",
    "Implement serialization logic in Rust to convert Rust data structures into `.dzn` format.",
    "Analyze MiniZinc's output format and define Rust structs to represent the parsed solutions.",
    "Implement deserialization logic in Rust to convert MiniZinc output into Rust data structures."
]
