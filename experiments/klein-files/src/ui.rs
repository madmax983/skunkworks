use crate::app::App;
use crate::math;
use ratatui::symbols::Marker;
use ratatui::{
    prelude::*,
    widgets::{
        Block, Borders,
        canvas::{Canvas, Circle},
    },
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Klein Files (WASD to Rotate, +/- to Zoom, Q to Quit)"),
        )
        .marker(Marker::Braille)
        .x_bounds([-100.0, 100.0])
        .y_bounds([-50.0, 50.0]) // Aspect ratio correction roughly 2:1 for terminal chars
        .paint(|ctx| {
            for file in &app.files {
                // Calculate position
                let p3 = math::klein_bottle(file.u, file.v, 1.5); // radius 1.5

                if let Some((x, y)) = math::project(p3, app.rotation, app.camera_dist, app.zoom) {
                    let color = if file.is_dir {
                        Color::Cyan
                    } else {
                        Color::Green
                    };

                    // Draw point
                    ctx.draw(&Circle {
                        x,
                        y,
                        radius: 0.5,
                        color,
                    });
                }
            }
        });

    frame.render_widget(canvas, area);
}
