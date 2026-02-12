use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use spqr_rsa::crypto::{KeyPair, decrypt, encrypt, generate_keys};
use spqr_rsa::roman::Roman;
use std::io;
use std::time::Duration;

struct App {
    keys: Option<KeyPair>,
    message: Option<Roman>,
    cipher: Option<Roman>,
    decrypted: Option<Roman>,
    status: String,
}

impl App {
    fn new() -> Self {
        App {
            keys: None,
            message: None,
            cipher: None,
            decrypted: None,
            status: "SALVE! Press 'G' to generate keys.".to_string(),
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('g') => {
                            app.status = "GENERATING KEYS... (Computing Primes)".to_string();
                            terminal.draw(|f| ui(f, app))?; // Force redraw
                            // Generate small keys (e.g. 16 bits) to be fast but impressive
                            app.keys = Some(generate_keys(16));
                            app.status = "KEYS GENERATED. Press 'E' to Encrypt.".to_string();
                            app.message = None;
                            app.cipher = None;
                            app.decrypted = None;
                        }
                        KeyCode::Char('e') => {
                            if let Some(ref keys) = app.keys {
                                app.status = "ENCRYPTING...".to_string();
                                terminal.draw(|f| ui(f, app))?;
                                // Encrypt "XLII" (42) as demo
                                // Or maybe random number?
                                // Let's use 42 for consistency with tests.
                                let msg = Roman::from_u64(42);
                                let c = encrypt(&msg, keys);
                                app.message = Some(msg);
                                app.cipher = Some(c);
                                app.decrypted = None;
                                app.status = "ENCRYPTED. Press 'D' to Decrypt.".to_string();
                            } else {
                                app.status = "NEED KEYS FIRST! Press 'G'.".to_string();
                            }
                        }
                        KeyCode::Char('d') => {
                            if let Some(ref keys) = app.keys {
                                if let Some(ref c) = app.cipher {
                                    app.status = "DECRYPTING...".to_string();
                                    terminal.draw(|f| ui(f, app))?;
                                    let m = decrypt(c, keys);
                                    app.decrypted = Some(m);
                                    app.status = "DECRYPTED. AVE CAESAR!".to_string();
                                } else {
                                    app.status = "NOTHING TO DECRYPT. Press 'E'.".to_string();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("SPQR RSA: CRYPTOGRAPHIA ROMANA")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Tablets area
    let mut key_text = String::new();
    if let Some(ref k) = app.keys {
        key_text.push_str(&format!("MODULUS (n): {}\n\n", k.modulus));
        key_text.push_str(&format!("PUBLICUS (e): {}\n\n", k.public));
        key_text.push_str(&format!("PRIVATUS (d): {}\n", k.private));
    } else {
        key_text.push_str("Tablets are empty.\nWait for the Scribe to carve them.\nPress 'G' to summon the Scribe.");
    }

    let keys_block = Paragraph::new(key_text)
        .block(
            Block::default()
                .title("CLAVES (Keys)")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(keys_block, chunks[1]);

    // Process area
    let mut process_text = String::new();
    if let Some(ref m) = app.message {
        process_text.push_str(&format!("NUNTIUS (Message): {}\n", m));
    }
    if let Some(ref c) = app.cipher {
        process_text.push_str(&format!("CRYPTA (Cipher): {}\n", c));
    }
    if let Some(ref d) = app.decrypted {
        process_text.push_str(&format!("REVELATIO (Decrypted): {}\n", d));
    }

    let process_block = Paragraph::new(process_text)
        .block(
            Block::default()
                .title("OPERATIO (Operation)")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(process_block, chunks[2]);

    let footer = Paragraph::new(format!(
        "STATUS: {} | [G]enerate [E]ncrypt [D]ecrypt [Q]uit",
        app.status
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}
