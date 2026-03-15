use proptest::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use tui_shared::TensionBar;

proptest! {
    #[test]
    fn test_tension_bar_underflow(
        height in 1..10u16,
        tension in 1.1..=10.0f64,
    ) {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, height));
        let widget = TensionBar::new(tension).block(Block::default().borders(Borders::ALL));

        widget.render(Rect::new(0, 0, 10, height), &mut buffer);
    }
}
