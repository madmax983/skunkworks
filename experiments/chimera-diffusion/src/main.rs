use anyhow::Result;
use chimera_lang::prelude::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gray_scott::GrayScott;
use locus::Vec2;
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

struct Organism {
    vm: ChimeraVM,
    pos: Vec2,
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

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

fn run_app<B: Backend + std::marker::Send + std::marker::Sync>(terminal: &mut Terminal<B>) -> Result<()>
where
    <B as Backend>::Error: std::marker::Send + std::marker::Sync + 'static
{
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    let mut rng = rand::thread_rng();

    let grid_width = 120;
    let grid_height = 60;
    let mut gs = GrayScott::new(grid_width, grid_height);

    // Initial chemical seed
    for _ in 0..10 {
        let x = rng.gen_range(20..grid_width - 20);
        let y = rng.gen_range(20..grid_height - 20);
        for dy in 0..5 {
            for dx in 0..5 {
                gs.add_chemical(x + dx, y + dy, 0.9);
            }
        }
    }

    // Organisms:
    // DNA: [ Siphon, Jump(0) ]
    let genes = vec![
        Gene {
            op: OpCode::Siphon,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];

    let dna = Dna::from_genes(genes);
    let mut organisms = Vec::new();
    for _ in 0..10 {
        organisms.push(Organism {
            vm: ChimeraVM::new(dna.clone()),
            pos: Vec2::new(
                rng.gen_range(0.0..grid_width as f64),
                rng.gen_range(0.0..grid_height as f64),
            ),
        });
    }

    loop {
        terminal.draw(|f| ui(f, &gs, &organisms))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            gs.update(0.055, 0.062, 1.0);

            for org in organisms.iter_mut() {
                // Agent deposits V chemical to alter the substrate
                let x = org.pos.x as usize;
                let y = org.pos.y as usize;
                if x < grid_width && y < grid_height {
                    let idx = gs.get_index(x, y);
                    let v_mut = gs.v_mut();
                    v_mut[idx] = (v_mut[idx] + 0.5).min(1.0);
                }

                // VM step
                org.vm.step();

                // Move randomly based on the VM state (e.g. if the stack has an element)
                let move_x = rng.gen_range(-1.0..2.0);
                let move_y = rng.gen_range(-1.0..2.0);
                org.pos.x = (org.pos.x + move_x).clamp(0.0, (grid_width - 1) as f64);
                org.pos.y = (org.pos.y + move_y).clamp(0.0, (grid_height - 1) as f64);
            }

            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, gs: &GrayScott, organisms: &[Organism]) {
    let size = f.area();
    // Use size.width.saturating_sub(2) and size.height.saturating_sub(2) due to borders
    let max_w = size.width.saturating_sub(2) as usize;
    let max_h = size.height.saturating_sub(2) as usize;

    let width = gs.width().min(max_w);
    let height = gs.height().min(max_h);

    let mut output = String::new();
    let v_buf = gs.v();

    for y in 0..height {
        for x in 0..width {
            let mut is_agent = false;
            for org in organisms {
                if org.pos.x as usize == x && org.pos.y as usize == y {
                    is_agent = true;
                    break;
                }
            }

            if is_agent {
                output.push('@');
            } else {
                let idx = gs.get_index(x, y);
                let val = v_buf[idx];
                let char_val = if val > 0.5 {
                    '#'
                } else if val > 0.3 {
                    '+'
                } else if val > 0.1 {
                    '.'
                } else {
                    ' '
                };
                output.push(char_val);
            }
        }
        if y < height - 1 {
            output.push('\n');
        }
    }

    let p = Paragraph::new(output).block(Block::default().borders(Borders::ALL).title("Chimera Diffusion"));
    f.render_widget(p, size);
}
