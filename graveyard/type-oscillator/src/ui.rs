use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Points},
    Frame,
};

pub fn draw(f: &mut Frame, points: &[(f64, f64)]) {
    let size = f.area();

    // Calculate bounds
    let (min_x, max_x, min_y, max_y) = points.iter().fold(
        (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );

    let width = max_x - min_x;
    let height = max_y - min_y;

    // Add some padding
    let padding_x = width * 0.1;
    let padding_y = height * 0.1;

    let x_bounds = [min_x - padding_x, max_x + padding_x];
    // Flip Y bounds because Canvas 0,0 is bottom-left usually?
    // Ratatui Canvas: "Cartesian coordinates". Y increases upwards.
    // Font coordinates: Y increases upwards.
    // So it should match.
    let y_bounds = [min_y - padding_y, max_y + padding_y];

    let canvas = Canvas::default()
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Type Oscillator"),
        )
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: points,
                color: Color::Cyan,
            });
        });

    f.render_widget(canvas, size);
}
