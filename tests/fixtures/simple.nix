{ lib, stdenv }:
stdenv.mkDerivation {
  pname = "demo";
  version = "1.0";

  meta.maintainers = [
    lib.maintainers.alice
    lib.maintainers.bob
  ];
}
