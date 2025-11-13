.PHONY: all build cargo2nix nix-build clean

all: nix-build

mCargo.nix:
	~/nix/vendor/rust/cargo2nix/target/cargo2nix --overwrite # Corrected path for cargo2nix

build:
	nix develop ./flake-phase1.nix#default --command cargo build

nix-cargo-build:
	nix develop ./flake-phase1.nix#default --command cargo build

cargo2nix: nix-cargo-build
	target/debug/cargo2nix --overwrite

nix-build:
	nix develop ./flake-phase1.nix#default --command cargo build 

run-nix-build: #cargo2nix
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

clean:
	rm -f Cargo.nix
	cargo clean
	nix store gc --optimise
