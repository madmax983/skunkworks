use proptest::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    widgets::Block,
    widgets::Borders,
};
use tui_shared::Button;

proptest! {
    /// 👺 Havoc: Proving that the `Button` widget panics when rendering within an area
    /// where `inner_area.height` evaluates to 0.
    ///
    /// 🧨 **The Trigger:** A height that allows borders but leaves 0 inner height,
    /// so that `inner_area.height < 1`. Without bounds checking, calculating the Y coordinate
    /// for text placement can exceed the actual screen space.
    #[test]
    #[should_panic(expected = "outside of buffer")]
    fn test_button_height_zero_panic(
        height in 0..=2u16
    ) {
        let button = Button::new("Test").block(Block::default().borders(Borders::ALL));
        let area = Rect::new(0, 0, 40, height);
        let mut buffer = Buffer::empty(area);

        button.render(area, &mut buffer);
    }
}
