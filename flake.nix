{
  description = "A minimal development shell for cargo2nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay = {
      url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.12";
    allocator-api2 = {
      url = "file:./submodules/allocator-api2";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.cargo2nix.follows = "cargo2nix";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, cargo2nix }:
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

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustToolchain = myRustc;
          packageOverrides = pkgs: [
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "config";
              version = "0.15.18"; # Specify the version of the config crate
              overrideAttrs = old: {
                # Explicitly set the edition, as it's failing to inherit from workspace
                CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_EDITION = "2024";
                CARGO_PROFILE_DEV_BUILD_OVERRIDE_EDITION = "2024";
              };
            })
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "allocator-api2";
              version = "0.3.1"; # Specify the version of the allocator-api2 crate
              src = self.inputs.allocator-api2; # Correctly reference the input
            })
          ];
        };

        #       cargo = rustPkgs.workspace.cargo2nix { };

        workspaceShell = pkgs.mkShell {
          packages = [ pkgs.statix pkgs.openssl_1_1.dev ];
          shellHook = ''
            export PKG_CONFIG_PATH=${pkgs.openssl_1_1.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
          '';
        };
      in
      #            export PATH=${myRustc}/bin:${cargo}/bin:$PATH
      rec {
        devShells = {
          default = workspaceShell;
        };

        packages = rec {
          cargo2nixDrv = rustPkgs.workspace.cargo2nix {}; # Get the derivation
          default = cargo2nixDrv;
        };

        apps = rec {
          cargo2nixApp = { type = "app"; program = "${packages.default}/bin/cargo2nix"; };
          default = cargo2nixApp;
        };
      }
    );
}
