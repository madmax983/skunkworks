use proptest::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_shared::TensionBar;

proptest! {
    #[test]
    fn prop_test_tension_bar_no_panic(
        tension in -10.0..10.0f64,
        inner_y in 0..10u16,
        inner_height in 0..10u16
    ) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 200, 200));
        let widget = TensionBar::new(tension);
        let area = Rect::new(0, inner_y, 10, inner_height);
        widget.render(area, &mut buf);
    }
}
