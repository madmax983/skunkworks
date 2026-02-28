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
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::Bobber;
    ///
    /// // Create an idle bobber at coordinates (50.0, 50.0)
    /// let idle_bobber = Bobber::new(50.0, 50.0, false);
    ///
    /// // Create a hooked bobber
    /// let hooked_bobber = Bobber::new(50.0, 50.0, true);
    /// ```
    pub fn new(x: f64, y: f64, is_hooked: bool) -> Self {
        Self { x, y, is_hooked }
    }

    /// Draws the bobber and its effects onto the given Canvas context.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The Ratatui Canvas Context.
    /// * `tick` - The current simulation tick, used for animation cycles.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_shared::Bobber;
    /// use ratatui::widgets::canvas::{Canvas, Context};
    /// use ratatui::layout::Rect;
    /// use ratatui::buffer::Buffer;
    /// use ratatui::widgets::Widget;
    ///
    /// let bobber = Bobber::new(50.0, 50.0, true);
    /// let current_tick = 42;
    ///
    /// let canvas = Canvas::default()
    ///     .x_bounds([0.0, 100.0])
    ///     .y_bounds([0.0, 100.0])
    ///     .paint(|ctx| {
    ///         bobber.draw(ctx, current_tick);
    ///     });
    ///
    /// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    /// canvas.render(Rect::new(0, 0, 10, 10), &mut buffer);
    /// ```
    pub fn draw(&self, ctx: &mut Context, tick: u64) {
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
            // Extra splash particles
            ctx.print(self.x - 4.0, self.y, "*");
            ctx.print(self.x + 5.0, self.y, "o");
            ctx.print(self.x, self.y + 3.0, "!");
        } else {
            // Gentle ripples for idle state
            ctx.print(self.x - 2.0, self.y, "≈");
            ctx.print(self.x + 2.0, self.y, "≈");
        }

        // Animated Splash / Ripple around bobber based on tick
        // This replaces manual logic previously in render_fishing
        if self.y < 50.0 {
            // Bobber is underwater/surface
            let phase = (tick % 6) / 2;
            let (left, right) = match phase {
                0 => ("(", ")"),
                1 => ("<", ">"),
                _ => ("{", "}"),
            };
            ctx.print(self.x - 2.0, self.y, left);
            ctx.print(self.x + 1.0, self.y, right);
        }
    }
}
