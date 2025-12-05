{
  description = "Command line utility for removing dependencies and build artifacts from unused projects.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
      flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in
        {
          packages = rec {
            polykill = pkgs.rustPlatform.buildRustPackage {
                pname = cargoToml.package.name;
                version = cargoToml.package.version;

                src = ./.;

                cargoHash = "sha256-bYBamcyHcIGTJBAkD7jYiSyTEXhtS8kNfM0D+BuDpw8=";

                meta = {
                  description = cargoToml.package.description;
                  homepage = cargoToml.package.repository;
                };
            };
            default = polykill;
          };
        }
      );
}
