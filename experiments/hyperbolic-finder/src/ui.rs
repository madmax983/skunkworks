use macroquad::prelude::*;

pub struct Button {
    text: String,
    rect: Rect,
}

impl Button {
    pub fn new(text: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            text: text.to_string(),
            rect: Rect::new(x, y, w, h),
        }
    }

    pub fn draw(&self) -> bool {
        let (mx, my) = mouse_position();
        let is_hover = self.rect.contains(vec2(mx, my));
        let is_clicked = is_hover && is_mouse_button_pressed(MouseButton::Left);

        let color = if is_clicked {
            DARKGRAY
        } else if is_hover {
            LIGHTGRAY
        } else {
            GRAY
        };

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            2.0,
            WHITE,
        );

        draw_text(
            &self.text,
            self.rect.x + 10.0,
            self.rect.y + 20.0,
            20.0,
            BLACK,
        );

        is_clicked
    }
}
