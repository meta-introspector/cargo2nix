# Makefile for trait-fixer

export CFG_RELEASE := 1
# Enable bootstrap cfg and unstable library features for rustc components

# Default build target
.PHONY: all
all: build

# Build the project in release mode
.PHONY: build
build:
	@echo "Building trait-fixer in release mode..."
	RUSTC_BOOTSTRAP=1 CARGO_RUSTFLAGS="-Z unstable-options" cargo build --verbose

# Run the trait-fixer tool (example usage)
# You might need to adjust the path to the executable and arguments
.PHONY: run
run: build
	@echo "Running trait-fixer dry run..."
	./target/release/trait-fixer src/ --check

# Clean the build artifacts
.PHONY: clean
clean:
	@echo "Cleaning trait-fixer build artifacts..."
	cargo clean

# Phony targets to avoid conflicts with files of the same name
.PHONY: all build run clean
