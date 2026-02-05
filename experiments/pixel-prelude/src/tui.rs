use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use image::{DynamicImage, GenericImageView, imageops::FilterType};
use std::io;

pub fn run(img: DynamicImage) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, img);

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

struct App {
    img: DynamicImage,
    view_mode: ViewMode,
}

#[derive(PartialEq)]
enum ViewMode {
    Cover,
    Hidden,
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, img: DynamicImage) -> io::Result<()> {
    let mut app = App {
        img,
        view_mode: ViewMode::Cover,
    };

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char(' ') => {
                        app.view_mode = match app.view_mode {
                            ViewMode::Cover => ViewMode::Hidden,
                            ViewMode::Hidden => ViewMode::Cover,
                        };
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Render Image
    // Use inner area for calculations to account for borders
    let block = Block::default().borders(Borders::ALL).title("Image Preview");
    let inner_area = block.inner(chunks[0]);

    let width = inner_area.width as u32;
    let height = inner_area.height as u32;

    // Avoid panic if area is too small
    if width > 0 && height > 0 {
        // Resize image to fit area
        let resized = app.img.resize_exact(width, height, FilterType::Nearest);

        let mut content = String::new();

        for y in 0..resized.height() {
            for x in 0..resized.width() {
                let p = resized.get_pixel(x, y);

                let char = match app.view_mode {
                    ViewMode::Cover => {
                        // Grayscale
                        let lum = 0.2126 * p[0] as f32 + 0.7152 * p[1] as f32 + 0.0722 * p[2] as f32;
                        let map = " .:-=+*#%@";
                        let idx = ((lum / 255.0) * (map.len() - 1) as f32) as usize;
                        map.chars().nth(idx).unwrap_or(' ')
                    },
                    ViewMode::Hidden => {
                        // Show LSBs
                        let r = p[0] & 1;
                        let g = p[1] & 1;
                        let b = p[2] & 1;
                        let val = (r << 2) | (g << 1) | b;
                        // 0-7
                        match val {
                             0 => ' ', // 000 - No noise
                             1 => '.', // 001
                             2 => ',', // 010
                             3 => ';', // 011
                             4 => '-', // 100
                             5 => '+', // 101
                             6 => '%', // 110
                             7 => '#', // 111 - Max noise
                             _ => '?',
                        }
                    }
                };
                content.push(char);
            }
            content.push('\n');
        }

        let paragraph = Paragraph::new(content).block(block);
        f.render_widget(paragraph, chunks[0]);
    } else {
         f.render_widget(block, chunks[0]);
    }


    let help_text = match app.view_mode {
        ViewMode::Cover => "Viewing: COVER IMAGE (Grayscale) | Press <SPACE> to Reveal Hidden Layer | <Q> to Quit",
        ViewMode::Hidden => "Viewing: HIDDEN BITS (LSB Plane) | Press <SPACE> to Hide | <Q> to Quit",
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(if app.view_mode == ViewMode::Hidden { Color::Red } else { Color::Green }))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}
