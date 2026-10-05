{ lib, stdenv }:
stdenv.mkDerivation {
  pname = "phase1-demo";
  version = "0.1";

  meta.maintainers = [
    lib.maintainers.alice
    lib.maintainers.bob
  ];

  passthru.exampleMeta = {
    maintainers = with lib.maintainers; [ alice bob ];
  };

  passthru.computedMeta = {
    maintainers = makeMaintainers alice;
  };
}
