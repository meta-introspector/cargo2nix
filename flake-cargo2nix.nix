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
        overlays = [ rust-overlay.overlays.default ]; # Only rust-overlay for now
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            permittedInsecurePackages = [ "openssl-1.1.1w" ];
          };
        };

        myRustc = pkgs.rust-bin.nightly."2025-09-16".default;

      in
      {
        packages.cargo2nix-bin = pkgs.rustPlatform.buildRustPackage {
          pname = "cargo2nix";
          version = "0.12.0"; # Assuming this is the version of cargo2nix
          src = self; # Build from the current flake's source

          cargoLock = pkgs.lib.mkForce ./Cargo.lock; # Use the project's Cargo.lock
          rustToolchain = myRustc;

          # Add any other build inputs or dependencies required by cargo2nix
          # buildInputs = [ pkgs.openssl ]; # Example
        };
      }
    );
}
