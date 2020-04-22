let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  myrust = (pkgs.rustChannelOf { date = "2020-04-12"; channel = "nightly"; }).rust.override { extensions = [ "rust-src" ]; };
in
  with pkgs;
  stdenv.mkDerivation {
    name = "game-of-life-rust";
    nativeBuildInputs = [
      myrust
      pijul
      gtk3 pkgconfig
    ];
    RUST_SRC_PATH= "${myrust}/lib/rustlib/src/rust/src";
  }
