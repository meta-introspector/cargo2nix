.PHONY: all build cargo2nix nix-build-with-cargo2nix clean generate-cargo-nix run-rust-src-scanner \
	build-cargo-llm-bootstrap build-hir-expand build-cargo-test-support build-prelude-generator \
	build-trait-fixer-hir-info-real nix-direct-build update.txt \
	nix-eval-cargo2nix-attrs nix-eval-flake-packages-attrs nix-eval-flake-root nix-flake-show \
	nix-eval-cargo2nix-attrs-json nix-eval-cargo2nix-raw-json build-submodule-tool build-gix-diff-minimal \
	process-repolist nix-cargo-build build-tracing-test build-hyper build-addr2line build-gix-merge \
	build-gix-attributes build-rustc-apfloat build-measureme build-rust-src-scanner \
	build-cargo-submodule-tool-lib build-addr2line-bin build-librocksdb-sys-debug
build-librocksdb-sys-debug:
	@echo "--- Debugging librocksdb-sys build environment ---"
	nix develop --command bash -c "echo LIBCLANG_PATH=$$LIBCLANG_PATH; echo LLVM_CONFIG=$$LLVM_CONFIG; echo LLVM_CONFIG_PATH=$$LLVM_CONFIG_PATH; echo PATH=$$PATH; RUSTC_BOOTSTRAP=1 cargo build -p librocksdb-sys --message-format=json 2>&1" \
	build-all-packages build-package-% report-build-status clean-build-logs main-report

# Default target
all: nix-direct-build

# Nix-related build targets
nix-direct-build:
	nix develop --command make build

nix-build-with-cargo2nix: generate-cargo-nix
	cargo build --message-format=json 2>&1

build:
	bash -c "RUSTC_BOOTSTRAP=1 cargo build --message-format=json 2>&1"

generate-cargo-nix:
	cargo update
	target/cargo2nix -o Cargo.nix

cargo2nix: nix-cargo-build
	target/debug/cargo2nix --overwrite

nix-cargo-build:
	RUSTC_BOOTSTRAP=1 cargo build --message-format=json 2>&1

main-report:
	@echo "--- Building main cargo2nix package and reporting errors (optimized) ---"
	@mkdir -p build_logs
	nix develop --command bash -c "RUSTC_BOOTSTRAP=1 cargo build -p cargo2nix --message-format=json 2>&1" > build_logs/cargo2nix_build_raw.log 2>&1 || true
	@echo "--- Errors for cargo2nix ---"
	@grep '"level":"error"' build_logs/cargo2nix_build_raw.log | \
	grep '"reason":"compiler-message"' | \
	sed 's/.*"message":{[^}]*"rendered":"\([^"]*\)".*/\1/' | \
	sed 's/\\n/\n/g' | \
	sed 's/\\t/\t/g' | \
	sed 's/\\"/"/g' | \
	sed 's/\\r//g' | \
	sed 's/\\//g' \
	> build_logs/cargo2nix_errors.log
	@if [ -s build_logs/cargo2nix_errors.log ]; then \
		echo "BUILD FAILED for cargo2nix. Detected errors:"; \
		cat build_logs/cargo2nix_errors.log; \
	else \
		echo "BUILD SUCCEEDED for cargo2nix. No compiler errors found."; \
	fi
	@echo "--- End of report for cargo2nix ---"

JQ_ERROR_FILTER = 'select(.reason == "compiler-message" and .message.level == "error") | .message.rendered'

# Define packages to build and test
PACKAGES = cargo2nix submodule-tool gix-diff gix-attributes rustc_apfloat measureme cargo-llm-bootstrap hir-expand cargo-test-support prelude-generator trait-fixer-hir-info-real tracing-test hyper addr2line gix-merge rust-src-scanner cargo-submodule-tool-lib addr2line-bin hir-ty

# New PHONY targets
.PHONY: build-all-packages build-package-% report-build-status clean-build-logs build-packages-in-error-from-last-run

# Target to build all defined packages and generate a report
build-all-packages: clean-build-logs $(foreach P,$(PACKAGES),build-package-$(P)) report-build-status

# Generic target to build a single package
build-package-%:
	@echo "--- Building package: $* ---"
	@mkdir -p build_logs
	nix develop --command bash -c "RUSTC_BOOTSTRAP=1 cargo build -p $* --message-format=json 2>&1" > build_logs/$*_build_raw.log 2>&1 || true
	@grep '"level":"error"' build_logs/$*_build_raw.log | \
	grep '"reason":"compiler-message"' | \
	sed 's/.*"message":{[^}]*"rendered":"\([^"]*\)".*/\1/' | \
	sed 's/\\n/\n/g' | \
	sed 's/\\t/\t/g' | \
	sed 's/\\"/"/g' | \
	sed 's/\\r//g' | \
	sed 's/\\//g' \
	> build_logs/$*_errors.log
	@if [ -s build_logs/$*_errors.log ]; then \
		echo "BUILD FAILED for $* (errors in build_logs/$*_errors.log)"; \
		echo "$* FAILED" >> build_logs/build_status.log; \
		cat build_logs/$*_errors.log; \
	else \
		echo "BUILD SUCCEEDED for $*"; \
		echo "$* SUCCEEDED" >> build_logs/build_status.log; \
	fi




# Target to report the build status of all packages
report-build-status:
	@echo ""
	@echo "--- Build Report ---"
	@if [ -f build_logs/build_status.log ]; then \
		cat build_logs/build_status.log; \
	else \
		echo "No build status log found. Run 'make build-all-packages' first."; \
	fi
	@echo "--------------------"
	@echo "Detailed logs in build_logs/ directory."

# Clean up build logs
clean-build-logs:
	@echo "Cleaning up build logs..."
	@rm -rf build_logs

# Target to rebuild only packages that failed in the previous build-all-packages run
build-packages-in-error-from-last-run:
	@echo "--- Rebuilding packages that failed in the last run ---"
	@if [ -f build_logs/build_status.log ]; then \
		FAILED_PACKAGES=$$(cat build_logs/build_status.log | grep "FAILED" | awk '{print $$1}'); \
		if [ -n "$$FAILED_PACKAGES" ]; then \
			for P in $$FAILED_PACKAGES; do \
				$(MAKE) build-package-$$P; \
			done; \
		else \
			echo "No failed packages found in build_logs/build_status.log from last run."; \
		fi; \
	else \
		echo "build_logs/build_status.log not found. Please run 'make build-all-packages' first."; \
	fi

# Utility targets
clean:
	cargo clean

update.txt:
	cargo update --recursive --verbose  > update.txt 2>&1

# Nix eval targets
nix-eval-cargo2nix-attrs:
	nix eval --raw --impure --expr 'builtins.attrNames (import ./flake.nix { }).packages.aarch64-linux.cargo2nix'

nix-eval-flake-packages-attrs:
	nix eval --raw --impure --expr '(import ./flake.nix { }).packages.aarch64-linux'

nix-eval-flake-root:
	nix eval --raw --impure --expr 'import ./flake.nix'

nix-flake-show:
	nix flake show

nix-eval-cargo2nix-attrs-json:
	nix eval --json --impure --expr 'builtins.attrNames (import ./flake.nix { }).packages.aarch64-linux.rustPkgs.workspace.cargo2nix'

nix-eval-cargo2nix-raw-json:
	nix eval --json --impure --expr '(import ./flake.nix { }).packages.aarch64-linux.rustPkgs.workspace.cargo2nix'

# Submodule related builds
build-submodule-tool:
	@echo "Building cargo-submodule-tool..."
	cd submodules/cargo/cargo-submodule-tool && nix develop ../../../flake-phase1.nix#default --command cargo build

build-gix-diff-minimal:
	RUSTC_BOOTSTRAP=1 cargo build -p gix-diff > gix_diff_build_log.txt 2>&1 || true

build-gix-attributes:
	@echo "Building gix-attributes..."
	nix develop --command cargo build -p gix-attributes

build-rustc-apfloat:
	@echo "Building rustc-apfloat..."
	nix develop --command cargo build -p rustc_apfloat

build-measureme:
	@echo "Building measureme..."
	nix develop --command cargo build -p measureme

build-cargo-llm-bootstrap:
	@echo "Building cargo-llm-bootstrap..."
	nix develop --command cargo build -p cargo-llm-bootstrap

build-hir-expand:
	@echo "Building hir-expand..."
	nix develop --command cargo build -p hir-expand

build-cargo-test-support:
	@echo "Building cargo-test-support..."
	nix develop --command cargo build -p cargo-test-support

build-prelude-generator:
	@echo "Building prelude-generator..."
	nix develop --command cargo build -p prelude-generator

build-trait-fixer-hir-info-real:
	@echo "Building trait-fixer-hir-info-real..."
	nix develop --command cargo build -p trait-fixer-hir-info-real

run-rust-src-scanner:
	@echo "Running rust-src-scanner..."
	cd tools/rust-src-scanner && cargo run --package rust-src-scanner -- \
		--rust-src-path /path/to/your/rust/source \
		--output-dir /path/to/your/output/directory

process-repolist:
	./process_repolist.sh repollist.json

test-hir-ty:
	RUSTC_BOOTSTRAP=1 cargo build -p hir-ty > hir-ty_build.log 2>&1 || true
	grep -E 'error(\[E[0-9]{4}\])?:' hir-ty_build.log > hir-ty_errors.log || true

build-tracing-test:
	nix develop --command cargo build -p tracing-test

build-hyper:
	nix develop --command cargo build -p hyper

build-addr2line:
	nix develop --command cargo build -p addr2line

build-gix-merge:
	nix develop --command cargo build -p gix-merge

build-rust-src-scanner:
	@echo "Building rust-src-scanner..."
	nix develop --command cargo build -p rust-src-scanner

build-cargo-submodule-tool-lib:
	@echo "Building cargo-submodule-tool-lib..."
	nix develop --command cargo build -p cargo-submodule-tool-lib

build-addr2line-bin:
	@echo "Building addr2line-bin..."
	nix develop --command cargo build -p addr2line-bin

JQ_ERROR_FILTER = 'select(.reason == "compiler-message" and .message.level == "error") | .message.rendered'
