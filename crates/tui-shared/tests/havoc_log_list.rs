use proptest::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_shared::LogList;

proptest! {
    /// 👺 Havoc: Test LogList against arbitrary strings.
    #[test]
    fn test_log_list_fuzzing(s in ".*") {
        let widget = LogList::new(vec![s]);
        let area = Rect::new(0, 0, 40, 10);
        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);
    }
}
