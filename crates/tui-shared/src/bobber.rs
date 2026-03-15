//! # Bobber 🎣
//!
//! Provides the [`Bobber`] component for rendering a fishing bobber onto a `ratatui`
//! `Canvas`.
//!
//! Unlike standard widgets, the `Bobber` is designed to be drawn directly onto a
//! `Context`, allowing it to integrate with continuous 2D simulation spaces rather
//! than strict layout grids. It visually communicates state changes (idle vs. hooked)
//! through different icons and animated water effects.

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
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::widgets::canvas::Canvas;
    use ratatui::widgets::Widget;

    #[test]
    fn test_bobber_idle_rendering() {
        let bobber = Bobber::new(5.0, 5.0, false);
        let tick = 0;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 20));

        let canvas = Canvas::default()
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|ctx| {
                bobber.draw(ctx, tick);
            });

        canvas.render(Rect::new(0, 0, 20, 20), &mut buffer);

        // Verify the bobber was drawn by checking for non-empty cells
        let mut found_bobber = false;
        let mut found_ripple = false;
        for y in 0..20 {
            for x in 0..20 {
                let cell = &buffer[(x, y)];
                if cell.symbol() == "⚪" {
                    found_bobber = true;
                } else if cell.symbol() == "≈" {
                    found_ripple = true;
                }
            }
        }

        assert!(found_bobber, "Should render idle bobber icon");
        assert!(found_ripple, "Should render ripples in idle state");
    }

    #[test]
    fn test_bobber_hooked_rendering() {
        let bobber = Bobber::new(5.0, 5.0, true);
        let tick = 0;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 20));

        let canvas = Canvas::default()
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|ctx| {
                bobber.draw(ctx, tick);
            });

        canvas.render(Rect::new(0, 0, 20, 20), &mut buffer);

        let mut found_bobber = false;
        let mut found_splash = false;
        for y in 0..20 {
            for x in 0..20 {
                let cell = &buffer[(x, y)];
                if cell.symbol() == "🔴" {
                    found_bobber = true;
                } else if cell.symbol() == "💦" {
                    found_splash = true;
                }
            }
        }

        assert!(found_bobber, "Should render hooked bobber icon");
        assert!(found_splash, "Should render splash effect in hooked state");
    }

    #[test]
    fn test_bobber_animation_phases() {
        // Test with y < 50.0 to trigger animation logic
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 20));

        for tick in 0..6 {
            let bobber = Bobber::new(5.0, 4.0, false);
            let canvas = Canvas::default()
                .x_bounds([0.0, 10.0])
                .y_bounds([0.0, 10.0])
                .paint(|ctx| {
                    bobber.draw(ctx, tick);
                });

            canvas.render(Rect::new(0, 0, 20, 20), &mut buffer);

            // Just verify we can draw it without panicking and it hits the branches
            let phase = (tick % 6) / 2;
            let mut found_left = false;
            let mut found_right = false;

            let expected_left = match phase {
                0 => "(",
                1 => "<",
                _ => "{",
            };

            let expected_right = match phase {
                0 => ")",
                1 => ">",
                _ => "}",
            };

            for y in 0..20 {
                for x in 0..20 {
                    let cell = &buffer[(x, y)];
                    if cell.symbol() == expected_left {
                        found_left = true;
                    }
                    if cell.symbol() == expected_right {
                        found_right = true;
                    }
                }
            }

            assert!(found_left, "Phase {} should have left animation", phase);
            assert!(found_right, "Phase {} should have right animation", phase);
        }
    }

    #[test]
    fn test_bobber_no_animation_above_surface() {
        // Test with y >= 50.0 to NOT trigger animation logic
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 20));
        let bobber = Bobber::new(5.0, 60.0, false);

        let canvas = Canvas::default()
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 100.0])
            .paint(|ctx| {
                bobber.draw(ctx, 0);
            });

        canvas.render(Rect::new(0, 0, 20, 20), &mut buffer);

        for y in 0..20 {
            for x in 0..20 {
                let cell = &buffer[(x, y)];
                // It should not draw the animation phases
                assert_ne!(cell.symbol(), "(");
                assert_ne!(cell.symbol(), ")");
                assert_ne!(cell.symbol(), "<");
                assert_ne!(cell.symbol(), ">");
                assert_ne!(cell.symbol(), "{");
                assert_ne!(cell.symbol(), "}");
            }
        }
    }
}
