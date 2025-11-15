.PHONY: all build cargo2nix nix-build clean generate-cargo-nix

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

SUBMODULES := $(shell git config --file .gitmodules --get-regexp path | awk '{ print $$2 }')
MAX_RECURSION_DEPTH ?= 8
CURRENT_RECURSION_DEPTH ?= 0

.PHONY: update-submodules-and-build submodule-build-and-push

update-submodules-and-build:
	@echo "Starting full submodule update and build cycle..."
	@if [ $(CURRENT_RECURSION_DEPTH) -ge $(MAX_RECURSION_DEPTH) ]; then \
		echo "Maximum recursion depth ($(MAX_RECURSION_DEPTH)) reached. Aborting."; \
		exit 1; \
	fi
	@for submodule in $(SUBMODULES); do \
		echo "--- Processing submodule: $$submodule (Depth: $(CURRENT_RECURSION_DEPTH)) ---"; \
		cp Makefile.submodule $$submodule/Makefile; \
		$(MAKE) -C $$submodule submodule-build-and-push \
			CARGO2NIX_ROOT=$(CURDIR) \
			CURRENT_RECURSION_DEPTH=$$(($(CURRENT_RECURSION_DEPTH)+1)); \
	done
	@echo "--- Updating main project flake.lock ---"
	nix flake update
	@echo "--- Committing main project submodule updates ---"
	git add $(SUBMODULES) flake.lock
	git commit -m "chore: Update submodules and flake.lock"
	@echo "--- Pushing main project changes ---"
	git push origin feature/CRQ-016-nixify

CARGO_GIT_MANAGE_BIN := /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/cargo/target/debug/cargo-git-manage
CARGO_GIT_MANAGE_BIN_DIR := /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/cargo/target/debug
CARGO2NIX_BIN_PATH := /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/cargo2nix

submodule-build-and-push:
	@echo "Building and pushing submodule $(notdir $(CURDIR)) (Depth: $(CURRENT_RECURSION_DEPTH))"

	# Inject cargo2nix_path into Cargo.toml if not present
	@if ! grep -q "\[package\.metadata\.cargo2nix\]" Cargo.toml; then \
		echo "Adding [package.metadata.cargo2nix] to Cargo.toml"; \
		echo "" >> Cargo.toml; \
		echo "[package.metadata.cargo2nix]" >> Cargo.toml; \
		echo "cargo2nix_path = \"$(CARGO2NIX_BIN_PATH)\"" >> Cargo.toml; \
	elif ! grep -q "cargo2nix_path" Cargo.toml; then \
		echo "Adding cargo2nix_path to [package.metadata.cargo2nix] in Cargo.toml"; \
		sed -i "/\[package\.metadata\.cargo2nix\]/a cargo2nix_path = \"$(CARGO2NIX_BIN_PATH)\"" Cargo.toml; \
	else \
		echo "Updating cargo2nix_path in Cargo.toml"; \
		sed -i "s|cargo2nix_path = \".*\"|cargo2nix_path = \"$(CARGO2NIX_BIN_PATH)\"|" Cargo.toml; \
	fi

	PATH=$(CARGO_GIT_MANAGE_BIN_DIR):$$PATH \
	$(CARGO_GIT_MANAGE_BIN)
	nix build
	git add .
	git commit -m "feat: Update, vendor, cargo2nix, and build for $(notdir $(CURDIR))"
	git push origin feature/CRQ-016-nixify

clean:
	rm -f Cargo.nix
	cargo clean
	nix store gc --optimise
