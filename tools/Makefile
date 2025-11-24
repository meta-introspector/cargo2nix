RUST_SRC_PATH ?= /data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src
GRAPH_OUTPUT_PATH ?= ../cargo-llm-bootstrap/compilation_results/dependency_graph.json
CACHE_DIR ?= ../cargo-llm-bootstrap/compilation_results/cargo_cache

# Makefile for compiling cargo-submodule-tool-lib

RUSTC := rustc
CRATE_NAME := cargo_submodule_tool_lib
CRATE_ROOT := cargo-submodule-tool-lib/src/lib.rs
TARGET_DIR := target/debug/deps
LOG_DIR := feature_build_logs

# Flags for rustc, simplified from cargo build --verbose output
RUSTC_FLAGS := \
	--edition=2021 \
	--crate-type lib \
	--emit=dep-info,metadata,link \
	-C embed-bitcode=no \
	-C debuginfo=2 \
	--check-cfg 'cfg(docsrs,test)' \
	--out-dir $(TARGET_DIR)

.PHONY: all build clean diagnose-features compile-rust-src

all: build-llm-bootstrap

compile-rust-src:
	@echo "Compiling Rust source files from: $(RUST_SRC_PATH)"
	@find $(RUST_SRC_PATH) -name "*.rs" -print0 | while IFS= read -r -d '' file; do \
		echo "Compiling $$file..."; \
		rustc "$$file"; \
		if [ $$? -ne 0 ]; then \
			echo "Error compiling $$file"; \
		fi; \
	done

build: $(TARGET_DIR)/lib$(CRATE_NAME).rlib

build: $(TARGET_DIR)/lib$(CRATE_NAME).rlib

$(TARGET_DIR)/lib$(CRATE_NAME).rlib: $(CRATE_ROOT)
	@mkdir -p $(TARGET_DIR)
	$(RUSTC) --crate-name $(CRATE_NAME) $(CRATE_ROOT) $(RUSTC_FLAGS)

diagnose-features:
	@echo "Running feature diagnostic script..."
	@bash fix_rust_files.sh
	@echo ""
	@echo "--- Feature Diagnostic Summary ---"
	@for log_file in $(LOG_DIR)/*.log; do \
		feature_name=$$(basename $$log_file .log); \
		error_count=$$(grep -c "error[" $$log_file); \
		warning_count=$$(grep -c "warning[" $$log_file); \
		if [ $$error_count -gt 0 ]; then \
			echo "Feature $$feature_name: ERRORS: $$error_count, WARNINGS: $$warning_count"; \
		else \
			echo "Feature $$feature_name: OK (WARNINGS: $$warning_count)"; \
		fi; \
	done
	@echo "----------------------------------"

clean:
	@echo "Cleaning build artifacts..."
	@rm -rf target $(TEMP_DIR) $(LOG_DIR)

.PHONY: build-llm-bootstrap
build-llm-bootstrap:
	@echo "Running cargo-llm-bootstrap..."
	cd cargo-llm-bootstrap && cargo run --package cargo-llm-bootstrap -- --rust-src-path $(RUST_SRC_PATH)


.PHONY: nix-build-llm-bootstrap
nix-build-llm-bootstrap:
	@echo "Building cargo-llm-bootstrap using nix develop and cargo build..."
	cd cargo-llm-bootstrap && nix develop .#default --command cargo build --package cargo-llm-bootstrap

.PHONY: recompile-from-json
recompile-from-json:
	@if [ -z "$(JSON_FILE)" ]; then \
		echo "Error: JSON_FILE is not set. Usage: make recompile-from-json JSON_FILE=<path/to/result.json>"; \
		exit 1; \
	fi
	@echo "Recompiling from JSON file: $(JSON_FILE)"
	@RUST_FILE=$$(jq -r '.file_path' "$(JSON_FILE)"); \
	if [ -z "$$RUST_FILE" ]; then \
		echo "Error: Could not extract 'file_path' from $(JSON_FILE)"; \
		exit 1; \
	fi; \
	echo "Extracted Rust file: $$RUST_FILE"; \
	cd cargo-llm-bootstrap && cargo run --package cargo-llm-bootstrap -- "$$RUST_FILE" --rust-src-path $(RUST_SRC_PATH)

.PHONY: generate-state
generate-state:
	@echo "Generating initial state using rust-src-scanner..."
	cd rust-src-scanner && cargo run --package rust-src-scanner -- --rust-src-path $(RUST_SRC_PATH) --output-dir ../cargo-llm-bootstrap/compilation_results

.PHONY: run-rust-src-scanner
run-rust-src-scanner:
	@echo "Running rust-src-scanner..."
	cd rust-src-scanner && RUST_BACKTRACE=full cargo run --package rust-src-scanner -- \
		--rust-src-path $(RUST_SRC_PATH) \
		--output-dir ../cargo-llm-bootstrap/compilation_results \
		--cache-path ../cargo-llm-bootstrap/compilation_results/file_cache.json 2>&1 | tee rust-src-scanner.log

.PHONY: generate-build-plan
generate-build-plan: run-rust-src-scanner
	@echo "Generating build plan from dependency graph..."
	cd graph-petal-generator && cargo run -- \
		--graph-input-path ../cargo-llm-bootstrap/compilation_results/dependency_graph.json \
		--output-path ../cargo-llm-bootstrap/compilation_results/layered_graph.json \
		--root-crate rustc-main
	@echo "Build plan generated and layered graph saved to ../cargo-llm-bootstrap/compilation_results/layered_graph.json"

.PHONY: rust-solana-level-10
rust-solana-level-10:
	@echo "Running cargo-llm-bootstrap for level 10 (dry run)..."
	cd cargo-llm-bootstrap && cargo run --package cargo-llm-bootstrap -- \
		--rust-src-path $(RUST_SRC_PATH) \
		--level 10
