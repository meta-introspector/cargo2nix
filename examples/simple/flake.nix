{
  description = "A simple example using cargo2nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    cargo2nix-flake.url = "path:../../.."; # Reference to the main cargo2nix flake
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (cargo2nix-flake.packages.${system}) cargo2nix;
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustVersion = "1.83.0"; # Match the version in the main flake
        };
        simpleExample = rustPkgs.workspace.simple-example {};
      in {
        packages.default = simpleExample;

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rustc
            pkgs.cargo
            cargo2nix
          ];
          shellHook = ''
            export PATH=$PWD/target/debug:$PATH
            echo "Welcome to the simple-example dev shell!"
            echo "Run 'cargo build' or 'cargo run'"
            echo "To regenerate Cargo.nix, run: cargo2nix > Cargo.nix"
          '';
        };
      });
}
