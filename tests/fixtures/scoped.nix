{ lib, stdenv }:
stdenv.mkDerivation {
  pname = "demo";
  version = "1.0";

  meta = {
    maintainers = with lib.maintainers; [ alice bob ];
  };
}
