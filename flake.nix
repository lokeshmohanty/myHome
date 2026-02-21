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
          config = {
            allowUnfree = true;
            android_sdk.accept_license = true;
          };
          overlays = [ fenix.overlays.default ];
        };

        # Rust toolchain: stable with extras and Android targets
        rustToolchain = pkgs.fenix.combine [
          (pkgs.fenix.stable.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
            "rust-analyzer"
          ])
          pkgs.fenix.targets.aarch64-linux-android.stable.rust-std
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

        # Android environment builder
        androidComposition = pkgs.androidenv.composeAndroidPackages {
          cmdLineToolsVersion = "8.0";
          toolsVersion = "26.1.1";
          platformToolsVersion = "35.0.1";
          buildToolsVersions = [ "35.0.0" ];
          platformVersions = [ "35" "30" ];
          includeEmulator = false;
          includeSystemImages = false;
          includeNDK = true;
          ndkVersions = [ "27.2.12479018" ];
          cmakeVersions = [ "3.22.1" ];
          abiVersions = [ "arm64-v8a" ];
          useGoogleAPIs = false;
          useGoogleTVAddOns = false;
        };

        androidSdk = androidComposition.androidsdk;

      in
      {
        devShells.default = pkgs.mkShell {
          name = "myHome";

          nativeBuildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.cargo-watch
            pkgs.just
            pkgs.cargo-apk
            pkgs.jdk17
            androidSdk
          ];

          buildInputs = [
            pkgs.fontconfig
            pkgs.openssl
          ] ++ runtimeLibs;

          # Shell environment variables
          ANDROID_HOME = "${androidSdk}/libexec/android-sdk";
          ANDROID_NDK_ROOT = "${androidSdk}/libexec/android-sdk/ndk/27.2.12479018";
          JAVA_HOME = "${pkgs.jdk17.home}";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;

          shellHook = ''
            export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"
          '';
        };

      });
}
