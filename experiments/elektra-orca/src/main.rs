use anyhow::Result;
use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::{ChimeraVM, Value};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

fn main() -> Result<()> {
    // Enable raw mode and set up alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm.prologue_state.orca_mode = true;
    vm.energy = 1000;

    // Pre-seed some Orca structure: a clock and some generators
    // A clock (C) with some parameter, sending bangs (*)
    vm.grid[5][5] = Value::Str("C".to_string());
    vm.grid[6][5] = Value::Str("*".to_string());
    vm.grid[5][4] = Value::Int(3); // Clock rate

    // A power generator (🔌)
    vm.grid[8][5] = Value::Str("🔌".to_string());
    vm.grid[8][6] = Value::Str("~".to_string()); // Wire

    // A switch (⏧)
    vm.grid[8][7] = Value::Str("⏧".to_string());

    // A ground (≡)
    vm.grid[8][9] = Value::Str("≡".to_string());

    let mut cursor = (0, 0);

    // Track execution delays based on voltage
    let mut execution_delay_counter = 0.0;

    loop {
        // Evaluate Orca execution speed dynamically based on total grid voltage
        let mut total_voltage: f32 = 0.0;
        for y in 0..16 {
            for x in 0..16 {
                total_voltage += vm.voltage_grid[y][x].abs();
            }
        }

        let target_speed = (1.0 + total_voltage / 500.0).clamp(1.0, 5.0); // 1x to 5x speed
        execution_delay_counter += target_speed;

        while execution_delay_counter >= 1.0 {
            // Tick the VM
            vm.step();

            // Link Orca signals back to Elektra voltage
            for y in 0..16 {
                for x in 0..16 {
                    if vm.prologue_state.signal_grid[y][x].is_some() {
                        // An active signal injects a small voltage pulse
                        vm.voltage_grid[y][x] = (vm.voltage_grid[y][x] + 10.0).clamp(-100.0, 100.0);
                    }
                }
            }

            execution_delay_counter -= 1.0;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.area());

            // Render Orca Grid
            let mut grid_lines = Vec::new();
            for y in 0..16 {
                let mut spans = Vec::new();
                for x in 0..16 {
                    let val = &vm.grid[y][x];
                    let signal = vm.prologue_state.signal_grid[y][x].is_some();
                    let mut style = Style::default();

                    let ch = match val {
                        Value::Int(n) if *n == 0 => "·".to_string(),
                        Value::Int(n) => n.to_string(),
                        Value::Str(s) => s.clone(),
                        _ => "?".to_string(),
                    };

                    if ch != "·" {
                        style = style.fg(Color::Cyan);
                    } else {
                        style = style.fg(Color::DarkGray);
                    }

                    if signal {
                        style = style
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD);
                    }

                    if cursor == (x, y) {
                        style = style.bg(Color::Yellow).fg(Color::Black);
                    }

                    spans.push(Span::styled(ch, style));
                    spans.push(Span::raw(" "));
                }
                grid_lines.push(Line::from(spans));
            }
            let orca_widget = Paragraph::new(grid_lines)
                .block(Block::default().borders(Borders::ALL).title("Orca Logic"));
            f.render_widget(orca_widget, chunks[0]);

            // Render Elektra Voltage Grid
            let mut v_lines = Vec::new();
            for y in 0..16 {
                let mut spans = Vec::new();
                for x in 0..16 {
                    let v = vm.voltage_grid[y][x];
                    let r = vm.resistance_grid[y][x];

                    let mut style = Style::default();
                    let ch = if r == -1.0 {
                        style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
                        "+"
                    } else if r == -2.0 {
                        style = style.fg(Color::Blue).add_modifier(Modifier::BOLD);
                        "-"
                    } else if v.abs() > 0.1 {
                        let intensity = (v.abs() * 2.5).clamp(0.0, 255.0) as u8;
                        style = style.fg(Color::Rgb(intensity, intensity, 0));
                        if v.abs() > 50.0 {
                            "⚡"
                        } else if v.abs() > 10.0 {
                            "≈"
                        } else {
                            "~"
                        }
                    } else {
                        style = style.fg(Color::DarkGray);
                        "·"
                    };

                    if cursor == (x, y) {
                        style = style.bg(Color::Yellow).fg(Color::Black);
                    }

                    spans.push(Span::styled(ch.to_string(), style));
                    spans.push(Span::raw(" "));
                }
                v_lines.push(Line::from(spans));
            }
            let elektra_widget = Paragraph::new(v_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Elektra Circuit"),
            );
            f.render_widget(elektra_widget, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if cursor.1 > 0 {
                            cursor.1 -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if cursor.1 < 15 {
                            cursor.1 += 1;
                        }
                    }
                    KeyCode::Left => {
                        if cursor.0 > 0 {
                            cursor.0 -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if cursor.0 < 15 {
                            cursor.0 += 1;
                        }
                    }
                    KeyCode::Char(c) => {
                        if c.is_ascii_graphic() {
                            vm.grid[cursor.1][cursor.0] = Value::Str(c.to_string());
                        } else if c == ' ' {
                            vm.grid[cursor.1][cursor.0] = Value::Int(0);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
