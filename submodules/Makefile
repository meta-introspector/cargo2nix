SUBMODULES := cargo config-rs dlv-list-rs faster-hex gitoxide heapless ordered-multimap-rs ron-rs rust-base64 rust-ini time-rs

.PHONY: all $(SUBMODULES)

all: $(SUBMODULES)

$(SUBMODULES):
	@echo "Processing submodule: $@"
	@$(eval SUBMODULE_PATH := $(CURDIR)/$@)
	@$(eval CARGO2NIX_ROOT := $(realpath ..))

	@cd $(SUBMODULE_PATH) && \
	mkdir -p .cargo && \
	cp $(CARGO2NIX_ROOT)/.cargo/config.toml .cargo && \
	test -f .gitignore || touch .gitignore && \
	grep -qxF "vendor" .gitignore || echo "vendor" >> .gitignore && \
	test -f Cargo.toml || { echo "Error: Cargo.toml not found in $(SUBMODULE_PATH)"; exit 1; } && \
	if ! grep -q "^\\[workspace\\]" Cargo.toml; then \
		echo -e "\n[workspace]" >> Cargo.toml; \
	fi && \
	echo "--- Debugging before cargo vendor ---" && \
	pwd && \
	ls -l && \
	cargo vendor && \
	echo "--- Debugging before cargo2nix ---" && \
	pwd && \
	ls -l && \
	$(CARGO2NIX_ROOT)/target/debug/cargo2nix --overwrite && \
	cp $(CARGO2NIX_ROOT)/flake.nix.template flake.nix && \
	cp $(CARGO2NIX_ROOT)/Makefile.template Makefile && \
	if [ "$@" = "cargo" ]; then \
		sed -i 's/rustChannel = "nightly";/rustChannel = "stable";/' flake.nix && \
		sed -i 's/rustVersion = "latest";/rustVersion = "1.91.0";/' flake.nix && \
		sed -i "s|overlays = import ./overlay rust-overlay.overlays.default;|overlays = import ${CARGO2NIX_ROOT}/overlay rust-overlay.overlays.default;|" flake.nix; \
	fi && \
	$(MAKE) all CARGO2NIX_ROOT=$(CARGO2NIX_ROOT)

check-cargo-config:
	@echo "Checking .cargo/config.toml in submodules..."
	@$(eval CARGO2NIX_ROOT := $(realpath ..))
	@for x in $(SUBMODULES); do \
		echo "--- Checking $$x ---"; \
		if [ -f "$$x/.cargo/config.toml" ]; then \
			echo ".cargo/config.toml exists."; \
			DIFF=$$(diff "$$x/.cargo/config.toml" "$(CARGO2NIX_ROOT)/.cargo/config.toml"); \
			if [ -z "$$DIFF" ]; then \
				echo "Content matches the main project's config.toml."; \
			else \
				echo "Content DOES NOT match the main project's config.toml. Diff:"; \
				echo "$$DIFF"; \
			fi; \
		else \
			echo ".cargo/config.toml DOES NOT exist."; \
		fi; \
	done

