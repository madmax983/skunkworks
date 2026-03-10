use ratatui::{backend::TestBackend, layout::Rect, Terminal};
use tui_shared::TensionBar;

#[test]
fn test_tension_bar_bobber_underflow_regression() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    // With tension at exactly 1.0, and Y at 0, previously the subtraction of 1 caused an underflow.
    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 10, 10);
            let widget = TensionBar::new(1.0).with_bobber(true);
            f.render_widget(widget, area);
        })
        .unwrap();
}
