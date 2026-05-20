{
  description = "Change file path to shortest unique form, like tide does";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "tidelike";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          doCheck = false; # speed thing up by avoiding second build

          meta = with pkgs.lib; {
            description = "Change file path to shortest unique form, like tide does";
            mainProgram = "tidelike";
            license = licenses.bsd3;
            platforms = platforms.linux;
          };
        };
      }
    );
}
