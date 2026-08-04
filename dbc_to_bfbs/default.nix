{
  pkgs,
  can_pkg,
  flatbuffers,
}:

pkgs.stdenv.mkDerivation {
  name = "can_bfbs";

  src = ./.;

  buildInputs = [
    can_pkg
    flatbuffers
    pkgs.python313Packages.cantools
  ];

  buildPhase = ''
    python3 ./main.py ${can_pkg}/*.dbc
    flatc -b --schema can_dbc.fbs
  '';

  installPhase = ''
    mkdir -p $out
    mv ./can_dbc.fbs $out/dbc.fbs
    mv ./can_dbc.bfbs $out/dbc.bfbs
  '';
}
