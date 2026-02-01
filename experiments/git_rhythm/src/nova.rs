use rand::seq::SliceRandom;

pub struct NarrativeGenerator {
    templates: Vec<&'static str>,
}

impl Default for NarrativeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NarrativeGenerator {
    pub fn new() -> Self {
        Self {
            templates: vec![
                "The code flowed like a river...",
                "Bugs whispered in the dark...",
                "A commit was made, but at what cost?",
                "The merge conflict was inevitable.",
                "Refactoring brings clarity to the chaos.",
            ],
        }
    }

    pub fn generate(&self, _input: &str) -> String {
        let mut rng = rand::thread_rng();
        self.templates
            .choose(&mut rng)
            .unwrap_or(&"Silence.")
            .to_string()
    }
}
