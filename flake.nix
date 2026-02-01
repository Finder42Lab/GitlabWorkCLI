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

        aw-cli = pkgs.rustPlatform.buildRustPackage {
          pname = "aw";
          version = "0.1.0";

          src = ./.;
          buildType = "release";

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          cargoBuildFlags = [ "--bin" "aw" ];

          installPhase = ''
              mkdir -p $out/lib
              cp target/release/aw $out/lib/
            '';

          meta = with pkgs.lib; {
            description = "Gitlab Work CLI";
            license = licenses.mit;
            platforms = platforms.linux;
          };
        };
      in {
        packages.default = aw-cli;
      }
    );
}
