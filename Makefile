.PHONY: all build cargo2nix nix-build-with-cargo2nix clean generate-cargo-nix run-rust-src-scanner \
	build-cargo-llm-bootstrap build-hir-expand build-cargo-test-support build-prelude-generator \
	build-trait-fixer-hir-info-real nix-direct-build update.txt \
	nix-eval-cargo2nix-attrs nix-eval-flake-packages-attrs nix-eval-flake-root nix-flake-show \
	nix-eval-cargo2nix-attrs-json nix-eval-cargo2nix-raw-json build-submodule-tool build-gix-diff-minimal \
	process-repolist nix-cargo-build build-tracing-test build-hyper build-addr2line build-gix-merge \
	build-gix-attributes build-rustc-apfloat build-measureme build-rust-src-scanner \
	build-cargo-submodule-tool-lib build-addr2line-bin build-and-report

# Default target
all: nix-direct-build

# Nix-related build targets
nix-direct-build:
	nix develop --command make build

nix-build-with-cargo2nix: generate-cargo-nix
	cargo build --message-format=json 2>&1

build:
	nix develop --command bash -c "RUSTC_BOOTSTRAP=1 cargo build --message-format=json 2>&1"

generate-cargo-nix:
	cargo update
	target/cargo2nix -o Cargo.nix

cargo2nix: nix-cargo-build
	target/debug/cargo2nix --overwrite

nix-cargo-build:
	RUSTC_BOOTSTRAP=1 cargo build --message-format=json 2>&1

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

build-and-report:
	@echo "Building with Nix and generating report..."
	nix develop --command bash -c "RUSTC_BOOTSTRAP=1 cargo build --message-format=json 2>&1" > full_build_report.json
	jq -r $(JQ_ERROR_FILTER) full_build_report.json > build_errors.log
	@echo "Full build report saved to full_build_report.json"
	@echo "Error log saved to build_errors.log"
	@echo "Detected errors:"
	@cat build_errors.log