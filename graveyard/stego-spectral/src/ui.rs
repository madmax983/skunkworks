use crate::app::{App, ViewMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph, Wrap,
    },
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("⚛️  SPECTRE-STEGO  ⚛️")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Main Content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Input
            Constraint::Percentage(70), // Visualizer
        ])
        .split(chunks[1]);

    // Input Area
    let input_block = Block::default()
        .title("Payload (Text)")
        .borders(Borders::ALL);
    let input = Paragraph::new(app.input.as_str())
        .block(input_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(input, main_chunks[0]);

    // Visualizer Area
    let view_mode_str = match app.view_mode {
        ViewMode::Normal => "Normal (Noise)",
        ViewMode::BitPlane => "Bit Plane (LSB)",
    };

    let canvas_block = Block::default()
        .title(format!("Visualizer [{}]", view_mode_str))
        .borders(Borders::ALL);

    // Using Canvas to render points.
    // This is computationally expensive for large grids in TUI, but for 64x32 it's fine.
    let canvas = Canvas::default()
        .block(canvas_block)
        .x_bounds([0.0, app.buffer.width as f64])
        .y_bounds([0.0, app.buffer.height as f64])
        .paint(|ctx| {
            for y in 0..app.buffer.height {
                for x in 0..app.buffer.width {
                    let idx = y * app.buffer.width + x;
                    let val = app.buffer.data[idx];

                    let color = match app.view_mode {
                        ViewMode::Normal => {
                            // Grayscale mapping
                            // Since terminal colors are limited, we threshold or use RGB if supported.
                            // Ratatui supports Rgb color.
                            Color::Rgb(val, val, val)
                        }
                        ViewMode::BitPlane => {
                            // LSB is 0 or 1.
                            // 0 -> Black, 1 -> Green (Matrix style)
                            if val & 1 == 1 {
                                Color::Green
                            } else {
                                Color::Black
                            }
                        }
                    };

                    // Draw point. Note: Canvas y increases upwards usually?
                    // Ratatui Canvas: (0,0) is bottom-left by default.
                    // Image coords: (0,0) is top-left.
                    // We flip Y to match image coords.
                    let draw_y = (app.buffer.height - 1 - y) as f64;

                    ctx.draw(&Points {
                        coords: &[(x as f64, draw_y)],
                        color,
                    });
                }
            }
        });

    frame.render_widget(canvas, main_chunks[1]);

    // Status Area
    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(
            "Status / Controls: [Esc] Quit, [Enter] Embed, [F2] Extract, [F5] Regen, [Tab] View",
        ));
    frame.render_widget(status, chunks[2]);
}
