use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub raw: String,
    pub timestamp: Option<String>,
    pub level: Option<String>,
    pub component: Option<String>,
    pub message: String,
}

pub struct LogParser {
    // We'll use a few regexes to try and match common formats
    // Priority:
    // 1. Standard bracketed: "[TIMESTAMP] [LEVEL] [COMPONENT] Message"
    // 2. Syslog style: "TIMESTAMP HOST COMPONENT[PID]: Message"
    // 3. Simple Level: "LEVEL: Message"
    regex_bracket: Regex,
    regex_syslog: Regex,
    regex_simple: Regex,
}

impl LogParser {
    pub fn new() -> Self {
        Self {
            // [2023-01-01 12:00:00] [INFO] [Auth] User logged in
            regex_bracket: Regex::new(r"^\[(.*?)\]\s+\[(\w+)\]\s+(?:\[(.*?)\]\s+)?(.*)$").unwrap(),
            // Oct 11 22:14:15 myhost sshd[123]: Failed password
            // Simplified: Timestamp Host Component: Message
            regex_syslog: Regex::new(r"^(\w{3}\s+\d+\s+\d{2}:\d{2}:\d{2})\s+\S+\s+([^:]+):\s+(.*)$").unwrap(),
            // INFO: Something happened
            regex_simple: Regex::new(r"^(\w+):\s+(.*)$").unwrap(),
        }
    }

    pub fn parse(&self, line: &str) -> LogEntry {
        let line = line.trim();

        if let Some(caps) = self.regex_bracket.captures(line) {
            return LogEntry {
                raw: line.to_string(),
                timestamp: Some(caps[1].to_string()),
                level: Some(caps[2].to_string()),
                component: caps.get(3).map(|m| m.as_str().to_string()),
                message: caps[4].to_string(),
            };
        }

        if let Some(caps) = self.regex_syslog.captures(line) {
            return LogEntry {
                raw: line.to_string(),
                timestamp: Some(caps[1].to_string()),
                level: None, // Syslog often doesn't have explicit level in text, encoded in priority
                component: Some(caps[2].to_string()),
                message: caps[3].to_string(),
            };
        }

        if let Some(caps) = self.regex_simple.captures(line) {
             return LogEntry {
                raw: line.to_string(),
                timestamp: None,
                level: Some(caps[1].to_string()),
                component: None,
                message: caps[2].to_string(),
            };
        }

        // Fallback
        LogEntry {
            raw: line.to_string(),
            timestamp: None,
            level: None,
            component: None,
            message: line.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracket_format() {
        let parser = LogParser::new();
        let line = "[2023-10-27 10:00:00] [ERROR] [Network] Connection timed out";
        let entry = parser.parse(line);

        assert_eq!(entry.timestamp.unwrap(), "2023-10-27 10:00:00");
        assert_eq!(entry.level.unwrap(), "ERROR");
        assert_eq!(entry.component.unwrap(), "Network");
        assert_eq!(entry.message, "Connection timed out");
    }

    #[test]
    fn test_syslog_format() {
        let parser = LogParser::new();
        let line = "Oct 27 10:00:00 myhost sshd[123]: Failed password for invalid user";
        let entry = parser.parse(line);

        assert_eq!(entry.timestamp.unwrap(), "Oct 27 10:00:00");
        assert_eq!(entry.component.unwrap(), "sshd[123]");
        assert_eq!(entry.message, "Failed password for invalid user");
    }

    #[test]
    fn test_simple_format() {
        let parser = LogParser::new();
        let line = "WARN: Disk usage high";
        let entry = parser.parse(line);

        assert_eq!(entry.level.unwrap(), "WARN");
        assert_eq!(entry.message, "Disk usage high");
    }

    #[test]
    fn test_fallback() {
        let parser = LogParser::new();
        let line = "Just a random string";
        let entry = parser.parse(line);

        assert_eq!(entry.message, "Just a random string");
        assert!(entry.timestamp.is_none());
    }
}
