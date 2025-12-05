# Makefile for trait-fixer

# Export CFG_RELEASE for rustc_span and RUSTC_OVERRIDE_VERSION_STRING for rustc_hir
CFG_RELEASE_VALUE := 1.74.0
export CFG_RELEASE := $(strip $(CFG_RELEASE_VALUE))
export CFG_RELEASE_CHANNEL := dev
export RUSTC_INSTALL_BINDIR := $(shell rustc --print sysroot)/bin

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