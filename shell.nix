{ pkgs ? import <nixpkgs> {} }:

let
  overrides = builtins.fromTOML (builtins.readFile ./rust-toolchain.toml);
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    pkg-config
    clang

    wayland
    libxkbcommon
    libglvnd
    vulkan-loader

    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libxcb
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.libglvnd
    pkgs.vulkan-loader
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.xorg.libxcb
  ];

  RUSTC_VERSION = overrides.toolchain.channel;

  shellHook = ''
    export PATH="''${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
    export PATH="''${RUSTUP_HOME:-$HOME/.rustup}/toolchains/$RUSTC_VERSION-${pkgs.stdenv.hostPlatform.rust.rustcTarget}/bin:$PATH"
  '';
}
