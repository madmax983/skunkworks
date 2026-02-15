use serde::{Deserialize, Serialize};
use std::process::Command;
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpan {
    pub file_name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    pub text: Vec<DiagnosticSpanLine>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpanLine {
    pub text: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: String,
    pub spans: Vec<DiagnosticSpan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCode {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CargoMessage {
    reason: String,
    message: Option<Diagnostic>,
}

pub fn run_cargo_check(manifest_path: &str) -> Result<Vec<Diagnostic>> {
    let output = Command::new("cargo")
        .args(&["check", "--message-format=json", "--manifest-path", manifest_path])
        .output()
        .context("Failed to run cargo check")?;

    let stdout = String::from_utf8(output.stdout)?;
    let mut diagnostics = Vec::new();

    for line in stdout.lines() {
        // Only parse lines that look like JSON objects
        if !line.starts_with('{') { continue; }

        if let Ok(msg) = serde_json::from_str::<CargoMessage>(line) {
            if msg.reason == "compiler-message" {
                if let Some(diagnostic) = msg.message {
                    diagnostics.push(diagnostic);
                }
            }
        }
    }

    Ok(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_diagnostic() {
        let json = r#"{"reason":"compiler-message","package_id":"origami-diagnostics 0.1.0 (path+file:///app/experiments/origami-diagnostics)","manifest_path":"/app/experiments/origami-diagnostics/Cargo.toml","target":{"kind":["bin"],"crate_types":["bin"],"name":"origami-diagnostics","src_path":"/app/experiments/origami-diagnostics/src/main.rs","edition":"2021","doc":true,"doctest":false,"test":true},"message":{"children":[],"code":null,"level":"warning","message":"unused import: `std::process::Command`","spans":[{"byte_end":39,"byte_start":18,"column_end":25,"column_start":5,"file_name":"src/main.rs","is_primary":true,"label":null,"line_end":2,"line_start":2,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_end":25,"highlight_start":5,"text":"use std::process::Command;"}]}]}}"#;

        let msg: CargoMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.reason, "compiler-message");
        let diag = msg.message.unwrap();
        assert_eq!(diag.message, "unused import: `std::process::Command`");
        assert_eq!(diag.spans.len(), 1);
        assert_eq!(diag.spans[0].line_start, 2);
    }
}
