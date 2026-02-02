#[derive(Clone, Debug)]
pub struct Card {
    pub name: &'static str,
    pub description: &'static str,
    pub art: &'static str,
}

pub const CARDS: &[Card] = &[
    Card {
        name: "The Architect",
        description: "Structure, Planning, Design Patterns",
        art: r#"
  .-------.
  |  / \  |
  | /_o_\ |
  |   |   |
  |  / \  |
  '-------'
"#,
    },
    Card {
        name: "The Bug",
        description: "Chaos, Entropy, Unexpected Behavior",
        art: r#"
  .-------.
  | \   / |
  | (o_o) |
  | // \\ |
  |   "   |
  '-------'
"#,
    },
    Card {
        name: "The Merge",
        description: "Unity, Completion, Integration",
        art: r#"
  .-------.
  |  \ /  |
  |   Y   |
  |   |   |
  |   |   |
  '-------'
"#,
    },
    Card {
        name: "The Refactor",
        description: "Transformation, Death and Rebirth",
        art: r#"
  .-------.
  |  ___  |
  | ( > ) |
  | // \\ |
  | \___/ |
  '-------'
"#,
    },
    Card {
        name: "The Deadline",
        description: "Time, Pressure, Finality",
        art: r#"
  .-------.
  | .---. |
  | | O | |
  | | | | |
  | '---' |
  '-------'
"#,
    },
    Card {
        name: "The Legacy",
        description: "History, Technical Debt, Wisdom",
        art: r#"
  .-------.
  |  ___  |
  | /___\ |
  | |old| |
  | \___/ |
  '-------'
"#,
    },
    Card {
        name: "The Green Build",
        description: "Success, Hope, Harmony",
        art: r#"
  .-------.
  |   _   |
  |  (_)  |
  |   |   |
  |  -+-  |
  '-------'
"#,
    },
    Card {
        name: "The Segfault",
        description: "Mystery, Sudden End, Null Pointer",
        art: r#"
  .-------.
  |  ___  |
  | /   \ |
  | | X | |
  | \___/ |
  '-------'
"#,
    },
];
