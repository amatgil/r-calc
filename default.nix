{
  pkgs ? import <nixpkgs> { },
}:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  packages = with pkgs; [
    # Empty!
  ];
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  cargoLock.outputHashes = {
    "arduino-hal-0.1.0" = "sha256-gr1xjGDhHeQwxvQpJzHhdnsRvEFo9mgID4ngtcocfwM=";
  };
  src = pkgs.lib.cleanSource ./.;
  meta.description = manifest.description ? null;
}
