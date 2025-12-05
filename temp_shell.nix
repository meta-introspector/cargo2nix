{ pkgs ? import <nixpkgs> { overlays = [ (import (builtins.getFlake "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify")).overlays.default ]; }
,
}:

let
  rustToolchain = pkgs.rust-bin.nightly."2025-10-06".default;
in
pkgs.mkShell {
  buildInputs = [
    rustToolchain
  ];
}
