description = "Update `CargoVendorCommand::needs_execution` to leverage Git state tracking. It should return `true` if `Cargo.lock` has changed and `vendor/` is out of date, or if any Git dependency (as determined by Git state tracking) has been updated upstream."
steps = [
    "Modify `CargoVendorCommand::needs_execution` to use the Git state tracking functions."
]