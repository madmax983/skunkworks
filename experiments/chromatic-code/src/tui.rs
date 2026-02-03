use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use image::RgbImage;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub fn run(image: RgbImage, decoded_text: String) -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new(image, decoded_text);

    loop {
        tui.terminal.draw(|f| app.render(f))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        app.tick();

        if app.is_done() {
            // Optional: Auto-exit or just wait?
            // Let's just wait for user to quit.
        }
    }

    Ok(())
}

struct App {
    image: RgbImage,
    full_text: String,
    revealed_count: usize,
    last_tick: Instant,
}

impl App {
    fn new(image: RgbImage, text: String) -> Self {
        Self {
            image,
            full_text: text,
            revealed_count: 0,
            last_tick: Instant::now(),
        }
    }

    fn tick(&mut self) {
        // Reveal text over time (matrix style)
        if self.revealed_count < self.full_text.len() {
            let elapsed = self.last_tick.elapsed();
            if elapsed >= Duration::from_millis(10) {
                // Reveal speed: 5 chars per tick
                self.revealed_count = (self.revealed_count + 5).min(self.full_text.len());
                self.last_tick = Instant::now();
            }
        }
    }

    fn is_done(&self) -> bool {
        self.revealed_count >= self.full_text.len()
    }

    fn render(&self, f: &mut Frame) {
        let area = f.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Render Image
        let image_widget = ImageWidget { image: &self.image };
        f.render_widget(image_widget, chunks[0]);

        // Render Text
        let revealed_text = &self.full_text[..self.revealed_count];
        let paragraph = Paragraph::new(revealed_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Decoded Source"),
            )
            .style(Style::default().fg(Color::Green)); // Matrix green

        // Scroll to bottom if needed
        let scroll = if revealed_text.lines().count() as u16 > chunks[1].height - 2 {
            (revealed_text.lines().count() as u16).saturating_sub(chunks[1].height - 2)
        } else {
            0
        };

        f.render_widget(paragraph.scroll((scroll, 0)), chunks[1]);
    }
}

struct ImageWidget<'a> {
    image: &'a RgbImage,
}

impl<'a> Widget for ImageWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Simple resizing / crop: Just iterate over area and sample image
        // We use HalfBlock ▀. Top half is one pixel, Bottom half is another.
        // So for each cell (x, y), we look at image (x, 2*y) and (x, 2*y+1).

        // Scale logic: Fit image to area.
        let img_w = self.image.width();
        let img_h = self.image.height();

        let area_w = area.width;
        let area_h = area.height;

        if area_w == 0 || area_h == 0 {
            return;
        }

        for y in 0..area_h {
            for x in 0..area_w {
                // Map screen coordinates to image coordinates
                // We want to cover the area.
                // Simple nearest neighbor.

                let img_x = (x as u32 * img_w) / area_w as u32;
                let img_y_top = (y as u32 * 2 * img_h) / (area_h as u32 * 2);
                let img_y_bot = ((y as u32 * 2 + 1) * img_h) / (area_h as u32 * 2);

                if img_x < img_w && img_y_bot < img_h {
                    let p_top = self.image.get_pixel(img_x, img_y_top);
                    let p_bot = self.image.get_pixel(img_x, img_y_bot);

                    let c_top = Color::Rgb(p_top[0], p_top[1], p_top[2]);
                    let c_bot = Color::Rgb(p_bot[0], p_bot[1], p_bot[2]);

                    buf[(area.x + x, area.y + y)]
                        .set_char('▀')
                        .set_fg(c_top)
                        .set_bg(c_bot);
                }
            }
        }
    }
}
