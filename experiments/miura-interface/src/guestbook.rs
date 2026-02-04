use regex::Regex;

#[derive(Debug, Clone)]
pub struct GuestbookEntry {
    pub concentration: String,
    pub location: String,
    pub scent_origin: String,
    pub status: String,
}

pub fn parse_guestbook(content: &str) -> Vec<GuestbookEntry> {
    let mut entries = Vec::new();

    // Regex patterns
    // Matches: ### [Concentration Level: HIGH] - Location: experiments/fungal-balancer
    let header_re = Regex::new(r"### \[Concentration Level: (.*?)\] - Location: (.*)").unwrap();
    // Matches: - **Scent Origin:** Genesis (The Mycologist) 🍄
    let scent_re = Regex::new(r"- \*\*Scent Origin:\*\* (.*)").unwrap();
    // Matches: - **Status:** Spores released...
    let status_re = Regex::new(r"- \*\*Status:\*\* (.*)").unwrap();

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if let Some(caps) = header_re.captures(line) {
            let concentration = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let location = caps.get(2).map_or("", |m| m.as_str()).to_string();

            let mut scent_origin = String::new();
            let mut status = String::new();

            // Look ahead for Scent and Status
            // Assuming they follow immediately or close by
            // Only look a few lines ahead to avoid skipping too far
            for j in 1..4 {
                if i + j >= lines.len() {
                    break;
                }
                let next_line = lines[i + j];

                if let Some(scent_caps) = scent_re.captures(next_line) {
                    scent_origin = scent_caps.get(1).map_or("", |m| m.as_str()).to_string();
                }
                if let Some(status_caps) = status_re.captures(next_line) {
                    status = status_caps.get(1).map_or("", |m| m.as_str()).to_string();
                }
            }

            entries.push(GuestbookEntry {
                concentration,
                location,
                scent_origin,
                status,
            });
        }
        i += 1;
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let content = r#"
### [Concentration Level: HIGH] - Location: experiments/fungal-balancer
- **Scent Origin:** Genesis (The Mycologist) 🍄
- **Status:** Spores released.

### [Concentration Level: DECOMPOSING] - Location: experiments/clockwork-cpu
- **Scent Origin:** Reaper
- **Status:** EXECUTED.
        "#;

        let entries = parse_guestbook(content);
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].concentration, "HIGH");
        assert_eq!(entries[0].location, "experiments/fungal-balancer");
        assert_eq!(entries[0].scent_origin, "Genesis (The Mycologist) 🍄");
        assert_eq!(entries[0].status, "Spores released.");

        assert_eq!(entries[1].concentration, "DECOMPOSING");
        assert_eq!(entries[1].location, "experiments/clockwork-cpu");
    }
}
