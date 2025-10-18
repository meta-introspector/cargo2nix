{
  description = "Rust toolchain test flake";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
        config = {
          permittedInsecurePackages = [ "openssl-1.1.1w" ];
        };
      };
      rustToolchain = pkgs.rust-bin.stable."1.81.0".default;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          rustToolchain
          pkgs.openssl_1_1
        ];
        shellHook = ''
          export OPENSSL_DIR=${pkgs.openssl_1_1}
          export PKG_CONFIG_PATH=${pkgs.openssl_1_1}/lib/pkgconfig:$PKG_CONFIG_PATH
        '';
      };
    };
}
