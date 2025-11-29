Here's a complete, practical guide to build a **fully instrumented Rust compiler (rustc)** using **Nix** — including coverage instrumentation (`-C instrument-coverage`), profile-guided optimization instrumentation (`-C profile-generate`), LLVM line tables, and debug info — all in a reproducible and pure way.

### Goal
Build `rustc` (stage2) with:
- Full debug info
- LLVM coverage instrumentation (`-C instrument-coverage`)
- Optional: PGO instrumentation (`-C profile-generate`)
- Optional: Sanitizers (AddressSanitizer, etc.)
- Everything done inside Nix (pure, reproducible, cachable)

### 1. Directory layout
```
rustc-instrumented/
├── flake.nix
└── rust-toolchain.toml   # optional, for rustup users
```

### 2. flake.nix (Recommended in 2025)

```nix
{
  description = "Instrumented Rust compiler (rustc) with coverage and PGO";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Use fenix to get exact rustc source matching a known nightly
        rustToolchain = fenix.packages.${system}.complete.withComponents [
          "cargo"
          "rustc"
          "rust-src"      # needed to build rustc
          "llvm-tools"    # for llvm-profdata, llvm-cov
          "rustfmt"
          "clippy"
        ];

        # Or pin to a specific nightly that supports instrument-coverage well
        nightly = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # replace
        };

        # Rust bootstrap (stable rustc + cargo) – needed to build rustc itself
        bootstrapRust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        # Custom rustc build with full instrumentation
        instrumentedRustc = pkgs.stdenv.mkDerivation {
          pname = "rustc-instrumented";
          version = "nightly-2025-11-29"; # change to today's nightly

          src = pkgs.fetchFromGitHub {
            owner = "rust-lang";
            repo = "rust";
            # Use the exact same commit as your nightly toolchain
            rev = "f4e9e6f00e3e5c8a6d8f3e8b8d7d6f5e4c3b2a19"; # ← REPLACE with actual nightly commit
            hash = "sha256-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX="; # ← REPLACE
          };

          nativeBuildInputs = [
            bootstrapRust
            pkgs.cmake
            pkgs.python3
            pkgs.libiconv
            pkgs.git
            pkgs.openssl
            pkgs.pkg-config
            pkgs.libgit2
            pkgs.curl
            pkgs.zlib
            pkgs.llvmPackages.llvm
            pkgs.llvmPackages.clang
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreServices
          ];

          buildInputs = [ pkgs.openssl pkgs.zlib pkgs.libgit2 pkgs.curl ];

          # THIS IS THE KEY PART — rustc's own build config
          configurePhase = ''
            mkdir -p build
            cat > config.toml <<EOF
            [llvm]
            download-ci-llvm = false
            link-shared = true

            [build]
            vendor = true
            # makes build pure
            extended = true              # builds rustdoc, clippy, etc.
            tools = ["cargo", "rustdoc", "clippy", "rustfmt"]

            [rust]
            debug = true                 # full debug info
            debug-logging = true
            channel = "nightly"
            # Enable coverage instrumentation in the compiler itself
            codegen-units = 1            # better coverage
            incremental = false

            # Instrument the compiler with LLVM coverage
            rustflags = ["-C", "instrument-coverage", "-C", "link-dead-code", "-C", "debuginfo=2"]

            # Optional: PGO instrumentation
            # profile-generate = "/tmp/pgo-data"
            # profile-use = "/tmp/pgo-data/merged.profdata"

            # Optional: AddressSanitizer on rustc itself
            # sanitize = ["address"]

            [target.${pkgs.stdenv.hostPlatform.rust.rustcTargetSpec}]
            llvm-config = "${pkgs.llvmPackages.llvm}/bin/llvm-config"
            EOF

            python3 ./x.py setup
            cat config.toml  # debug
          '';

          buildPhase = ''
            python3 ./x.py build --stage 2 -j$NIX_BUILD_CORES library/std rustc cargo clippy
          '';

          installPhase = ''
            mkdir -p $out
            cp -r build/*/stage2/* $out/
            cp -r build/*/stage2-tools-bin/* $out/bin/ 2>/dev/null || true
          '';

          # Make the build pure and reproducible
          dontFixup = true;
          hardeningDisable = [ "fortify" ];

          meta = {
            description = "Fully instrumented Rust compiler with LLVM coverage";
            platforms = pkgs.lib.platforms.all;
          };
        };

      in
      {
        packages = {
          default = instrumentedRustc;
          inherit rustToolchain;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            instrumentedRustc
            pkgs.llvmPackages.llvm
            pkgs.llvmPackages.bintools
            pkgs.lcov
            pkgs.grcov   # optional: Rust coverage tool
          ];

          shellHook = ''
            export RUSTC="${instrumentedRustc}/bin/rustc"
            export RUSTFLAGS="-C instrument-coverage -C link-dead-code"
            echo "Instrumented rustc ready: $RUSTC"
            $RUSTC --version --verbose
          '';
        };
      });
}
```

### 3. rust-toolchain.toml (optional, for pinning)

```toml
[toolchain]
channel = "nightly-2025-11-29"   # change daily
components = [ "rustc", "rust-src", "cargo", "llvm-tools" ]
targets = [ "x86_64-unknown-linux-gnu" ]
profile = "minimal"
```

### 4. How to build

```bash
git clone https://github.com/yourname/rustc-instrumented.git
cd rustc-instrumented
nix build .#default          # or nix build
./result/bin/rustc --version # → rustc 1.85.0-nightly (xxxxxxx 2025-11-29)
```

### 5. Collect coverage from the compiler itself

```bash
# Run some compilation that uses your instrumented rustc
./result/bin/rustc -C instrument-coverage your_project.rs

# Generate .profraw files
LLVM_PROFILE_FILE="rustc-%p.profraw" ./result/bin/rustc ...

# Merge and view coverage
llvm-profdata merge -sparse rustc-*.profraw -o rustc.profdata
llvm-cov show \
  ./result/bin/rustc --instr-profile=rustc.profdata --show-line-counts-or-regions
```

### Bonus: One-liner for quick testing (nix run)

Add this to your `flake.nix` outputs:

```nix
apps.default = {
  type = "app";
  program = "${self.packages.${system}.default}/bin/rustc";
};
```

Then run:

```bash
nix run github:yourname/rustc-instrumented -- --version -C instrument-coverage
```

You now have a fully instrumented, reproducible Rust compiler built with Nix! Perfect for fuzzing, coverage-guided testing, or deep compiler debugging.
