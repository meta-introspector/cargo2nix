description = "Create a new `GitPullCommand` that implements `CargoCommand`. Its `needs_execution` should return `true` if the remote HEAD is ahead of the local HEAD. Its `execute` method should perform a `git pull`."
steps = [
    "Define `struct GitPullCommand`.",
    "Implement `CargoCommand` for `GitPullCommand`, using Git state tracking functions for `needs_execution`.",
    "Implement `execute` to run `git pull`."
]
