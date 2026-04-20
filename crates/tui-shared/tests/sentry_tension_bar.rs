use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_shared::TensionBar;

#[test]
fn test_tension_bar_safe_area_zero() {
    let bar = TensionBar::new(1.0);
    // Render into an area that does not intersect with the buffer (or intersects to zero area)
    let area = Rect::new(100, 100, 10, 10);
    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10)); // buffer area is 0,0,10,10

    // This will cause safe_area to have width=0, height=0
    bar.render(area, &mut buf);
}
