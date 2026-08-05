//! # Codex Vulgaris 📜
//!
//! > "The language of the machine is not immune to the ravages of time."
//!
//! **Codex Vulgaris** is a philological experiment that treats source code identifiers as living words in a natural language. It applies historical sound change laws (Grimm's Law, Great Vowel Shift, Lenition) to simulate how your code might sound if it evolved over centuries of oral tradition.
//!
//! ## Concept
//!
//! Code is usually static, but what if it drifted?
//! - `calculate_sum` -> *Grimm's Law* -> `halhulate_thum`
//! - `let mut variable` -> *Vowel Shift* -> `let mut veriable`
//!
//! This tool visualizes this decay (or evolution) in real-time.
//!
//! ## Features
//!
//! - **Phonological Engine:** Implements `Grimm's Law`, `Great Vowel Shift`, `Lenition` (intervocalic voicing), and `Assimilation`.
//! - **Lexical Analysis:** Uses `logos` to tokenize Rust code, separating immutable keywords from evolving identifiers.
//! - **Etymological Dictionary:** Tracks the original form of every mutated word.
//! - **Time Travel:** Move forward and backward through "Eras" of linguistic change.
//!
//! ## Controls
//!
//! - `Left` / `Right`: Travel through time (Eras).
//! - `?`: Toggle Help.
//! - `q`: Quit.
//!
//! ## The Linguistic Laws
//!
//! 1.  **Grimm's Law:** Proto-Germanic shift. Unvoiced stops become fricatives (`p`->`f`, `t`->`th`, `k`->`h`).
//! 2.  **Great Vowel Shift:** English vowel raising (`a`->`e`->`i`->`ai`).
//! 3.  **Lenition:** Softening of consonants between vowels (`p`->`b`, `t`->`d`).
//! 4.  **Assimilation:** Consonants adapting to neighbors (`n` -> `m` before `p`).
//!
//! ## Running
//!
//! ```bash
//! cargo run -p codex-vulgaris
//! ```
//!
mod app;
mod lexer;
mod phonology;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{io, time::Duration};
use ui::ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B::Error: std::fmt::Debug,
{
    loop {
        terminal
            .draw(|f| ui(f, app))
            .map_err(|e| io::Error::other(format!("{:?}", e)))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                        return Ok(());
                    }
                    KeyCode::Char('?') => {
                        app.toggle_help();
                    }
                    KeyCode::Esc => {
                        if app.show_help {
                            app.show_help = false;
                        }
                    }
                    KeyCode::Right => {
                        app.next_era();
                    }
                    KeyCode::Left => {
                        app.prev_era();
                    }
                    _ => {}
                }
            }
        }
    }
}
