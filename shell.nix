{
  pkgs ? import <nixpkgs> { },
}:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    cargo
    rustc
    rust-analyzer
    clippy
    rustfmt
  ];
  buildInputs = [
    udev
    alsa-lib-with-plugins
    vulkan-loader
    libx11
    libxcursor
    libxi
    libxrandr
    libxkbcommon
    wayland
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  RUST_SRC_PATH = rustPlatform.rustLibSrc;

  shellHook = ''
    codium .
  '';
}

