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
          cargo
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
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "pmg";
        version = "0.1.0";

        src = ./.;

        cargoSha256 = "sha256-p8WCTOPf/UuMn6/YPBkCmzyX493qEgH1GKvMXi0TstU=";
      };
    });
}
