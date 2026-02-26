use ratatui::widgets::canvas::Context;

/// A visual component representing a fishing bobber on a Canvas.
///
/// This is not a standard `Widget` but a helper for drawing onto a `Canvas` context.
/// It encapsulates the visual design of the bobber, including state-dependent icons (hooked vs idle)
/// and environmental effects (ripples, splashes).
pub struct Bobber {
    /// X coordinate on the Canvas (0.0 - 100.0 usually).
    pub x: f64,
    /// Y coordinate on the Canvas.
    pub y: f64,
    /// Whether a fish is currently hooked.
    pub is_hooked: bool,
}

impl Bobber {
    /// Creates a new Bobber.
    pub fn new(x: f64, y: f64, is_hooked: bool) -> Self {
        Self { x, y, is_hooked }
    }

    /// Draws the bobber and its effects onto the given Canvas context.
    pub fn draw(&self, ctx: &mut Context) {
        let icon = if self.is_hooked { "🔴" } else { "⚪" };

        // Draw the main bobber body
        ctx.print(self.x, self.y, icon);

        // Draw effects
        if self.is_hooked {
            // Splash effects for hooked state
            ctx.print(self.x - 3.0, self.y + 1.0, "💦");
            ctx.print(self.x + 3.0, self.y + 2.0, "∴");
            ctx.print(self.x - 2.0, self.y + 2.0, "°");
            ctx.print(self.x + 4.0, self.y + 1.0, "∷");
        } else {
            // Gentle ripples for idle state
            ctx.print(self.x - 2.0, self.y, "≈");
            ctx.print(self.x + 2.0, self.y, "≈");
        }
    }
}
