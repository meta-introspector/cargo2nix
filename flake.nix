{
  description = "Development shell for cargo2nix with necessary libraries for building";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay = {
      url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            permittedInsecurePackages = [ "openssl-1.1.1w" ];
          };
        };

        myRustc = pkgs.rust-bin.nightly."2025-10-05".default;

      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            # Rust toolchain
            myRustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.clippy

            # System libraries needed for libgit2-sys and curl-sys
            pkgs.libgit2 # For libgit2-sys
            pkgs.curl    # For curl-sys
            pkgs.openssl_1_1.dev # Explicitly use OpenSSL 1.1.1w development files
            pkgs.libssh2 # For git SSH support
            pkgs.zlib    # For compression
            pkgs.nghttp2 # For HTTP/2 support
            pkgs.pkg-config # Needed for build scripts to find libraries

            # Compilers (clang is used in error messages, so include it)
            pkgs.llvmPackages_19.llvm # LLVM for rustc_llvm
            pkgs.clang # Add clang to default dev shell

            # Other potentially useful tools
            pkgs.statix
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.openssl_1_1.dev}/lib/pkgconfig''${PKG_CONFIG_PATH:+:}$PKG_CONFIG_PATH";
            export LLVM_CONFIG="${pkgs.llvmPackages_19.llvm}/bin/llvm-config";
            export LIBCLANG_PATH="${pkgs.llvmPackages_19.libclang}";
            export LLVM_CONFIG_PATH="${pkgs.llvmPackages_19.llvm}/lib";
            export REAL_LIBRARY_PATH_VAR="LD_LIBRARY_PATH";
            export REAL_LIBRARY_PATH="$LD_LIBRARY_PATH";
            # Ensure cargo is available in PATH for cargo build inside nix develop
            export PATH="${myRustc}/bin:${pkgs.cargo}/bin''${PATH:+:}$PATH";
            export RUSTC_BOOTSTRAP=1;
            export CFLAGS="-O2 -g";
            export CXXFLAGS="-O2 -g";
            export CFG_RELEASE="1.70.0"; # Added to resolve rustc_hir error
            echo "Nix development shell with Rust, libgit2, curl, and OpenSSL 1.1.1w ready.";
          '';
        };

        devShells.build = pkgs.mkShell {
          packages = [
            # Rust toolchain
            myRustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.clippy

            # System libraries needed for libgit2-sys and curl-sys
            pkgs.libgit2 # For libgit2-sys
            pkgs.curl    # For curl-sys
            pkgs.openssl_1_1.dev # Explicitly use OpenSSL 1.1.1w development files
            pkgs.libssh2 # For git SSH support
            pkgs.zlib    # For compression
            pkgs.nghttp2 # For HTTP/2 support
            pkgs.pkg-config # Needed for build scripts to find libraries
            pkgs.llvmPackages_19.llvm # LLVM for rustc_llvm
            # Compilers (clang is used in error messages, so include it)
            pkgs.clang
            pkgs.gcc

            # Other potentially useful tools
            pkgs.statix
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.openssl_1_1.dev}/lib/pkgconfig''${PKG_CONFIG_PATH:+:}$PKG_CONFIG_PATH";
            export LLVM_CONFIG="${pkgs.llvmPackages_19.llvm}/bin/llvm-config";
            export LIBCLANG_PATH="${pkgs.llvmPackages_19.libclang}";
            export LLVM_CONFIG_PATH="${pkgs.llvmPackages_19.llvm}/lib";
            export REAL_LIBRARY_PATH_VAR="LD_LIBRARY_PATH";
            export REAL_LIBRARY_PATH="$LD_LIBRARY_PATH";
            # Ensure cargo is available in PATH for cargo build inside nix develop
            export PATH="${myRustc}/bin:${pkgs.cargo}/bin''${PATH:+:}$PATH";
            export RUSTC_BOOTSTRAP=1;
            export CFLAGS="-O2 -g";
            export CXXFLAGS="-O2 -g";
            echo "Nix development shell with Rust, libgit2, curl, and OpenSSL 1.1.1w ready. Running make build...";
            make build;
          '';
        };
      }
    );
}