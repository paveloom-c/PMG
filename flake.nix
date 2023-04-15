{
  description = "Using a parametric model of the Galaxy to infer its parameters";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          bashInteractive
          cargo
          clippy
          julia
          lsof
          rust-analyzer
          rustc
          (texlive.combine {
            inherit (texlive) luatex85 pgf pgfplots scheme-basic standalone;
          })
          zip
        ];
      };
    });
}
