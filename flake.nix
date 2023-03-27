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
          julia
          lsof
          rustup
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

        cargoSha256 = "sha256-bMelaLwqi1Pen4EA3L1xmt7A+85r17TkCyWtDNiNHgE=";
      };
    });
}
