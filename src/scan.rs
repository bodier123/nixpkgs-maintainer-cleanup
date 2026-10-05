//! Phase 1 Nix maintainer-reference scanner.
//!
//! This module is intentionally read-only. It demonstrates that `rnix` can
//! distinguish explicit maintainer-list syntax from expressions that should be
//! left for manual review. Source rewriting is a later project phase.

use rnix::ast;
use rowan::ast::AstNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindingKind {
    /// `lib.maintainers.<handle>` inside a literal list.
    Direct,
    /// `<handle>` inside `with lib.maintainers; [ ... ]`.
    Scoped,
    /// The target text appears in a maintainers expression that Phase 1 does
    /// not attempt to interpret.
    ManualReview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub line: usize,
    pub expression: String,
    pub kind: FindingKind,
}

fn attr_name(attr: ast::Attr) -> Option<String> {
    match attr {
        ast::Attr::Ident(name) => Some(name.syntax().text().to_string()),
        ast::Attr::Str(string) => {
            let text = string.syntax().text().to_string();
            let body = text.strip_prefix('"')?.strip_suffix('"')?;
            if body.contains("${") || body.contains('\\') {
                None
            } else {
                Some(body.to_string())
            }
        }
        ast::Attr::Dynamic(_) => None,
    }
}

fn attr_path(path: ast::Attrpath) -> Option<Vec<String>> {
    path.attrs().map(attr_name).collect()
}

fn unparen(mut expr: ast::Expr) -> ast::Expr {
    loop {
        match &expr {
            ast::Expr::Paren(paren) => match paren.expr() {
                Some(inner) => expr = inner,
                None => return expr,
            },
            _ => return expr,
        }
    }
}

fn static_selection(expr: ast::Expr) -> Option<Vec<String>> {
    match unparen(expr) {
        ast::Expr::Ident(name) => Some(vec![name.syntax().text().to_string()]),
        ast::Expr::Select(select) if select.default_expr().is_none() => {
            let mut prefix = static_selection(select.expr()?)?;
            prefix.extend(attr_path(select.attrpath()?)?);
            Some(prefix)
        }
        _ => None,
    }
}

fn line_number(source: &str, offset: usize) -> usize {
    source.as_bytes()[..offset]
        .iter()
        .filter(|byte| **byte == b'\n')
        .count()
        + 1
}

fn start_offset(node: &rnix::SyntaxNode) -> usize {
    u32::from(node.text_range().start()) as usize
}

fn scan_list(
    source: &str,
    list: ast::List,
    handle: &str,
    scoped: bool,
    findings: &mut Vec<Finding>,
) {
    for item in list.items() {
        let text = item.syntax().text().to_string();
        let line = line_number(source, start_offset(item.syntax()));

        if scoped {
            if let ast::Expr::Ident(ident) = unparen(item.clone()) {
                if ident.syntax().text().to_string() == handle {
                    findings.push(Finding {
                        line,
                        expression: text,
                        kind: FindingKind::Scoped,
                    });
                    continue;
                }
            }
        }

        if let Some(path) = static_selection(item.clone()) {
            if path.len() == 3
                && path[0] == "lib"
                && path[1] == "maintainers"
                && path[2] == handle
            {
                findings.push(Finding {
                    line,
                    expression: text,
                    kind: FindingKind::Direct,
                });
                continue;
            }
        }

        if text.contains(handle) {
            findings.push(Finding {
                line,
                expression: text,
                kind: FindingKind::ManualReview,
            });
        }
    }
}

/// Scan one Nix source file for references to `handle` in attributes whose
/// final name is `maintainers`.
///
/// Phase 1 recognizes two explicit forms and reports other matching forms for
/// manual review. It never changes the input source.
pub fn scan_source(source: &str, handle: &str) -> Result<Vec<Finding>, String> {
    let parsed = rnix::Root::parse(source);
    if !parsed.errors().is_empty() {
        return Err(format!("Nix parse error: {:?}", parsed.errors()));
    }

    let tree = parsed.tree();
    let mut findings = Vec::new();

    for binding in tree
        .syntax()
        .descendants()
        .filter_map(ast::AttrpathValue::cast)
    {
        let Some(path) = binding.attrpath().and_then(attr_path) else {
            continue;
        };
        if path.last().map(String::as_str) != Some("maintainers") {
            continue;
        }

        let Some(value) = binding.value() else {
            continue;
        };
        let binding_line = line_number(source, start_offset(binding.syntax()));
        let value_text = value.syntax().text().to_string();

        match unparen(value) {
            ast::Expr::List(list) => {
                scan_list(source, list, handle, false, &mut findings);
            }
            ast::Expr::With(with_expr) => {
                let namespace_is_maintainers = with_expr
                    .namespace()
                    .and_then(static_selection)
                    .is_some_and(|path| {
                        path.len() == 2 && path[0] == "lib" && path[1] == "maintainers"
                    });

                if namespace_is_maintainers {
                    match with_expr.body().map(unparen) {
                        Some(ast::Expr::List(list)) => {
                            scan_list(source, list, handle, true, &mut findings);
                        }
                        _ if value_text.contains(handle) => findings.push(Finding {
                            line: binding_line,
                            expression: value_text,
                            kind: FindingKind::ManualReview,
                        }),
                        _ => {}
                    }
                } else if value_text.contains(handle) {
                    findings.push(Finding {
                        line: binding_line,
                        expression: value_text,
                        kind: FindingKind::ManualReview,
                    });
                }
            }
            _ if value_text.contains(handle) => findings.push(Finding {
                line: binding_line,
                expression: value_text,
                kind: FindingKind::ManualReview,
            }),
            _ => {}
        }
    }

    Ok(findings)
}
