.PHONY: all build cargo2nix nix-build clean generate-cargo-nix run-rust-src-scanner

update.txt:
	cargo update --recursive --verbose  > update.txt 2>&1

all: nix-build

generate-cargo-nix:
	/nix/store/1x74bj4qh82967g90knam14sc51rqhfk-cargo-1.89.0-aarch64-unknown-linux-gnu/bin/cargo update
	target/cargo2nix -o Cargo.nix

mCargo.nix:
	~/nix/vendor/rust/cargo2nix/target/cargo2nix --overwrite # Corrected path for cargo2nix

build:
	nix develop ./flake-phase1.nix#default --command cargo build

nix-cargo-build:
	nix develop ./flake-phase1.nix#default --command cargo build

cargo2nix: nix-cargo-build
	target/debug/cargo2nix --overwrite

nix-build: generate-cargo-nix
	nix develop ./flake-phase1.nix#default --command cargo build 

run-nix-build: generate-cargo-nix #cargo2nix
	nix build -f full-flake.nix -vvv --trace-verbose  --show-trace --keep-build-log --keep-derivations  --keep-env-derivations --keep-failed --keep-going --keep-outputs 2>&1 | tee nixbuild.log

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
