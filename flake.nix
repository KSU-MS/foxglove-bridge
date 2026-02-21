{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    can_pkg_flake.url = "github:KSU-MS/ksu-ms-dbc/main";
  };

  outputs =
    {
      nixpkgs,
      can_pkg_flake,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);

      bfbs_overlay = final: prev: {
        bfbs_pkg = final.callPackage ./dbc_to_bfbs/default.nix { };
      };

      rust_server_overlay = final: prev: {
        server_pkg = final.callPackage ./rust_server/default.nix { };
      };
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              can_pkg_flake.overlays.default
              bfbs_overlay
              rust_server_overlay
            ];
          };

          libclangPath = pkgs.lib.makeLibraryPath [
            pkgs.llvmPackages_latest.libclang.lib
          ];
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              rust-analyzer
              flatbuffers
              bfbs_pkg
              can_pkg
              libclang
              server_pkg
            ];

            shellHook = ''
              export DBC_PATH=${pkgs.can_pkg}/car.dbc
              export BFBS_PATH=${pkgs.bfbs_pkg}/dbc.bfbs
              export LIBCLANG_PATH=${libclangPath}
            '';
          };
        }
      );
    };
}
