{
  pkgs,
  can_pkg,
  bfbs_pkg,
}:

pkgs.rustPlatform.buildRustPackage {
  pname = "foxglove-bridge";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "h264_webcam_stream-0.1.0" = "sha256-DzUeC5kgLcQKhhiNJTlnRIWTiVglPQQzyZ6abI+Uz3s=";
    };
  };

  # For bindgen
  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [
    pkgs.llvmPackages_latest.libclang.lib
  ];

  buildInputs = [
    pkgs.flatbuffers
    can_pkg
    bfbs_pkg
  ];
}
