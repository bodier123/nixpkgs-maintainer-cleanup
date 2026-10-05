use nixpkgs_maintainer_cleanup::{scan_source, FindingKind};

#[test]
fn finds_direct_reference_in_literal_list() {
    let source = r#"
{ lib, stdenv }:
stdenv.mkDerivation {
  meta.maintainers = [
    lib.maintainers.alice
    lib.maintainers.bob
  ];
}
"#;

    let findings = scan_source(source, "alice").expect("source should parse");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].kind, FindingKind::Direct);
}

#[test]
fn finds_scoped_reference() {
    let source = r#"
{ lib, stdenv }:
stdenv.mkDerivation {
  meta = {
    maintainers = with lib.maintainers; [ alice bob ];
  };
}
"#;

    let findings = scan_source(source, "alice").expect("source should parse");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].kind, FindingKind::Scoped);
}

#[test]
fn computed_expression_is_manual_review() {
    let source = r#"
{ lib, stdenv }:
stdenv.mkDerivation {
  meta.maintainers = makeMaintainers alice;
}
"#;

    let findings = scan_source(source, "alice").expect("source should parse");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].kind, FindingKind::ManualReview);
}

#[test]
fn unrelated_text_is_ignored() {
    let source = r#"
{ lib, stdenv }:
stdenv.mkDerivation {
  description = "alice appears here but is not a maintainer field";
  meta.maintainers = [ lib.maintainers.bob ];
}
"#;

    let findings = scan_source(source, "alice").expect("source should parse");
    assert!(findings.is_empty());
}
