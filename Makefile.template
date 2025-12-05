.PHONY: all build nix-build nix-flake-build

all: nix-build

build:
	cargo build

nix-build:
	nix develop --command cargo build --override-input cargo2nix-root $(CARGO2NIX_ROOT)

nix-flake-build:
	nix build --override-input cargo2nix-root $(CARGO2NIX_ROOT)

clean:
	rm -f Cargo.nix
	cargo clean
	nix store gc --optimise