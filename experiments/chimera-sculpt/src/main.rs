mod math;
mod render;

use anyhow::Result;
use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::{ChimeraVM, Value},
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};

use crate::math::Vec3;
use crate::render::SdfView;

// Scale factor for fixed point math in VM (1.0 = 100)
const SCALE: f64 = 100.0;

fn make_initial_dna() -> Dna {
    // A simple sphere: length(p) - r
    // Since we don't have sqrt, we'll use a Box approximation for now:
    // max(|x|, |y|, |z|) - r
    // Or Manhattan: |x| + |y| + |z| - r (Octahedron)

    // Let's do Octahedron (Manhattan distance) because it's easiest without conditional jumps for Abs
    // Wait, Abs is hard without jumps.
    // Let's do a simple plane: y - 0.

    // Actually, let's try to implement a simple "Sphere" using squared distance
    // and we'll take the sqrt of the result outside, assuming the VM outputs d^2?
    // No, that doesn't work for combination operations.

    // Let's implement Abs using a helper or just arithmetic tricks?
    // (x^2)^0.5 ? No.

    // Let's assume the VM calculates: x*x + y*y + z*z - r*r
    // And we return that.
    // This is not a distance, but we can try to render it as an implicit surface f(p)=0.
    // Raymarching might glitch.

    // Ops:
    // Stack on entry: [z, y, x] (pushed in reverse order of args usually, let's decide: push x, push y, push z)
    // So stack top is z.

    // Genes:
    // Dup Mul -> z*z
    // Swap -> z*z, y
    // Dup Mul -> z*z, y*y
    // Add -> z*z + y*y
    // Swap -> sum, x
    // Dup Mul -> sum, x*x
    // Add -> x*x + y*y + z*z (R2)
    // Push(Rad^2) -> R2, 10000 (r=10 -> 100*100=10000?? No r=10 world units -> 1000 fixed point. 1000^2 = 1,000,000)
    // Sub -> dist_sq

    let genes = vec![
        // Stack: [x, y, z] (Top is z)
        Gene { op: OpCode::Dup, args: vec![] }, // z, z
        Gene { op: OpCode::Mul, args: vec![] }, // z*z
        Gene { op: OpCode::Swap, args: vec![] }, // z*z, y
        Gene { op: OpCode::Dup, args: vec![] }, // y, y
        Gene { op: OpCode::Mul, args: vec![] }, // y*y
        Gene { op: OpCode::Add, args: vec![] }, // z*z + y*y
        Gene { op: OpCode::Swap, args: vec![] }, // sum, x
        Gene { op: OpCode::Dup, args: vec![] }, // x, x
        Gene { op: OpCode::Mul, args: vec![] }, // x*x
        Gene { op: OpCode::Add, args: vec![] }, // sum_sq (d^2)
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

// Helper to run VM statelessly
fn eval_sdf(dna: &Dna, p: Vec3, radius: f64) -> f64 {
    // 1. Create VM
    let mut vm = ChimeraVM::new(dna.clone());

    // 2. Push inputs (Fixed point)
    let ix = (p.x * SCALE) as i64;
    let iy = (p.y * SCALE) as i64;
    let iz = (p.z * SCALE) as i64;

    // Push x, y, z. Top will be z.
    vm.stack.push(Value::Int(ix));
    vm.stack.push(Value::Int(iy));
    vm.stack.push(Value::Int(iz));

    // 3. Run
    let max_ticks = 100;
    for _ in 0..max_ticks {
        if vm.halted { break; }
        vm.step();
    }

    // 4. Pop result
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(i) => {
                // The gene calculates "potential" (d^2).
                // We assume positive potential (outside sphere).
                let float_i = i as f64;
                if float_i < 0.0 { return 100.0; } // Error/Inside?

                // d = sqrt(potential) / SCALE
                let dist_from_origin = float_i.sqrt() / SCALE;
                dist_from_origin - radius
            },
            _ => 100.0,
        }
    } else {
        100.0
    }
}

struct App {
    camera_pos: Vec3,
    camera_target: Vec3,
    time: f64,
    should_quit: bool,
    dna: Dna,
    radius: f64,
}

impl App {
    fn new() -> Self {
        Self {
            camera_pos: Vec3::new(0.0, 0.0, -30.0),
            camera_target: Vec3::new(0.0, 0.0, 0.0),
            time: 0.0,
            should_quit: false,
            dna: make_initial_dna(),
            radius: 10.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.05;
    }

    fn mutate(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Mutate radius
        self.radius += rng.gen_range(-1.0..1.0);
        if self.radius < 1.0 { self.radius = 1.0; }

        // Mutate Genes (Swap logic or Add/Sub)
        if let Some(strand) = self.dna.helix.strands.get_mut(0) {
            let idx = rng.gen_range(0..strand.genes.len());
            let gene = &mut strand.genes[idx];
            match gene.op {
                 OpCode::Add => if rng.gen_bool(0.1) { gene.op = OpCode::Sub },
                 OpCode::Mul => if rng.gen_bool(0.1) { gene.op = OpCode::Add },
                 _ => {}
            }
        }
    }
}

fn main() -> Result<()> {
    // Setup TUI
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    // Restore TUI
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        let dna_clone = app.dna.clone();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let radius = app.radius;
            let scene_sdf = move |p: Vec3| {
                eval_sdf(&dna_clone, p, radius)
            };

            let view = SdfView::new(scene_sdf, app.camera_pos, app.camera_target, app.time);
            f.render_widget(view, chunks[0]);

            let text = vec![Line::from(vec![
                Span::raw("WASD: Move | M: Mutate | Q: Quit"),
            ])];
            let info = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?; // ratatui 0.30 returns io::Result, we are in anyhow::Result function, so ? works if mapped?
             // anyhow::Result handles io::Error auto conversion.

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('w') => app.camera_pos.z += 1.0,
                        KeyCode::Char('s') => app.camera_pos.z -= 1.0,
                        KeyCode::Char('a') => app.camera_pos.x -= 1.0,
                        KeyCode::Char('d') => app.camera_pos.x += 1.0,
                        KeyCode::Char('m') => app.mutate(),
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }

        app.tick();
    }
    Ok(())
}
