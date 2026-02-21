{
  description = "My Home: Home Management App";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        # Rust toolchain: stable with extras
        rustToolchain = pkgs.fenix.stable.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ];

        # Runtime libraries needed by the Slint binary
        runtimeLibs = with pkgs; [
          mesa
          fontconfig
          libxkbcommon
          wayland
          libx11
          libxcursor
          libxi
          libxrandr
          libGL
          libGLU
          libglut
          wayland
        ];

      in {
        devShells.default = pkgs.mkShell {
          name = "myHome";

          nativeBuildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.cargo-watch
            pkgs.just
          ];

          buildInputs = [
            pkgs.fontconfig
            pkgs.openssl
          ] ++ runtimeLibs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;

          # shellHook = ''
          #   export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
          # '';
        };
      });
}
