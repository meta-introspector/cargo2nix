{
  description = "A simple SSH-enabled Nix flake";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux"; # Explicitly define system for simplicity
      pkgs = import nixpkgs {
        inherit system;
        config = {
          permittedInsecurePackages = [ "openssl-1.1.1w" ];
        };
      };
    in
    {
      devShells.default = pkgs.mkShell {
        packages = [
          pkgs.openssh
        ];
        shellHook = ''
          echo "Welcome to the SSH shell!"
          echo "You can now connect to ubuntu@129.213.85.94 using 'ssh ubuntu@129.213.85.94'"
          echo "Make sure your SSH key is properly configured in your ~/.ssh directory or specified with -i."
        '';
      };
    };
}
}
