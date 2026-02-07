use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::canvas::{Context, Line},
};
use crate::world::World;
use chimera_lang::vm::Chirality;

pub fn draw_world(ctx: &mut Context, world: &World) {
    for agent in &world.agents {
        let color = match agent.chirality {
            Chirality::Left => Color::Cyan,
            Chirality::Right => Color::Magenta,
        };

        let x = agent.pos.x as f64;
        let y = agent.pos.y as f64;

        // Symbol
        let sym = match agent.chirality {
            Chirality::Left => "L",
            Chirality::Right => "R",
        };
        ctx.print(x, y, Span::styled(sym, Style::default().fg(color)));

        // Velocity Vector
        ctx.draw(&Line {
            x1: x,
            y1: y,
            x2: x + agent.vel.x as f64 * 10.0,
            y2: y + agent.vel.y as f64 * 10.0,
            color,
        });
    }
}
