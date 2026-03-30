use proptest::prelude::*;
use tui_shared::TensionBar;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

proptest! {
    #[test]
    fn test_tension_bar_underflow_fixed(
        tension in -10.0..10.0f64,
        inner_y in 0..100u16,
        inner_height in 0..100u16,
    ) {
        // By manipulating tension and area, try to reproduce draw_y panics
        // which might have occurred if unchecked_sub was used originally
        let widget = TensionBar::new(tension);
        let area = Rect::new(0, inner_y, 10, inner_height);

        // Ensure buffer bounds handle weird inputs
        let buf_height = (inner_y as u32 + inner_height as u32 + 10).min(u16::MAX as u32) as u16;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, buf_height));

        widget.render(area, &mut buffer);
    }
}
