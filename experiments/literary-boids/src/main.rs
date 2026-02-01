use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};

pub mod boid;
#[cfg(feature = "nova")]
pub mod critic;
#[cfg(feature = "nova")]
pub mod syntax_physics;
#[cfg(feature = "nova")]
pub mod traces;
pub mod world;

use world::World;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    world: World,
    running: bool,
    #[cfg(feature = "nova")]
    traces: traces::TraceLayer,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        let text = "One morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin. He lay on his armour-like back, and if he lifted his head a little he could see his brown belly, slightly domed and divided by arches into stiff sections. The bedding was hardly able to cover it and seemed ready to slide off any moment. His many legs, pitifully thin compared with the size of the rest of him, waved about helplessly as he looked. \"What's happened to me?\" he thought. It wasn't a dream. His room, a proper human room although a little too small, lay peacefully between its four familiar walls. A collection of textile samples lay spread out on the table - Samsa was a travelling salesman - and above it there hung a picture that he had recently cut out of an illustrated magazine and housed in a nice, gilded frame. It showed a lady fitted out with a fur hat and fur boa who sat upright, raising a heavy fur muff that covered the whole of her lower arm towards the viewer. Gregor then turned to look out the window at the dull weather. Drops of rain could be heard hitting the pane, which made him feel quite sad. \"How about if I sleep a little bit longer and forget all this nonsense\", he thought, but that was something he was unable to do because he was used to sleeping on his right, and in his present state couldn't get into that position. However hard he threw himself onto his right, he always rolled back to where he was. He must have tried it a hundred times, shut his eyes so that he wouldn't have to look at the floundering legs, and only stopped when he began to feel a mild, dull pain there that he had never felt before.".to_string();

        Self {
            world: World::new(width, height, text),
            running: true,
            #[cfg(feature = "nova")]
            traces: traces::TraceLayer::new(500),
        }
    }

    fn on_tick(&mut self) {
        self.world.update();

        #[cfg(feature = "nova")]
        {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            for boid in &self.world.boids {
                // 2% chance per tick to leave a trace
                if rng.gen_bool(0.02) {
                    self.traces.add(traces::Trace {
                        position: boid.position,
                        content: boid.dna.char_representation,
                        color: boid.dna.color,
                        lifetime: 100.0,
                    });
                }
            }
            self.traces.update();
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let _size = terminal.size()?;
    // Map terminal size to world size (1 char = 1 unit? or scaled?)
    // Let's use 1:1 for simplicity, maybe scale if too small.
    // 2x scale for height to account for aspect ratio of chars?
    // Let's just use float coordinates 0..100, 0..100 and map them to canvas.

    let world_width = 200.0;
    let world_height = 200.0;

    let mut app = App::new(world_width, world_height);

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                // Check if key is 'q' to quit
                let is_q = key.code == KeyCode::Char('q');
                if key.kind == KeyEventKind::Press && is_q {
                    app.running = false;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area()); // Changed from f.size() to f.area() for ratatui 0.26+

    // Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Literary Boids"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            #[cfg(feature = "nova")]
            app.traces.draw(ctx);

            // Draw Food
            for food in &app.world.food {
                ctx.print(
                    food.position.0,
                    food.position.1,
                    Span::styled(food.content.to_string(), Style::default().fg(Color::Green)),
                );
            }

            // Draw Boids
            for boid in &app.world.boids {
                ctx.print(
                    boid.position.0,
                    boid.position.1,
                    Span::styled(
                        boid.dna.char_representation.to_string(),
                        Style::default().fg(boid.dna.color),
                    ),
                );
            }

            #[cfg(feature = "nova")]
            for critic in &app.world.critics {
                ctx.print(
                    critic.position.0,
                    critic.position.1,
                    Span::styled(
                        critic.symbol.to_string(),
                        Style::default().fg(critic.color).add_modifier(ratatui::style::Modifier::BOLD),
                    ),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status bar
    let boid_count = app.world.boids.len();
    let food_count = app.world.food.len();

    #[cfg(feature = "nova")]
    let status = format!(
        "Boids: {} | Food: {} | Critics: {} | Press 'q' to quit",
        boid_count,
        food_count,
        app.world.critics.len()
    );

    #[cfg(not(feature = "nova"))]
    let status = format!(
        "Boids: {} | Food: {} | Press 'q' to quit",
        boid_count, food_count
    );

    let p = Paragraph::new(status).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(p, chunks[1]);
}
