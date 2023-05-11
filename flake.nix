{
  description = "Using a parametric model of the Galaxy to infer its parameters";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs-old.url = "github:NixOS/nixpkgs?rev=79b3d4bcae8c7007c9fd51c279a8a67acfa73a2a";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    nixpkgs-old,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      pkgs-old = nixpkgs-old.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs;
          [
            bashInteractive
            cargo
            clippy
            julia
            lsof
            qpdf
            rust-analyzer
            rustc
            zip
          ]
          ++ (with pkgs-old; [
            biber
            tectonic
            texlab
            (texlive.combine {
              inherit (texlive) luatex85 pgf pgfplots scheme-basic standalone xkeyval;
            })
          ]);
      };
    });
}
