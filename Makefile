.PHONY: all build cargo2nix nix-build clean

all: nix-build

build:
	cargo build

cargo2nix: build
	target/debug/cargo2nix --overwrite

nix-build: cargo2nix
	nix build

clean:
	rm -f Cargo.nix
	cargo clean
	nix store gc --optimise
