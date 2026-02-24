use crate::lexer::Token;
use crate::phonology::SoundChange;
use logos::Logos;

pub struct Evolver;

impl Evolver {
    pub fn evolve(code: &str, change: &dyn SoundChange) -> String {
        let mut lex = Token::lexer(code);
        let mut result = String::new();

        // logos iteration
        while let Some(token_result) = lex.next() {
            let text = lex.slice();

            match token_result {
                Ok(Token::Ident) => {
                    // Apply sound change to identifier
                    let evolved = change.apply(text);
                    result.push_str(&evolved);
                }
                _ => {
                    // Keep everything else as is
                    result.push_str(text);
                }
            }
        }

        result
    }
}
