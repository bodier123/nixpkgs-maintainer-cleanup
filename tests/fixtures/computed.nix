{ lib, stdenv }:
stdenv.mkDerivation {
  pname = "demo";
  version = "1.0";

  meta.maintainers = makeMaintainers alice;
}
