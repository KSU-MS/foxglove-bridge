{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    can_pkg_flake.url = "github:KSU-MS/ksu-ms-dbc/main";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      flake-utils,
      naersk,
      can_pkg_flake,
      nixpkgs,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ can_pkg_flake.overlays.default ];
        };

        naersk' = pkgs.callPackage naersk { };

        NIX_LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

      in
      {
        overlays.default = nixpkgs.lib.composeManyExtensions [

        ];

        # For `nix build` & `nix run`:
        packages.default = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            can_pkg
          ];

          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            libclang
          ];

          # Setting up the environment variables you need during development.
          shellHook = ''
            dbc_path=${pkgs.can_pkg}
            export DBC_PATH=$dbc_path
            libclang_path=${NIX_LIBCLANG_PATH}
            export LIBCLANG_PATH=$libclang_path
          '';
        };
      }
    );
}
