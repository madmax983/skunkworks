use anyhow::Result;
use std::process::Command;

/// Runs `git diff HEAD` (or against a target) and extracts deleted characters.
/// Returns a flat list of characters that were deleted.
pub fn get_deleted_chars() -> Result<Vec<char>> {
    let output = Command::new("git").args(["diff", "HEAD"]).output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_diff_output(&stdout))
}

pub fn parse_diff_output(output: &str) -> Vec<char> {
    let mut deleted_chars = Vec::new();

    for line in output.lines() {
        if let Some(stripped) = line.strip_prefix('-') {
            // Ignore "--- a/file" header lines
            if stripped.starts_with("--") {
                continue;
            }
            // Add chars from this deleted line
            for ch in stripped.chars() {
                if !ch.is_whitespace() {
                    deleted_chars.push(ch);
                }
            }
        }
    }
    deleted_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let diff = "
diff --git a/test b/test
index ...
--- a/test
+++ b/test
@@ -1,2 +1,2 @@
-hello world
+hello universe
-foo bar
";
        let chars = parse_diff_output(diff);
        // Expect: h,e,l,l,o,w,o,r,l,d,f,o,o,b,a,r
        let expected: Vec<char> = "helloworldfoobar".chars().collect();
        assert_eq!(chars, expected);
    }
}
