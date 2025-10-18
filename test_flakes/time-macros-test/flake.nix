{
  description = "Test flake for time-macros";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:meta-introspector/cargo2nix?ref=feature/CRQ-016-nixify";
    time-rs.url = "github:meta-introspector/time-rs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, cargo2nix, time-rs }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ cargo2nix.overlays.default ];
      };
      rustPkgs = pkgs.rustBuilder.makePackageSet {
        packageFun = import ./Cargo.nix;
        rustChannel = "stable";
        rustVersion = "1.81.0";
      };
    in
    {
      packages.aarch64-linux.time-macros = rustPkgs.workspace.time-macros;
      defaultPackage.aarch64-linux = self.packages.aarch64-linux.time-macros;
    };
}
