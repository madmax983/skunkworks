use ::rand::rngs::StdRng;
use ::rand::SeedableRng;
use glossolalia::lexicon::{Lexicon, TokenType};
use glossolalia::phonology::{GrimmsLaw, Rule, VowelShift};
use macroquad::prelude::*;

const SAMPLE_CODE: &str = r#"
fn main() {
    let mut civilization = Civilization::new();
    let mut year = 0;

    loop {
        civilization.evolve();
        year += 1;

        if year > 1000 {
            println!("The fall of Rome");
            break;
        }
    }
}

struct Civilization {
    population: u64,
    language: String,
}

impl Civilization {
    fn new() -> Self {
        Self {
            population: 100,
            language: "Latin".to_string(),
        }
    }

    fn evolve(&mut self) {
        // Entropy increases
        self.population -= 1;
    }
}
"#;

#[macroquad::main("Glossolalia")]
async fn main() {
    let mut lexicon = Lexicon::new(SAMPLE_CODE);
    let mut rng = StdRng::seed_from_u64(42);

    let rules: Vec<Box<dyn Rule>> = vec![Box::new(GrimmsLaw), Box::new(VowelShift)];

    let mut century = 0;
    let mut last_evolution = get_time();
    let evolution_interval = 2.0; // Seconds per century
    let mut auto_evolve = true;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0)); // Dark grey

        // Update
        if is_key_pressed(KeyCode::Space) {
            auto_evolve = !auto_evolve;
        }

        if is_key_pressed(KeyCode::Right)
            || (auto_evolve && get_time() - last_evolution > evolution_interval)
        {
            lexicon.evolve(&rules, &mut rng);
            century += 100;
            last_evolution = get_time();
        }

        if is_key_pressed(KeyCode::R) {
            lexicon = Lexicon::new(SAMPLE_CODE);
            century = 0;
            rng = StdRng::seed_from_u64(42);
        }

        // Draw
        let start_y = 40.0;
        let line_height = 20.0;
        let start_x = 20.0;

        draw_text(&format!("Year: {}", century), start_x, 20.0, 30.0, GOLD);
        draw_text(
            "Space: Pause/Play | Right: Step | R: Reset",
            300.0,
            20.0,
            20.0,
            LIGHTGRAY,
        );

        let mut x = start_x;
        let mut y = start_y;

        // Simple line wrapping logic (very basic)
        // Or just render line by line if we assume code is pre-formatted
        // My tokenizer preserves whitespace (newlines), so I can just follow the tokens.

        for token in &lexicon.tokens {
            let text = match token.token_type {
                TokenType::Identifier => {
                    if let Some(word) = lexicon.evolved_identifiers.get(&token.content) {
                        word.to_string_word()
                    } else {
                        token.content.clone()
                    }
                }
                _ => token.content.clone(),
            };

            // Check for newlines in whitespace tokens
            if token.token_type == TokenType::Whitespace {
                // If it contains newlines, reset x and increment y
                let newlines = text.matches('\n').count();
                if newlines > 0 {
                    y += newlines as f32 * line_height;
                    x = start_x;
                    // Handle indentation: the text after the last newline is the indentation
                    if let Some(_last_line) = text.lines().last() {
                        // But wait, split keeps the newlines?
                        // text.lines() removes newlines.
                        // If text is "\n    ", lines gives ["", "    "].
                        // I just need to calculate the length of the last part.
                        // Or just draw the spaces?
                        // Simplest: just draw every char? No, variable width font.
                        // macroquad uses a monospaced font by default? No, it uses a default font (sans-serif).
                        // I should load a mono font or just hope for the best.
                        // But drawing whitespace " " advances x.
                        // Newline "\n" resets x.
                    }
                    // For simplicity, let's process char by char for whitespace tokens containing newlines
                    for c in text.chars() {
                        if c == '\n' {
                            y += line_height;
                            x = start_x;
                        } else {
                            let dims = measure_text(&c.to_string(), None, 20, 1.0);
                            x += dims.width;
                        }
                    }
                    continue;
                }
            }

            let color = match token.token_type {
                TokenType::Keyword => SKYBLUE,
                TokenType::Identifier => WHITE, // They evolve!
                TokenType::Symbol => GRAY,
                TokenType::Literal => GREEN,
                TokenType::Comment => DARKGRAY,
                TokenType::Whitespace => WHITE,
            };

            // Draw the token text
            draw_text(&text, x, y, 20.0, color);

            // Advance cursor
            let dims = measure_text(&text, None, 20, 1.0);
            x += dims.width;
        }

        next_frame().await
    }
}
