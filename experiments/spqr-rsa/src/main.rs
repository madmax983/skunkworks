use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph, Wrap},
    style::{Color, Modifier, Style},
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
    status_type: StatusType,
}

enum StatusType {
    Info,
    Success,
    Error,
    Busy,
}

impl App {
    fn new() -> Self {
        App {
            keys: None,
            message: None,
            cipher: None,
            decrypted: None,
            status: "SALVE! Press 'G' to generate keys.".to_string(),
            status_type: StatusType::Info,
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

#[allow(clippy::collapsible_if)]
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
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('g') => {
                            app.status = "GENERATING KEYS... (Computing Primes)".to_string();
                            app.status_type = StatusType::Busy;
                            terminal.draw(|f| ui(f, app))?; // Force redraw

                            // Generate small keys (e.g. 16 bits) to be fast but impressive
                            app.keys = Some(generate_keys(16));

                            app.status = "KEYS GENERATED. Press 'E' to Encrypt.".to_string();
                            app.status_type = StatusType::Success;
                            app.message = None;
                            app.cipher = None;
                            app.decrypted = None;
                        }
                        KeyCode::Char('e') => {
                            if let Some(ref keys) = app.keys {
                                app.status = "ENCRYPTING...".to_string();
                                app.status_type = StatusType::Busy;
                                terminal.draw(|f| ui(f, app))?;

                                // Encrypt "XLII" (42) as demo
                                let msg = Roman::from_u64(42);
                                let c = encrypt(&msg, keys);
                                app.message = Some(msg);
                                app.cipher = Some(c);
                                app.decrypted = None;

                                app.status = "ENCRYPTED. Press 'D' to Decrypt.".to_string();
                                app.status_type = StatusType::Success;
                            } else {
                                app.status = "NEED KEYS FIRST! Press 'G'.".to_string();
                                app.status_type = StatusType::Error;
                            }
                        }
                        KeyCode::Char('d') => {
                            if let Some(ref keys) = app.keys {
                                if let Some(ref c) = app.cipher {
                                    app.status = "DECRYPTING...".to_string();
                                    app.status_type = StatusType::Busy;
                                    terminal.draw(|f| ui(f, app))?;

                                    let m = decrypt(c, keys);
                                    app.decrypted = Some(m);

                                    app.status = "DECRYPTED. AVE CAESAR!".to_string();
                                    app.status_type = StatusType::Success;
                                } else {
                                    app.status = "NOTHING TO DECRYPT. Press 'E'.".to_string();
                                    app.status_type = StatusType::Error;
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

    // Title Block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Yellow));

    let title = Paragraph::new(Span::styled(
            " SPQR RSA: CRYPTOGRAPHIA ROMANA ",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        ))
        .block(title_block)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Keys Area
    let keys_block_style = Block::default()
        .title(Span::styled(" CLAVES (Keys) ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Cyan));

    let mut key_lines = Vec::new();
    if let Some(ref k) = app.keys {
        key_lines.push(Line::from(vec![
            Span::styled("MODULUS (n): ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", k.modulus)),
        ]));
        key_lines.push(Line::from(""));
        key_lines.push(Line::from(vec![
            Span::styled("PUBLICUS (e): ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", k.public)),
        ]));
        key_lines.push(Line::from(""));
        key_lines.push(Line::from(vec![
            Span::styled("PRIVATUS (d): ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", k.private)),
        ]));
    } else {
        key_lines.push(Line::from(Span::styled("Tablets are empty.", Style::default().fg(Color::DarkGray))));
        key_lines.push(Line::from(Span::styled("Wait for the Scribe to carve them.", Style::default().fg(Color::DarkGray))));
        key_lines.push(Line::from(Span::styled("Press 'G' to summon the Scribe.", Style::default().fg(Color::Yellow))));
    }

    let keys_paragraph = Paragraph::new(key_lines)
        .block(keys_block_style)
        .wrap(Wrap { trim: true });
    f.render_widget(keys_paragraph, chunks[1]);

    // Process Area
    let process_block_style = Block::default()
        .title(Span::styled(" OPERATIO (Operation) ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Magenta));

    let mut process_lines = Vec::new();
    if let Some(ref m) = app.message {
        process_lines.push(Line::from(vec![
            Span::styled("NUNTIUS (Message): ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", m)),
        ]));
        process_lines.push(Line::from(""));
    }
    if let Some(ref c) = app.cipher {
        process_lines.push(Line::from(vec![
            Span::styled("CRYPTA (Cipher): ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}", c), Style::default().fg(Color::Red)),
        ]));
        process_lines.push(Line::from(""));
    }
    if let Some(ref d) = app.decrypted {
        process_lines.push(Line::from(vec![
            Span::styled("REVELATIO (Decrypted): ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}", d), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]));
    }

    if process_lines.is_empty() {
         process_lines.push(Line::from(Span::styled("No operations performed yet.", Style::default().fg(Color::DarkGray))));
    }

    let process_paragraph = Paragraph::new(process_lines)
        .block(process_block_style)
        .wrap(Wrap { trim: true });
    f.render_widget(process_paragraph, chunks[2]);

    // Footer / Status
    let status_color = match app.status_type {
        StatusType::Info => Color::White,
        StatusType::Success => Color::Green,
        StatusType::Error => Color::Red,
        StatusType::Busy => Color::Yellow,
    };

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(status_color));

    let footer_text = vec![
        Line::from(vec![
            Span::styled("STATUS: ", Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::styled(&app.status, Style::default().fg(status_color)),
        ]),
        Line::from(vec![
            Span::raw(" | "),
            Span::styled("[G]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("enerate "),
            Span::styled("[E]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("ncrypt "),
            Span::styled("[D]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("ecrypt "),
            Span::styled("[Q/Esc]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("uit"),
        ])
    ];

    let footer = Paragraph::new(footer_text)
        .block(footer_block)
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[3]);
}
