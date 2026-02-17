use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum SegmentType {
    User,
    System,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraceSegment {
    pub content: String,
    pub segment_type: SegmentType,
}

pub fn parse_trace(input: &str) -> Vec<TraceSegment> {
    let mut segments = Vec::new();
    let re = Regex::new(r"^\s*\d+:\s*(.*)$").unwrap();
    let sys_prefixes = [
        "std::", "core::", "alloc::", "tokio::", "panic::", "actix::",
    ];

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let function_name = caps.get(1).map_or("", |m| m.as_str()).trim().to_string();
            let segment_type = if sys_prefixes.iter().any(|&p| function_name.starts_with(p)) {
                SegmentType::System
            } else {
                SegmentType::User
            };
            segments.push(TraceSegment {
                content: function_name,
                segment_type,
            });
        }
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_trace() {
        let input = "
stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/std/src/lib.rs:100
   1: core::fmt::num::imp::fmt_u64
             at /rustc/core/src/fmt/num.rs:200
   2: my_app::main
             at src/main.rs:10
";
        let segments = parse_trace(input);

        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].segment_type, SegmentType::System);
        assert_eq!(segments[1].segment_type, SegmentType::System);
        assert_eq!(segments[2].segment_type, SegmentType::User);
    }
}
