.PHONY: all build cargo2nix nix-build clean

all: nix-build

build:
	cargo build

nix-cargo-build:
	nix develop --command cargo build

cargo2nix: build
	target/debug/cargo2nix --overwrite

nix-build:
	nix develop --command cargo build 

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
