use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::retina::Retina;
use crate::eye::Eye;

pub struct App {
    pub retina: Retina,
    pub eye: Eye,
    pub should_quit: bool,
    pub content: Vec<String>,
    pub frame_count: u64,
}

impl App {
    pub fn new(width: usize, height: usize, content: String) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let retina = Retina::new(width, height);
        // Ensure eye dimensions match retina
        let mut eye = Eye::new(width as f32, height as f32);
        // We set the target to the center initially
        eye.target_x = (width / 2) as f32;
        eye.target_y = (height / 2) as f32;

        Self {
            retina,
            eye,
            should_quit: false,
            content: lines,
            frame_count: 0,
        }
    }

    pub fn on_tick(&mut self) {
        self.frame_count += 1;

        // 1. Map content to retina inputs
        let width = self.retina.width;
        let height = self.retina.height;

        for y in 0..height {
            let line = if y < self.content.len() {
                &self.content[y]
            } else {
                ""
            };

            // Pad or truncate logic handled implicitly by loop
            for x in 0..width {
                if let Some(ch) = line.chars().nth(x) {
                    self.retina.set_input(x, y, ch);
                } else {
                    self.retina.set_input(x, y, ' ');
                }
            }
        }

        // 2. Update Physics
        self.retina.update(1.0); // dt = 1.0 ms

        // 3. Update Eye
        self.eye.update(&self.retina.spikes, width);
    }

    pub fn draw(&self, frame: &mut Frame) {
        let size = frame.area();

        // Render the text with foveated effect
        let mut spans = Vec::new();

        // We only render as much as fits in the terminal or retina buffer
        // For simplicity, we limit to retina height
        let render_height = std::cmp::min(size.height as usize, self.retina.height);

        for y in 0..render_height {
            let mut line_spans = Vec::new();

            // Get line content or empty
            let line_content = if y < self.content.len() {
                 self.content[y].clone()
            } else {
                 String::new()
            };

            // Iterate over columns
            for x in 0..size.width as usize {
                if x >= self.retina.width {
                    break;
                }

                let ch = line_content.chars().nth(x).unwrap_or(' ');

                let in_fovea = self.eye.in_fovea(x as f32, y as f32);
                let spike_idx = y * self.retina.width + x;

                let is_spiking = if spike_idx < self.retina.spikes.len() {
                     self.retina.spikes[spike_idx]
                } else {
                     false
                };

                // Determine style
                let style = if in_fovea {
                    if is_spiking {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    }
                } else {
                    // Periphery
                    if is_spiking {
                        Style::default().fg(Color::DarkGray).bg(Color::Red) // faint flash
                    } else {
                         Style::default().fg(Color::DarkGray)
                    }
                };

                // Replace char if periphery?
                let display_char = if in_fovea {
                    ch.to_string()
                } else {
                    // Peripheral vision sees blocks instead of text
                    if ch == ' ' { " ".to_string() } else { "░".to_string() }
                };

                line_spans.push(Span::styled(display_char, style));
            }
            spans.push(Line::from(line_spans));
        }

        let paragraph = Paragraph::new(spans)
            .block(Block::default().borders(Borders::ALL).title(format!("Foveated Code | Eye: ({:.1}, {:.1})", self.eye.x, self.eye.y)));

        frame.render_widget(paragraph, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_app_draw() {
        let content = "fn main() {\n    println!(\"Hello\");\n}".to_string();
        let width = 40;
        let height = 10;
        let mut app = App::new(width, height, content);

        // Force eye to center
        app.eye.x = 20.0;
        app.eye.y = 5.0;

        let backend = TestBackend::new(width as u16, height as u16);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| app.draw(f)).unwrap();

        let buffer = terminal.backend().buffer();
        // Verify buffer isn't empty
        assert_eq!(buffer.area.width, 40);

        // Debug output to see what was drawn (optional)
        // println!("{:?}", buffer);
    }
}
