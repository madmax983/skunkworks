use std::process::Command;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum LineType {
    Addition,
    Deletion,
    Context,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub content: String,
    pub line_type: LineType,
}

pub fn get_diff() -> Result<Vec<DiffLine>> {
    // Try diff against HEAD to get all changes (staged and unstaged)
    let output = Command::new("git")
        .args(&["diff", "HEAD"])
        .output();

    let diff_str = match output {
        Ok(out) if !out.stdout.is_empty() => String::from_utf8(out.stdout)?,
        _ => {
            // Fallback: git show HEAD (last commit)
             let output_log = Command::new("git")
                .args(&["show", "HEAD", "--format="]) // content only
                .output()?;
             String::from_utf8(output_log.stdout)?
        }
    };

    if diff_str.trim().is_empty() {
        // Ultimate fallback: Mock data if no git or empty repo
        return Ok(vec![
            DiffLine { content: "No changes detected!".to_string(), line_type: LineType::Context },
            DiffLine { content: "fn main() {".to_string(), line_type: LineType::Addition },
            DiffLine { content: "    println!(\"Hello World\");".to_string(), line_type: LineType::Addition },
            DiffLine { content: "}".to_string(), line_type: LineType::Addition },
        ]);
    }

    parse_diff(&diff_str)
}

fn parse_diff(raw: &str) -> Result<Vec<DiffLine>> {
    let mut lines = Vec::new();
    for line in raw.lines() {
        // Skip metadata headers
        if line.starts_with("diff ")
            || line.starts_with("index ")
            || line.starts_with("--- ")
            || line.starts_with("+++ ")
            || line.starts_with("@@")
        {
            continue;
        }

        if let Some(rest) = line.strip_prefix('+') {
             lines.push(DiffLine { content: rest.to_string(), line_type: LineType::Addition });
        } else if let Some(rest) = line.strip_prefix('-') {
             lines.push(DiffLine { content: rest.to_string(), line_type: LineType::Deletion });
        } else {
             // Treat as context - maybe obstacles?
             // For now, let's include them but sparse
             // lines.push(DiffLine { content: line.to_string(), line_type: LineType::Context });
        }
    }

    // If we filtered everything out (e.g. just file moves), return something
    if lines.is_empty() {
        lines.push(DiffLine { content: "No content changes found.".to_string(), line_type: LineType::Context });
    }

    Ok(lines)
}
