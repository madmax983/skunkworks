use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use sha2::{Digest, Sha256};
use rand::Rng;
use chimera_lang::prelude::*;
use strum::IntoEnumIterator;

mod rune;

struct Genome {
    dna: Vec<u8>,
}

impl Genome {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let len = rng.gen_range(16..128);
        let dna: Vec<u8> = (0..len).map(|_| rng.gen()).collect();
        Self { dna }
    }

    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.dna);
        hasher.finalize().to_vec()
    }

    fn crossover(a: &Self, b: &Self) -> Self {
        let mut rng = rand::thread_rng();
        if a.dna.is_empty() || b.dna.is_empty() { return Self::random(); }

        let cut_a = rng.gen_range(0..a.dna.len());
        let cut_b = rng.gen_range(0..b.dna.len());

        let mut new_dna = Vec::new();
        new_dna.extend_from_slice(&a.dna[..cut_a]);
        new_dna.extend_from_slice(&b.dna[cut_b..]);

        // Trim or pad to reasonable limits
        if new_dna.len() > 256 { new_dna.truncate(256); }
        if new_dna.len() < 16 {
            while new_dna.len() < 16 { new_dna.push(rng.gen()); }
        }

        Self { dna: new_dna }
    }

    fn mutate(&mut self) {
         let mut rng = rand::thread_rng();
         if self.dna.is_empty() { return; }

         // 10% chance to resize
         if rng.gen_bool(0.1) {
             if rng.gen_bool(0.5) {
                 self.dna.push(rng.gen());
             } else if self.dna.len() > 16 {
                 self.dna.pop();
             }
         }

         let idx = rng.gen_range(0..self.dna.len());
         self.dna[idx] = rng.gen();
    }
}

fn execute_dna(dna_bytes: &[u8]) -> String {
    let mut genes = Vec::new();
    let mut iter = dna_bytes.iter();
    let op_count = OpCode::iter().count();

    while let Some(&byte) = iter.next() {
        let op = OpCode::iter().nth(byte as usize % op_count).unwrap_or(OpCode::Nop);

        let args = match op {
            OpCode::Push => {
                if let Some(&arg) = iter.next() {
                    vec![Nucleotide::Number(arg as i64)]
                } else {
                    vec![Nucleotide::Number(0)]
                }
            },
             OpCode::Jump | OpCode::Brz | OpCode::Call => {
                if let Some(&arg) = iter.next() {
                    vec![Nucleotide::Number((arg % 16) as i64)]
                } else {
                    vec![Nucleotide::Number(0)]
                }
            },
            _ => vec![]
        };

        genes.push(Gene { op, args });
    }

    if genes.is_empty() { return "Empty Genome".to_string(); }

    let helix = Helix { strands: vec![Strand { genes }] };
    let dna = Dna { helix };

    // Create VM
    let mut vm = ChimeraVM::new(dna);

    let mut log = String::new();
    log.push_str("Running ChimeraVM...\n");

    for _ in 0..50 {
        vm.step(); // Returns ()
        if vm.energy <= 0 {
             log.push_str("Status: Died (0 Energy)\n");
             break;
        }
    }

    log.push_str(&format!("Energy: {}\nStack: {:?}", vm.energy, vm.stack));
    log
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut parent1 = Genome::random();
    let mut parent2 = Genome::random();
    let mut child = Genome::crossover(&parent1, &parent2);
    let mut output = String::new();

    loop {
        terminal.draw(|f| ui(f, &parent1, &parent2, &child, &output))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char(' ') => {
                    child = Genome::crossover(&parent1, &parent2);
                    output.clear();
                },
                 KeyCode::Char('m') => {
                    parent1.mutate();
                    parent2.mutate();
                    child = Genome::crossover(&parent1, &parent2);
                    output.clear();
                },
                KeyCode::Char('r') => {
                     parent1 = Genome::random();
                     parent2 = Genome::random();
                     child = Genome::crossover(&parent1, &parent2);
                     output.clear();
                },
                KeyCode::Enter => {
                    output = execute_dna(&child.dna);
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, p1: &Genome, p2: &Genome, child: &Genome, output: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[0]);

    draw_rune(f, top_chunks[0], p1, "Parent A");
    draw_rune(f, top_chunks[1], p2, "Parent B");
    draw_rune(f, top_chunks[2], child, "Child (Enter to Run)");

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let help = Paragraph::new(vec![
        Line::from("Controls:"),
        Line::from("  [Space] Breed (Crossover)"),
        Line::from("  [M]     Mutate Parents"),
        Line::from("  [R]     Randomize Parents"),
        Line::from("  [Enter] Execute Child DNA"),
        Line::from("  [Q]     Quit"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Chimera Runes Lab"));
    f.render_widget(help, bottom_chunks[0]);

    let log = Paragraph::new(output)
        .block(Block::default().borders(Borders::ALL).title("VM Output"));
    f.render_widget(log, bottom_chunks[1]);
}

fn draw_rune(f: &mut Frame, area: Rect, genome: &Genome, title: &str) {
    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);
    if inner_area.width == 0 || inner_area.height == 0 { return; }

    let w = inner_area.width as u32;
    let h = inner_area.height as u32 * 2;

    let img = rune::generate(&genome.hash(), w, h);

    for y in 0..inner_area.height {
        for x in 0..inner_area.width {
             let p1 = img.get_pixel(x as u32, y as u32 * 2);
             let p2 = img.get_pixel(x as u32, y as u32 * 2 + 1);

             let c1 = Color::Rgb(p1[0], p1[1], p1[2]);
             let c2 = Color::Rgb(p2[0], p2[1], p2[2]);

             f.buffer_mut().get_mut(inner_area.x + x, inner_area.y + y)
                .set_char('▀')
                .set_fg(c1)
                .set_bg(c2);
        }
    }
}
