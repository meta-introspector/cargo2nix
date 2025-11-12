{
  description = "A minimal development shell for cargo2nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay = {
      url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true; # Allow unfree packages if needed
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            openssl.dev # Include OpenSSL development libraries
            pkg-config # Often needed for C dependencies
          ];
          # Set environment variables if necessary, e.g., for OpenSSL
          # OPENSSL_DIR = "${pkgs.openssl}";
          # OPENSSL_STATIC = "1"; # If static linking is desired
        };
      }
    );
}
