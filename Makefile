
.PHONY: all build cargo2nix nix-build clean generate-cargo-nix run-rust-src-scanner

update.txt:
	cargo update --recursive --verbose  > update.txt 2>&1

all: nix-build

generate-cargo-nix:
	cargo update
	target/cargo2nix -o Cargo.nix

build:
	RUSTC_BOOTSTRAP=1 cargo build --message-format=json 2>&1

nix-cargo-build:
	RUSTC_BOOTSTRAP=1 cargo build --message-format=json 2>&1

cargo2nix: nix-cargo-build
	target/debug/cargo2nix --overwrite

nix-build: generate-cargo-nix
	cargo build --message-format=json 2>&1

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

.PHONY: build-submodule-tool
build-submodule-tool:
	@echo "Building cargo-submodule-tool..."
	cd submodules/cargo/cargo-submodule-tool && nix develop ../../../flake-phase1.nix#default --command cargo build



clean:
	rm -f Cargo.nix
	cargo clean
	nix store gc --optimise

.PHONY: run-rust-src-scanner
run-rust-src-scanner:
	@echo "Running rust-src-scanner..."
	cd tools/rust-src-scanner && cargo run --package rust-src-scanner -- \
		--rust-src-path /path/to/your/rust/source \
		--output-dir /path/to/your/output/directory



.PHONY: process-repolist
process-repolist:
	./process_repolist.sh repolist.json
