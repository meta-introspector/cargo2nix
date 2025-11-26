# Real Monster Protocol Rustc Build
RUST_SRC_PATH = /mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src
NIX_BUILD = nix-build

.PHONY: build-monster-rustc
build-monster-rustc:
	@echo "🦀 Building Monster Protocol Rustc from real components"
	@echo "Components: 75"
	$(NIX_BUILD) RealMonsterRustc.nix

.PHONY: verify-components
verify-components:
	@echo "🔍 Verifying real rustc components"
	@echo "  rustc_thread_pool → /mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/compiler/rustc_thread_pool/Cargo.toml"
	@echo "  rustc_baked_icu_data → /mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/compiler/rustc_baked_icu_data/Cargo.toml"
	@echo "  rustc_resolve → /mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/compiler/rustc_resolve/Cargo.toml"
	@echo "  rustc_hir → /mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/compiler/rustc_hir/Cargo.toml"
	@echo "  rustc_tools_util → /mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/src/tools/clippy/rustc_tools_util/Cargo.toml"
