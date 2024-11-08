{
  pkgs ? import <nixpkgs> { },
  lib,
}:
let
  packages = with pkgs; [
    (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = [ "rust-src" ];
    }))
    rust-analyzer
    rustfmt
    clippy
    mold
    rustup # mostly for rustup doc
    ravedude

    #wayland
    #xorg.libX11
    #xorg.libXcursor
  ];
  avr = with pkgs.pkgsCross.avr.buildPackages; [
    binutils
    gcc
    avrdude
    ravedude
    simavr
  ];
in
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  nativeBuildInputs = packages;
  buildInputs = avr ++ packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}
