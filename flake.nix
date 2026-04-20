{
  description = "Rust dev shell for macroquad/miniquad on NixOS";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };

      libs = with pkgs; [
        alsa-lib
        libGL
        vulkan-loader
        libxkbcommon
        wayland

        libX11
        libXcursor
        libXi
        libXrandr
        libXinerama
        libXext
        libXrender
        libXfixes
      ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          clippy
          rustfmt
          pkg-config
          gcc

          alsa-lib
          libGL
          vulkan-loader
          libxkbcommon
          wayland

          libX11
          libXcursor
          libXi
          libXrandr
          libXinerama
          libXext
          libXrender
          libXfixes
        ];

        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath libs}:$LD_LIBRARY_PATH"
          echo "LD_LIBRARY_PATH configured for macroquad/miniquad"
        '';
      };
    };
}
