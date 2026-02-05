use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meaning {
    pub actor: String,
    pub action: String,
    pub target: String,
}

impl Meaning {
    pub fn new(actor: &str, action: &str, target: &str) -> Self {
        Self {
            actor: actor.to_string(),
            action: action.to_string(),
            target: target.to_string(),
        }
    }
}

impl fmt::Display for Meaning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}, {})", self.action, self.actor, self.target)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    Nominative, // Subject
    Accusative, // Object
    Dative,     // Indirect Object (unused for now, simplified to SVO)
    None,       // Eroded/Analytic
}

impl Case {
    pub fn suffix(&self) -> &str {
        match self {
            Case::Nominative => "us",
            Case::Accusative => "um",
            Case::Dative => "o",
            Case::None => "",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    pub root: String,
    pub case: Case,
    pub raw: String, // The actual string produced (root + suffix)
}

impl Word {
    pub fn new(root: &str, case: Case) -> Self {
        let raw = format!("{}{}", root, case.suffix());
        Self {
            root: root.to_string(),
            case,
            raw,
        }
    }

    pub fn from_raw(raw: &str) -> Self {
        // Simple heuristic parser for the "Proto-Language"
        if raw.ends_with("us") {
            Self {
                root: raw.trim_end_matches("us").to_string(),
                case: Case::Nominative,
                raw: raw.to_string(),
            }
        } else if raw.ends_with("um") {
            Self {
                root: raw.trim_end_matches("um").to_string(),
                case: Case::Accusative,
                raw: raw.to_string(),
            }
        } else {
            Self {
                root: raw.to_string(),
                case: Case::None,
                raw: raw.to_string(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordOrder {
    SVO,
    SOV,
    VSO,
    Free,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub word_order: WordOrder,
    pub morphology_strength: f32, // 0.0 to 1.0. Probability of using Case suffixes.
}

impl Default for Grammar {
    fn default() -> Self {
        Self {
            word_order: WordOrder::Free, // Starts as Latin-like
            morphology_strength: 1.0,    // Starts with full cases
        }
    }
}
