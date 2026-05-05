{
  description = "A Minimal Fetching Utility.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        buildPackage = import ./nix/buildPackage.nix { inherit pkgs; };
        devShell = import ./nix/devShell.nix { inherit pkgs; };
      in
      {
        packages.default = buildPackage;
        packages.sysfetch = buildPackage;
        
        devShells.default = devShell;
        
        apps.default = {
          type = "app";
          program = "${buildPackage}/bin/sysfetch";
        };
      });
}