use proptest::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_shared::Button;

proptest! {
    /// 👺 Havoc: Proving that the `Button` widget panics when rendering outside the buffer area.
    ///
    /// 🧨 **The Trigger:** The `Button` widget draws text using `buf.set_line` without first intersecting
    /// its rendering `area` with the `buf.area`. If the UI layout calculation shifts the button even
    /// slightly outside the terminal screen bounds, it crashes the entire application.
    #[test]
    #[should_panic(expected = "outside of buffer")]
    fn test_button_out_of_bounds_panic(
        x in 20..=50u16,
        y in 20..=50u16
    ) {
        let button = Button::new("Crash");
        // A buffer representing a 10x10 terminal screen
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));

        // Render the button completely outside the screen bounds
        let area = Rect::new(x, y, 10, 3);
        button.render(area, &mut buffer);
    }
}
