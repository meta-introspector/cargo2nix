{
  description = "A minimal development shell for cargo2nix (Phase 2: uses self-built cargo)";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay = {
      url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.12"; # This will be overridden
    cargo-src = {
      url = "path:./submodules/cargo"; # Path to our cargo source
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
      inputs.cargo2nix.follows = "cargo2nix"; # Ensure cargo-src uses the same cargo2nix input
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, cargo2nix, cargo-src }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ cargo2nix.overlays.default rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            permittedInsecurePackages = [ "openssl-1.1.1w" ];
          };
        };

        myRustc = pkgs.rust-bin.nightly."2025-09-16".default;

        # Build cargo from our source
        selfBuiltCargo = cargo-src.packages.${system}.cargo;

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustToolchain = myRustc;
        };

        # Use the self-built cargo for cargo2nix's development
        cargo = selfBuiltCargo;

        workspaceShell = pkgs.mkShell {
          packages = [ pkgs.statix pkgs.openssl_1_1.dev ];
          shellHook = ''
            export PKG_CONFIG_PATH=${pkgs.openssl_1_1.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
            export PATH=${myRustc}/bin:${cargo}/bin:$PATH
          '';
        };
      in
      rec {
        devShells = {
          default = workspaceShell;
        };

        packages = rec {
          inherit cargo;
          workspaceCrates = rustPkgs.workspace;
          default = cargo;
        };

        apps = rec {
          cargo = { type = "app"; program = "${packages.cargo}/bin/cargo"; };
          default = cargo;
        };
      }
    );
}
