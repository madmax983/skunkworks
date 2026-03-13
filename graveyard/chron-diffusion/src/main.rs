use anyhow::Result;
use chrontext::blame::{BlameAnalyzer, LineInfo};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gray_scott::GrayScott;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{fs, io, path::PathBuf, time::Duration};

struct App {
    _path: PathBuf,
    content: Vec<String>,
    blame_info: Vec<LineInfo>,
    gs: GrayScott,
    scroll: usize,
    terminal_width: usize,
    terminal_height: usize,
}

impl App {
    fn new(path: PathBuf) -> Result<Self> {
        let content_str = fs::read_to_string(&path)?;
        let content: Vec<String> = content_str.lines().map(|s| s.to_string()).collect();

        // Start path is CWD
        let analyzer = BlameAnalyzer::new(".");
        let blame_info = analyzer.analyze(&path)?;

        let (width, height) = crossterm::terminal::size()?;

        let mut gs = GrayScott::new(width as usize, height as usize);

        // Seed the center
        gs.add_chemical((width / 2) as usize, (height / 2) as usize, 1.0);
        gs.add_chemical((width / 2 + 1) as usize, (height / 2) as usize, 1.0);
        gs.add_chemical((width / 2) as usize, (height / 2 + 1) as usize, 1.0);
        gs.add_chemical((width / 2 + 1) as usize, (height / 2 + 1) as usize, 1.0);

        Ok(Self {
            _path: path,
            content,
            blame_info,
            gs,
            scroll: 0,
            terminal_width: width as usize,
            terminal_height: height as usize,
        })
    }

    fn update_gs(&mut self) {
        // We will modify feed and kill rates based on the underlying code age.
        // We run a few steps of GS per frame for speed.

        // standard rates
        let base_f = 0.055;
        let base_k = 0.062;

        for _ in 0..10 {
            // Because GS update requires global feed/kill in the current implementation,
            // we will just use global update for now.
            // Wait, standard GS requires uniform f/k unless we modify it.
            // But we can apply chemicals directly or use a trick:
            // Since we can't modify f/k per cell without modifying the library,
            // we will simulate the age effect by artificially injecting U or V
            // based on code age at that location!

            self.gs.update(base_f, base_k, 1.0);
        }

        // Apply chronological influence:
        // New code adds V (catalyst)
        // Old code removes V (or adds U)
        let w = self.gs.width();
        let h = self.gs.height();

        for y in 0..h {
            let line_idx = self.scroll + y;
            if line_idx < self.content.len() {
                // Find age score for this line. LineInfo is 1-based.
                // It might not exactly match indices due to how BlameAnalyzer returns hunks.
                // Let's just find the first blame info that covers this line (roughly).
                // Or simply pre-map lines to age_score.
                let age_score = self
                    .blame_info
                    .iter()
                    .find(|b| b.line_number == line_idx + 1)
                    .map(|b| b.age_score)
                    .unwrap_or(0.5); // Default to middle

                let line_len = self.content[line_idx].len().min(w);

                for x in 0..line_len {
                    // Only apply effect where there is actual text
                    if self.content[line_idx].chars().nth(x).unwrap_or(' ') != ' ' {
                        // New code (score close to 1.0) -> high catalyst injection
                        // Old code (score close to 0.0) -> dampening
                        if age_score > 0.8 {
                            self.gs.add_chemical(x, y, 0.05); // Seed V
                        }
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        // Handle resize
        if area.width as usize != self.terminal_width
            || area.height as usize != self.terminal_height
        {
            self.terminal_width = area.width as usize;
            self.terminal_height = area.height as usize;

            // Re-init GS grid
            self.gs = GrayScott::new(self.terminal_width, self.terminal_height);
            // Seed the center
            self.gs
                .add_chemical(self.terminal_width / 2, self.terminal_height / 2, 1.0);
        }

        let mut lines = Vec::new();

        let w = self.gs.width();
        let h = self.gs.height();

        for y in 0..area.height as usize {
            let line_idx = self.scroll + y;
            let mut spans = Vec::new();

            if line_idx < self.content.len() {
                let text_line = &self.content[line_idx];
                let chars: Vec<char> = text_line.chars().collect();

                for x in 0..area.width as usize {
                    let v_val = if x < w && y < h {
                        self.gs.v()[self.gs.get_index(x, y)]
                    } else {
                        0.0
                    };

                    let ch = if x < chars.len() { chars[x] } else { ' ' };

                    // Color based on V concentration
                    // V ranges roughly 0.0 to 1.0.
                    // Let's map it to a fiery color scheme: Black -> Dark Red -> Red -> Yellow -> White
                    let r = (v_val * 255.0).clamp(0.0, 255.0) as u8;
                    let g = ((v_val - 0.3).max(0.0) * 300.0).clamp(0.0, 255.0) as u8;
                    let b = ((v_val - 0.7).max(0.0) * 500.0).clamp(0.0, 255.0) as u8;

                    let color = if v_val > 0.1 {
                        Color::Rgb(r, g, b)
                    } else {
                        Color::DarkGray
                    };

                    spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                }
            } else {
                // Empty lines might still have chemical V
                for x in 0..area.width as usize {
                    let v_val = if x < w && y < h {
                        self.gs.v()[self.gs.get_index(x, y)]
                    } else {
                        0.0
                    };

                    let r = (v_val * 255.0).clamp(0.0, 255.0) as u8;
                    let g = ((v_val - 0.3).max(0.0) * 300.0).clamp(0.0, 255.0) as u8;
                    let b = ((v_val - 0.7).max(0.0) * 500.0).clamp(0.0, 255.0) as u8;

                    let color = if v_val > 0.1 {
                        Color::Rgb(r, g, b)
                    } else {
                        Color::Reset
                    };

                    let ch = if v_val > 0.2 { '·' } else { ' ' };
                    spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                }
            }
            lines.push(Line::from(spans));
        }

        let p = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
        frame.render_widget(p, area);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chron-diffusion <file_path>");
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(path)?;

    loop {
        app.update_gs();

        terminal.draw(|f| app.draw(f))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.scroll < app.content.len().saturating_sub(1) {
                            app.scroll += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.scroll > 0 {
                            app.scroll -= 1;
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
