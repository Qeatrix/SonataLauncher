{
  description = "xx";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable"; };

  outputs = { self, nixpkgs, ... }:
    let
      pkgsFor = system:
        import nixpkgs {
          inherit system;
          overlays = [ ];
        };

      targetSystems = [ "x86_64-linux" ];
    in {
      devShells = nixpkgs.lib.genAttrs targetSystems (system:
        let 
          pkgs = pkgsFor system;
          packagesD = with pkgs; [
              electron_30-bin
              chromium
              ungoogled-chromium
              gtk3
              rustup

              # Compilers
              cargo
              rustc
              cargo-tauri
              nodejs_22

              # Libs
              openssl_3
              dbus
              libsoup
              gdk-pixbuf
              librsvg
              emmet-language-server
              webkitgtk

              # Tools
              curl
              wget
              pkg-config
              clippy
              rust-analyzer
              rustfmt
              cairo
            ];
        in {
          default = pkgs.mkShell {
            name = "xx";
            nativeBuildInputs = packagesD;
          };
          shellHook =
            ''
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath packagesD}:$LD_LIBRARY_PATH
              export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
            '';
        });
    };
}
