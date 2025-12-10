{
  description = "Development shell for cargo2nix with necessary libraries for building";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        pkgs = import nixpkgs {
          inherit system;
          config = {
            permittedInsecurePackages = [ "openssl-1.1.1w" ];
          };
        };

        myRustc = pkgs.rustc;

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

            pkgs.llvmPackages_21.llvm # LLVM for rustc_llvm
            pkgs.llvmPackages_21.libclang # Add libclang to default dev shell
            pkgs.gcc # Add gcc to default dev shell
            pkgs.glibc.dev # Provides system headers
            pkgs.libc
            pkgs.libc
            pkgs.snappy # For "snappy" feature
            pkgs.lz4 # For "lz4" feature
            pkgs.zstd # For "zstd" feature
            pkgs.zlib # For "zlib" feature
            pkgs.bzip2 # For "bzip2" feature
            pkgs.liburing # For "io-uring" feature (pkg-config will find it)
            pkgs.pkg-config # Needed for pkg-config in build.rs
            pkgs.openssl_1_1.dev # Included in permittedInsecurePackages, but needs to be in packages for its headers to be found by bindgen.

          ];
                    shellHook = ''

            export CFG_RELEASE="1.70.0"; # Added to resolve rustc_hir error
            export CFLAGS="-O2 -g";
            export CPATH="${pkgs.glibc.dev}/include:${pkgs.gcc}/include${CPATH:+:}$CPATH"; # Added glibc.dev/include to CPATH
            export CXXFLAGS="-O2 -g -isystem ${pkgs.glibc.dev}/include";
            export LIBCLANG_FLAGS="--sysroot=${pkgs.glibc.dev}"; # For bindgen to find stdbool.h
            export LIBCLANG_PATH="${pkgs.llvmPackages_21.libclang.lib}/lib"; # Explicit path
            export LLVM_CONFIG="${pkgs.llvmPackages_21.llvm.dev}/bin/llvm-config";
            export LLVM_CONFIG_PATH="${pkgs.llvmPackages_21.llvm}/lib";
            export PATH="${myRustc}/bin:${pkgs.cargo}/bin''${PATH:+:}$PATH";
            # export PATH="${myRustc}/bin:${pkgs.cargo}/bin:$PATH";
            export PKG_CONFIG_PATH="${pkgs.openssl_1_1.dev}/lib/pkgconfig''${PKG_CONFIG_PATH:+:}$PKG_CONFIG_PATH";
            export REAL_LIBRARY_PATH="$LD_LIBRARY_PATH";
            export REAL_LIBRARY_PATH_VAR="LD_LIBRARY_PATH";
            export RUSTC_BOOTSTRAP=1;
            export NIX_GLIBC_DEV="${pkgs.glibc.dev}";
            export NIX_GCC_PATH="${pkgs.gcc}";
            export NIX_GCC_REAL_PATH="${pkgs.gcc.cc}";

            # # Ensure cargo is available in PATH for cargo build inside nix develop
            
            # # Flags for bindgen to find system headers.
            # # This incorporates the logic from oldflake.nix shellHook.
            # export BINDGEN_EXTRA_CLANG_ARGS=$(
            #   cat ${pkgs.stdenv.cc}/nix-support/libc-crt1-cflags \
            #        ${pkgs.stdenv.cc}/nix-support/libc-cflags \
            #        ${pkgs.stdenv.cc}/nix-support/cc-cflags) \
            # ${pkgs.lib.optionalString pkgs.stdenv.cc.isClang "-idirafter ${pkgs.stdenv.cc.cc.lib}/lib/clang/${pkgs.lib.getVersion pkgs.stdenv.cc.cc}/include"}

            export BINDGEN_EXTRA_CLANG_ARGS="$(
              cat ${pkgs.stdenv.cc}/nix-support/libc-crt1-cflags \
                  ${pkgs.stdenv.cc}/nix-support/libc-cflags \
                  ${pkgs.stdenv.cc}/nix-support/cc-cflags
            ) ${pkgs.lib.optionalString pkgs.stdenv.cc.isClang "-idirafter ${pkgs.stdenv.cc.cc.lib}/lib/clang/${pkgs.lib.getVersion pkgs.stdenv.cc.cc}/include"}";            

            echo "Nix development shell with Rust, libgit2, curl, openssl, and rocksdb dependencies ready.";
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
            pkgs.llvmPackages_21.llvm # LLVM for rustc_llvm
            # Compilers (clang is used in error messages, so include it)
            pkgs.llvmPackages_21.libclang
            pkgs.gcc
            # Other potentially useful tools
            pkgs.statix
            pkgs.snappy # For "snappy" feature
            pkgs.lz4 # For "lz4" feature
            pkgs.zstd # For "zstd" feature
            pkgs.bzip2 # For "bzip2" feature
            pkgs.liburing # For "io-uring" feature
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.openssl_1_1.dev}/lib/pkgconfig:$PKG_CONFIG_PATH";
            export LLVM_CONFIG="${pkgs.llvmPackages_21.llvm.dev}/bin/llvm-config";
            export LIBCLANG_PATH="${pkgs.llvmPackages_21.libclang.lib}/lib";
            export LLVM_CONFIG_PATH="${pkgs.llvmPackages_21.llvm}/lib"; 
            export REAL_LIBRARY_PATH_VAR="LD_LIBRARY_PATH";
            export REAL_LIBRARY_PATH="$LD_LIBRARY_PATH";
            export CPATH="${pkgs.glibc.dev}/include:${pkgs.gcc}/include${CPATH:+:}$CPATH"; # Added glibc.dev/include to CPATH
            # Ensure cargo is available in PATH for cargo build inside nix develop
            export PATH="${myRustc}/bin:${pkgs.cargo}/bin''${PATH:+:}$PATH";
            export RUSTC_BOOTSTRAP=1;
            export NIX_GLIBC_DEV="${pkgs.glibc.dev}";
            export NIX_GCC_PATH="${pkgs.gcc}";
            export NIX_GCC_REAL_PATH="${pkgs.gcc.cc}";
            export CFLAGS="-O2 -g";
            export CXXFLAGS="-O2 -g -isystem ${pkgs.glibc.dev}/include";
            export LIBCLANG_FLAGS="--sysroot=${pkgs.glibc.dev}"; # For bindgen to find stdbool.h
            echo "Nix development shell with Rust, libgit2, curl, openssl, and rocksdb dependencies ready. Running make build...";
            '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cargo-repo-sync";
          version = "0.1.0"; # Use a placeholder version, or extract from Cargo.toml if available

          src = ./.; # The entire workspace as source

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # This is where we specify which member of the workspace to build as the default package
          # Assuming 'cargo-repo-sync' is a binary crate located in 'tools/cargo-repo-sync'
          # This might need adjustment based on the actual structure and desired output
          cargoBuildFlags = [ "--package" "cargo-repo-sync" ];

          # Add any other necessary build inputs
          buildInputs = [
            # Add common build inputs from devShell if needed, e.g., openssl_1_1.dev, libgit2, curl
            pkgs.openssl_1_1.dev
            pkgs.libgit2
            pkgs.curl
            pkgs.libssh2
            pkgs.zlib
            pkgs.nghttp2
            pkgs.pkg-config
            pkgs.llvmPackages_21.llvm
            pkgs.llvmPackages_21.libclang
            pkgs.gcc
            pkgs.glibc.dev
          ];

          # Ensure the correct rustc is used during build
          # This might conflict with myRustc definition earlier, need to ensure consistency
          # For now, let's just assume `pkgs.rustc` is sufficient
          rustc = pkgs.rustc;
        };

      }
    );
}
