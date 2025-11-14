{
  description = "A flake for building cargo2nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            permittedInsecurePackages = [ "openssl-1.1.1w" ];
          };
        };

        myRustc = pkgs.rust-bin.nightly."2025-09-16".default;

        cargo2nixPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "cargo2nix";
          version = "0.12.0";
          src = ./.; # Explicitly point to the current directory
          manifestDir = "."; # Explicitly set the manifest directory

          cargoLock = pkgs.lib.mkForce ./Cargo.lock;
          rustToolchain = myRustc;
          package = "cargo2nix"; # Explicitly specify the package to build
        };

      in
      rec {
        packages.cargo2nix-bin = cargo2nixPackage;
        defaultPackage = self.packages.${system}.cargo2nix-bin;
      }
    );
}
