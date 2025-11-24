{
  description = "A minimal development shell for cargo2nix (Phase 1: uses pre-built cargo)";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay = {
      url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    #cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.12";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , flake-utils
    , #cargo2nix
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        #cargo2nix.overlays.default
        rust-overlay.overlays.default
      ];
      pkgs = import nixpkgs {
        inherit system overlays;
        config = {
          permittedInsecurePackages = [ "openssl-1.1.1w" ];
        };
      };

      myRustc = pkgs.rust-bin.nightly."2025-09-16".default;

      rustPkgs = pkgs.rustBuilder.makePackageSet {
        rustToolchain = myRustc;
      };

      # Use pre-built cargo from nixpkgs for Phase 1
      cargo = pkgs.cargo;

      workspaceShell = pkgs.mkShell {
        packages = [ pkgs.statix pkgs.openssl_1_1.dev pkgs.zlib.dev pkgs.sccache pkgs.llvmPackages_18.libclang pkgs.llvmPackages_18.llvm pkgs.llvmPackages_18.clang pkgs.gcc pkgs.gdb ];
        shellHook = ''
          export PKG_CONFIG_PATH=${pkgs.openssl_1_1.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
          export PATH=${myRustc}/bin:${cargo}/bin:${pkgs.sccache}/bin:${pkgs.llvmPackages_18.llvm}/bin:${pkgs.llvmPackages_18.clang}/bin:${pkgs.gcc}/bin:${pkgs.gdb}/bin:$PATH
          export LIBCLANG_PATH="${pkgs.llvmPackages_18.libclang.lib}/lib"
          export LD_LIBRARY_PATH="${pkgs.llvmPackages_18.libclang.lib}/lib:${pkgs.llvmPackages_18.llvm.lib}/lib:$LD_LIBRARY_PATH"
          export LLVM_SYS_180_PREFIX=${pkgs.llvmPackages_18.llvm.dev}
          export CC=${pkgs.gcc}/bin/gcc
          export CXX=${pkgs.gcc}/bin/g++
        '';
      };

      llvmDevShell = pkgs.mkShell {
        packages = [ pkgs.statix pkgs.openssl_1_1.dev pkgs.zlib.dev pkgs.sccache pkgs.llvm_18.dev pkgs.clang_18 ];
        shellHook = ''
          export PKG_CONFIG_PATH=${pkgs.openssl_1_1.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
          export PATH=${myRustc}/bin:${cargo}/bin:${pkgs.sccache}/bin:${pkgs.llvm_18}/bin:${pkgs.clang_18}/bin:$PATH
          export LLVM_SYS_180_PREFIX=${pkgs.llvm_18.dev}
        '';
      };

      gccDevShell = pkgs.mkShell {
        packages = [ pkgs.statix pkgs.openssl_1_1.dev pkgs.zlib.dev pkgs.sccache pkgs.gcc pkgs.gdb ];
        shellHook = ''
          export PKG_CONFIG_PATH=${pkgs.openssl_1_1.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
          export PATH=${myRustc}/bin:${cargo}/bin:${pkgs.sccache}/bin:${pkgs.gcc}/bin:${pkgs.gdb}/bin:$PATH
          export CC=${pkgs.gcc}/bin/gcc
          export CXX=${pkgs.gcc}/bin/g++
        '';
      };
    in
    rec {
      devShells = {
        default = workspaceShell;
        llvm = llvmDevShell;
        gcc = gccDevShell;
      };

      packages = rec {
        inherit cargo;
        default = cargo;
      };

      apps = rec {
        cargo = { type = "app"; program = "${packages.cargo}/bin/cargo"; };
        default = cargo;
      };
    }
    );
}
