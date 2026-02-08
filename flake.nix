{
  description = "Gitlab Work CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        commonArgs = {
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        gw-cli = pkgs.rustPlatform.buildRustPackage ({
          pname = "gw"
          version = "0.1.0";
          cargoBuildFlags = [ "-p" "gw" ]; # имя пакета из cli/Cargo.toml
        } // commonArgs);

        gw-backend = pkgs.rustPlatform.buildRustPackage ({
          pname = "gw-backend";
          version = "0.1.0";
          cargoBuildFlags = [ "-p" "gw-backend" ]; # имя пакета backend
        } // commonArgs);
      in {
        packages = {
          default = gw-cli;
          cli = gw-cli;
          backend = gw-backend;
        };
      }
    );
}
