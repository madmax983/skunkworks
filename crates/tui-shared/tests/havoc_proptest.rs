use tui_shared::Button;
use ratatui::widgets::Widget;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_button_label_fuzz(s in ".*", width in 0u16..100, height in 0u16..100) {
        let btn = Button::new(s);
        let area = Rect::new(0, 0, width, height);
        let mut buffer = Buffer::empty(area);

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            btn.render(area, &mut buffer);
        }));
    }
}

#[test]
#[should_panic(expected = "index outside of buffer")]
fn test_button_label_crash() {
    let btn = Button::new("a");
    let area = Rect::new(0, 0, 3, 0);
    let mut buffer = Buffer::empty(area);
    btn.render(area, &mut buffer);
}
