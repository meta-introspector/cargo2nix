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
