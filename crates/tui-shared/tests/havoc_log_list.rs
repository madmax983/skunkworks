use proptest::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_shared::LogList;

proptest! {
    /// 👺 Havoc: Test Exploit for LogList layout and underflow vulnerabilities.
    #[test]
    fn test_log_list_havoc(
        width in 0..100u16,
        height in 0..100u16,
        logs_count in 0..100usize,
        title in ".*"
    ) {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 100));
        let area = Rect::new(0, 0, width, height);

        let mut logs = Vec::new();
        for i in 0..logs_count {
            logs.push(format!("Log message {}", i));
        }

        let widget = LogList::new(logs).with_title(&title);

        widget.render(area, &mut buffer);
    }
}
