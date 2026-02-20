use macroquad::prelude::*;

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

    pub fn with_colors(mut self, normal: Color, hover: Color, active: Color) -> Self {
        self.normal_color = normal;
        self.hover_color = hover;
        self.active_color = active;
        self
    }

    pub fn draw(&self) -> bool {
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

        let border_color = if is_hover { WHITE } else { self.border_color };

        // Background
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg_color);

        // Border
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            2.0,
            border_color,
        );

        // Text
        let font_size = 20.0;
        let text_dims = measure_text(&self.text, None, font_size as u16, 1.0);

        let text_x = self.rect.x + (self.rect.w - text_dims.width) / 2.0;
        let text_y = self.rect.y + (self.rect.h + text_dims.height) / 2.0;

        draw_text(&self.text, text_x, text_y, font_size, self.text_color);

        is_clicked
    }
}
