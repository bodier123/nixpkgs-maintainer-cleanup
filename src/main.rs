use std::{env, fs, process::ExitCode};

use nixpkgs_maintainer_cleanup::{scan_source, FindingKind};

fn usage() {
    eprintln!("usage: nixpkgs-maintainer-cleanup HANDLE FILE.nix");
    eprintln!("Phase 1 is read-only: it parses one Nix file and reports matches.");
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(handle) = args.next() else {
        usage();
        return ExitCode::from(2);
    };
    let Some(path) = args.next() else {
        usage();
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        usage();
        return ExitCode::from(2);
    }

    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("could not read {path}: {error}");
            return ExitCode::from(2);
        }
    };

    let findings = match scan_source(&source, &handle) {
        Ok(findings) => findings,
        Err(error) => {
            eprintln!("{path}: {error}");
            return ExitCode::from(2);
        }
    };

    if findings.is_empty() {
        println!("no maintainer references for '{handle}' found in {path}");
        return ExitCode::SUCCESS;
    }

    for finding in findings {
        let label = match finding.kind {
            FindingKind::Direct => "direct reference",
            FindingKind::Scoped => "scoped reference",
            FindingKind::ManualReview => "manual review",
        };
        println!(
            "line {}: {}: {}",
            finding.line,
            label,
            finding.expression.trim()
        );
    }

    ExitCode::SUCCESS
}
