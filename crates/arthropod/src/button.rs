use macroquad::prelude::*;

/// An interactive, immediate-mode button widget for `macroquad`.
///
/// The `Button` encapsulates its bounding box, styling colors, and text.
/// It detects mouse hover and clicks automatically when drawn, using
/// `macroquad`'s input systems.
///
/// ## Examples
///
/// ```no_run
/// use arthropod::Button;
/// use macroquad::prelude::*;
///
/// #[macroquad::main("Button Example")]
/// async fn main() {
///     let my_button = Button::new("Click Me", 50.0, 50.0, 150.0, 40.0)
///         .with_colors(RED, ORANGE, YELLOW);
///
///     loop {
///         clear_background(BLACK);
///
///         if my_button.draw() {
///             println!("Button clicked!");
///         }
///
///         next_frame().await;
///     }
/// }
/// ```
pub struct Button {
    text: String,
    rect: Rect,
    hover_color: Color,
    active_color: Color,
    normal_color: Color,
    border_color: Color,
    text_color: Color,
}

impl Button {
    /// Constructs a new `Button` with the specified text and bounding rectangle.
    ///
    /// By default, the button is styled with grayscale colors for normal, hover,
    /// and active states, with a light gray border and white text.
    ///
    /// # Arguments
    ///
    /// * `text` - The label displayed in the center of the button.
    /// * `x` - The X coordinate of the top-left corner.
    /// * `y` - The Y coordinate of the top-left corner.
    /// * `w` - The width of the button.
    /// * `h` - The height of the button.
    pub fn new(text: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            text: text.to_string(),
            rect: Rect::new(x, y, w, h),
            hover_color: Color::new(0.3, 0.3, 0.3, 1.0),
            active_color: Color::new(0.2, 0.2, 0.2, 1.0),
            normal_color: Color::new(0.1, 0.1, 0.1, 0.8),
            border_color: LIGHTGRAY,
            text_color: WHITE,
        }
    }

    /// Overrides the default grayscale background colors with a custom palette.
    ///
    /// This method allows chaining (the builder pattern) during initialization.
    ///
    /// # Arguments
    ///
    /// * `normal` - The background color when the mouse is not interacting.
    /// * `hover` - The background color when the mouse cursor is over the button.
    /// * `active` - The background color when the mouse button is pressed down.
    pub fn with_colors(mut self, normal: Color, hover: Color, active: Color) -> Self {
        self.normal_color = normal;
        self.hover_color = hover;
        self.active_color = active;
        self
    }

    /// Renders the button to the screen and evaluates mouse interactions.
    ///
    /// This method performs immediate-mode drawing logic:
    /// 1. Queries the current `macroquad` mouse position.
    /// 2. Checks intersection with the button's bounding rectangle.
    /// 3. Evaluates mouse button state (down vs released).
    /// 4. Draws the background, border, and text dynamically based on interaction state.
    ///
    /// Evaluates to `true` on the exact frame the user releases the left mouse button
    /// over the widget (a complete "click").
    ///
    /// ## Panics
    ///
    /// This function does not panic. If the button's bounding rectangle has non-finite
    /// dimensions (e.g. `NaN`, `Infinity`) or the coordinates exceed the arbitrary
    /// limit of `100_000.0`, the function will return `false` early and not draw anything.
    /// This mitigates crashes in headless CI environments when extreme coordinates are supplied.
    pub fn draw(&self) -> bool {
        if !self.rect.x.is_finite()
            || !self.rect.y.is_finite()
            || !self.rect.w.is_finite()
            || !self.rect.h.is_finite()
            || self.rect.w <= 0.0
            || self.rect.h <= 0.0
            || self.rect.x > 100_000.0 // Arbitrary reasonable bound to prevent geometry explosion
            || self.rect.y > 100_000.0
        {
            return false;
        }

        let (mx, my) = mouse_position();
        let is_hover = self.rect.contains(vec2(mx, my));
        let is_clicked = is_hover && is_mouse_button_released(MouseButton::Left);
        let is_down = is_hover && is_mouse_button_down(MouseButton::Left);

        let bg_color = if is_down {
            self.active_color
        } else if is_hover {
            self.hover_color
        } else {
            self.normal_color
        };

        let border_color = if is_down {
            YELLOW
        } else if is_hover {
            WHITE
        } else {
            self.border_color
        };

        let text_color = if is_down {
            YELLOW
        } else if is_hover {
            WHITE
        } else {
            self.text_color
        };

        // Shadow (drawn fixed behind the button)
        draw_rectangle(
            self.rect.x,
            self.rect.y + 4.0,
            self.rect.w,
            self.rect.h,
            BLACK,
        );

        // Offset when clicked
        let offset_y = if is_down { 4.0 } else { 0.0 };

        // Background
        draw_rectangle(
            self.rect.x,
            self.rect.y + offset_y,
            self.rect.w,
            self.rect.h,
            bg_color,
        );

        // Border
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y + offset_y,
            self.rect.w,
            self.rect.h,
            2.0,
            border_color,
        );

        // Text
        let font_size = 20.0;
        let text_dims = measure_text(&self.text, None, font_size as u16, 1.0);

        let text_x = self.rect.x + (self.rect.w - text_dims.width) / 2.0;
        let text_y = self.rect.y + offset_y + (self.rect.h + text_dims.height) / 2.0;

        draw_text(&self.text, text_x, text_y, font_size, text_color);

        is_clicked
    }
}
