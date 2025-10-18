{
  description = "Test flake for time-macros";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    time-rs.url = "path:../../vendor/time-rs";
  };

  outputs = { self, nixpkgs, time-rs }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
    in
    {
      packages.aarch64-linux.time-macros = time-rs.packages.aarch64-linux.time-macros;
      defaultPackage.aarch64-linux = self.packages.aarch64-linux.time-macros;
    };
}
