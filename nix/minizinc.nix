{ pkgs, lib }:

let
  minizinc-models = pkgs.stdenv.mkDerivation {
    name = "minizinc-monster-models";
    src = ../models;
    installPhase = ''
      mkdir -p $out/share/minizinc/models
      cp -r *.mzn *.dzn $out/share/minizinc/models/
    '';
  };

in {
  minizinc-env = pkgs.buildEnv {
    name = "minizinc-monster-env";
    paths = [
      pkgs.minizinc
      pkgs.gecode
      minizinc-models
    ];
  };

  runMiniZinc = { model, data ? null }:
    pkgs.runCommand "minizinc-solution" {
      buildInputs = [ pkgs.minizinc pkgs.gecode ];
    } ''
      ${if data != null then
        "minizinc --output-mode json ${model} ${data} > $out"
      else
        "minizinc --output-mode json ${model} > $out"
      }
    '';
}
