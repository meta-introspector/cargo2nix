{
  description = "Nix flake for cargo-llm-bootstrap";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
          ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cargo-llm-bootstrap";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      });
}
