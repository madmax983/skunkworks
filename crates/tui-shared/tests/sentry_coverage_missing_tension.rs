use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};
use tui_shared::TensionBar;

#[test]
fn test_tension_bar_fractions() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));

    // Tension bar fractional symbol changes are horizontally bottom-up inside the inner block area
    // Let's test the widget to exercise these branches.
    let fractions = [0.05, 0.15, 0.30, 0.45, 0.60, 0.70, 0.80, 0.95];

    for tension in fractions {
        let bar = TensionBar::new(tension).block(Block::default());
        bar.render(Rect::new(0, 0, 10, 10), &mut buf);
    }
}
