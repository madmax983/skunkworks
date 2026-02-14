use crate::sim::Cortex;
use ratatui::{
    style::Color,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
    },
    Frame,
};

pub fn draw(f: &mut Frame, cortex: &Cortex) {
    let size = f.area();
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Granular Cortex "))
        .x_bounds([0.0, cortex.width as f64])
        .y_bounds([0.0, cortex.height as f64])
        .paint(|ctx| {
            let mut red_points = Vec::new();
            let mut yellow_points = Vec::new();
            let mut green_points = Vec::new();
            let mut blue_points = Vec::new();
            let mut gray_points = Vec::new();

            for neuron in &cortex.neurons {
                let pt = (neuron.x as f64, neuron.y as f64);
                let v = neuron.izh.v;

                if v >= 20.0 {
                    red_points.push(pt);
                } else if v >= -40.0 {
                    yellow_points.push(pt);
                } else if v >= -55.0 {
                    green_points.push(pt);
                } else if v >= -65.0 {
                    blue_points.push(pt);
                } else {
                    gray_points.push(pt);
                }
            }

            ctx.draw(&Points { coords: &gray_points, color: Color::DarkGray });
            ctx.draw(&Points { coords: &blue_points, color: Color::Blue });
            ctx.draw(&Points { coords: &green_points, color: Color::Green });
            ctx.draw(&Points { coords: &yellow_points, color: Color::Yellow });
            // Draw red last (on top)
            ctx.draw(&Points { coords: &red_points, color: Color::Red });
        });

    f.render_widget(canvas, size);
}
