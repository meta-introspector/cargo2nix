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

.PHONY: all build clean diagnose-features

all: build

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
	cd cargo-llm-bootstrap && cargo run --package cargo-llm-bootstrap


.PHONY: nix-build-llm-bootstrap
nix-build-llm-bootstrap:
	@echo "Building cargo-llm-bootstrap using nix develop and cargo build..."
	cd cargo-llm-bootstrap && nix develop .#default --command cargo build --package cargo-llm-bootstrap