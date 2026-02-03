use crate::sim::World;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Context},
    },
};

pub fn draw(f: &mut Frame, world: &World) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // We can't resize the world here easily because we have immutable reference.
    // The main loop should handle resizing `world.width` and `world.height`.

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Typo Rain: Deleted Code Physics "),
        )
        .x_bounds([0.0, world.width])
        .y_bounds([world.height, 0.0]) // 0 at top for "falling" feel, or 0 at bottom?
        // In sim, we treated positive Y as down (gravity adds to Y).
        // Standard Canvas has Y going UP.
        // So if particle Y increases, it goes UP in standard canvas.
        // We want Y to go DOWN.
        // So we set y_bounds([0.0, height]) -> 0 at bottom.
        // Wait, if Y=0 is top, gravity increases Y.
        // If we want 0 at top, we set y_bounds([0.0, world.height])?
        // No, Ratatui canvas: y_bounds([min, max]).
        // If we want Y=0 at TOP, we can invert drawing Y?
        // Let's stick to standard physics: Y=0 is ground, Y+ is up.
        // Gravity: vy -= g.
        // My sim logic: `vy += gravity`. This implies Y is DOWN.
        // So Y=0 is top. Y=Height is bottom.
        // To render Y=0 at top on Canvas:
        // Canvas usually has Y increasing UPwards.
        // If we map bounds [height, 0.0], then min=height (bottom?), max=0 (top?).
        // Actually, let's just use `.y_bounds([world.height, 0.0])` which inverts the axis.
        // If Y=0, it's at max bound (0.0).
        // If Y=Height, it's at min bound (height).
        // Let's try `y_bounds([world.height, 0.0])`.
        .paint(|ctx| {
            for p in &world.particles {
                draw_particle(ctx, p);
            }
        });

    f.render_widget(canvas, chunks[0]);

    let stats = Paragraph::new(format!(
        "Particles: {} | [Space] Shake | [R] Refresh Diff | [Q] Quit",
        world.particles.len()
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats, chunks[1]);
}

fn draw_particle(ctx: &mut Context, p: &crate::sim::Particle) {
    let color = if p.settled {
        Color::DarkGray
    } else {
        Color::Red
    };

    ctx.print(
        p.x,
        p.y,
        Span::styled(p.char.to_string(), Style::default().fg(color)),
    );
}
