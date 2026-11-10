{
  inputs = {
    sh4-rust.url = "github:mergmann/sh4-rust";
    nixpkgs.follows = "sh4-rust/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      sh4-rust,
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            sh4-rust.packages.${system}.default
          ];
          # RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
