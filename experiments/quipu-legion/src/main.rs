use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_bigint::BigUint;
use quipu::Cord;
use quipu_legion::crypto::{self, KeyPair};
use quipu_legion::roman::Roman;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::{io, time::Duration};

enum AppState {
    Welcome,
    GeneratingKeys,
    InputMessage,
    Encrypted(BigUint, Cord),  // Cipher value, Quipu representation
    Decrypted(BigUint, Roman), // Decrypted value, Roman representation
}

struct App {
    state: AppState,
    keypair: Option<KeyPair>,
    input_buffer: String,
    status_msg: String,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::Welcome,
            keypair: None,
            input_buffer: String::new(),
            status_msg: "Press 'G' to generate keys.".to_string(),
        }
    }

    fn generate_keys(&mut self) {
        self.state = AppState::GeneratingKeys;
        self.status_msg = "Generating 64-bit RSA keys...".to_string();
    }

    fn finish_generation(&mut self) {
        let keys = crypto::generate_keypair(64);
        self.keypair = Some(keys);
        self.state = AppState::InputMessage;
        self.status_msg = "Keys Generated! Enter a number or Roman Numeral to encrypt.".to_string();
        self.input_buffer.clear();
    }

    fn encrypt(&mut self) {
        if let Some(keys) = &self.keypair {
            // Parse input
            let val = if let Ok(n) = self.input_buffer.parse::<u64>() {
                BigUint::from(n)
            } else if let Ok(roman) = self.input_buffer.parse::<Roman>() {
                roman.value()
            } else {
                self.status_msg = "Invalid input! Use number or Roman (I, V, X...)".to_string();
                return;
            };

            if val >= keys.public.n {
                self.status_msg = format!(
                    "Message too large! Must be < N ({})",
                    Roman::from_biguint(keys.public.n.clone())
                );
                return;
            }

            let cipher = crypto::encrypt(&val, &keys.public);

            // Convert cipher to Quipu Cord (u64 limit for now)
            // BigUint -> u64 (truncate)
            let cipher_u64 = cipher.iter_u64_digits().next().unwrap_or(0);

            let cord = Cord::from(cipher_u64);
            self.state = AppState::Encrypted(cipher.clone(), cord);
            self.status_msg =
                "Message Encrypted! Displayed as Quipu Knots. Press 'D' to Decrypt.".to_string();
        }
    }

    fn decrypt(&mut self) {
        if let AppState::Encrypted(c, _) = &self.state {
            if let Some(keys) = &self.keypair {
                let m = crypto::decrypt(c, &keys.private);
                let roman = Roman::from_biguint(m.clone());
                self.state = AppState::Decrypted(m, roman);
                self.status_msg =
                    "Message Decrypted! Returned to Roman form. Press 'R' to Reset.".to_string();
            }
        }
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App
    let mut app = App::new();

    // Run loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    loop {
        // Handle heavy task outside draw if needed, or just block (simple)
        if let AppState::GeneratingKeys = app.state {
            terminal.draw(|f| ui(f, app))?;
            // Force a draw before blocking
            // Ideally we'd yield but this is synchronous.
            // Let's just run it.
            app.finish_generation();
            continue;
        }

        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    AppState::InputMessage => match key.code {
                        KeyCode::Char(c) => {
                            app.input_buffer.push(c.to_ascii_uppercase());
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            app.encrypt();
                        }
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    _ => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char('g') => app.generate_keys(),
                            KeyCode::Char('d') => {
                                if let AppState::Encrypted(_, _) = app.state {
                                    app.decrypt();
                                }
                            }
                            KeyCode::Char('r') => {
                                // Reset to input
                                if app.keypair.is_some() {
                                    app.state = AppState::InputMessage;
                                    app.input_buffer.clear();
                                    app.status_msg = "Enter new message.".to_string();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Status
        ])
        .split(f.area());

    let title = Paragraph::new("🧬 QUIPU-LEGION: Roman-Incan Cryptography")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Left Panel: Roman / Input
    let left_block = Block::default()
        .title(" SPQR (Roman) ")
        .borders(Borders::ALL);

    let left_content = match &app.state {
        AppState::Welcome => {
            "Welcome, Legate.\n\nPress 'G' to generate secure RSA keys for the Empire.".to_string()
        }
        AppState::GeneratingKeys => "Forging keys in the fires of Vulcan...".to_string(),
        AppState::InputMessage => {
            let mut s = String::new();
            if let Some(keys) = &app.keypair {
                s.push_str(&format!(
                    "Public Modulus (N):\n{}\n\n",
                    Roman::from_biguint(keys.public.n.clone())
                ));
                s.push_str(&format!(
                    "Public Exponent (E):\n{}\n\n",
                    Roman::from_biguint(keys.public.e.clone())
                ));
            }
            s.push_str("Enter Message (Decimal or Roman):\n> ");
            s.push_str(&app.input_buffer);
            s
        }
        AppState::Encrypted(_, _) => {
            let mut s = String::new();
            if let Some(keys) = &app.keypair {
                s.push_str(&format!(
                    "Public Modulus (N):\n{}\n\n",
                    Roman::from_biguint(keys.public.n.clone())
                ));
            }
            s.push_str("Message Encrypted.\n\nSee Right Panel for Quipu Ciphertext.");
            s
        }
        AppState::Decrypted(_, ref roman) => {
            format!("Decrypted Message:\n\n{}", roman)
        }
    };

    let left_p = Paragraph::new(left_content)
        .block(left_block)
        .wrap(Wrap { trim: true });
    f.render_widget(left_p, main_chunks[0]);

    // Right Panel: Quipu / Cipher
    let right_block = Block::default()
        .title(" QUIPU (Inca) ")
        .borders(Borders::ALL);
    let right_content = match &app.state {
        AppState::Encrypted(_, cord) => {
            format!("Ciphertext Cord:\n\n{}", cord)
        }
        AppState::Decrypted(_, _) => "Cord Untied.\nMessage revealed in Roman panel.".to_string(),
        _ => "Waiting for ciphertext...".to_string(),
    };

    let right_p = Paragraph::new(right_content)
        .block(right_block)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(right_p, main_chunks[1]);

    // Status Bar
    let status = Paragraph::new(app.status_msg.as_str())
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}
