{
  description = "Server app for SlimeVR ecosystem";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.pre-commit.url = "github:cachix/pre-commit-hooks.nix";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    pre-commit,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = with pkgs; [
          cmake
          git
          gcc
          xorg.libXext
          xorg.libXft
          xorg.libXinerama
          xorg.libXcursor
          xorg.libXrender
          xorg.libXfixes
          wayland
          wayland-protocols
          udev
          libxkbcommon
          dbus.dev
          libcerf
          pango
          cairo
          libGL
          mesa
          pkg-config
          openssl
        ];
        buildInputs = with pkgs; [
        ];
      in {
        checks = {
          pre-commit-check = pre-commit.lib.${system}.run {
            src = ./.;
            hooks = {
              alejandra.enable = true;
              # rustfmt.enable = true;
            };
          };
        };
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            nativeBuildInputs
            ++ [
            ];
          buildInputs =
            buildInputs
            ++ [
              rustTarget
            ];

          inherit (self.checks.${system}.pre-commit-check) shellHook;
        };
      }
    );
}
