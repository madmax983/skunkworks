use proptest::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use tui_shared::TensionBar;

proptest! {
    /// 🔒 Warden: Test Exploit for TensionBar underflow vulnerability.
    ///
    /// The `TensionBar` widget used to panic when rendering partial blocks with constrained heights
    /// because `draw_y` or `x` could exceed the bounds of the provided terminal `Buffer` (`buf.area`).
    ///
    /// 🧨 **The Threat:** Out-of-bounds indexing `buf[(x, draw_y)]` causing a deterministic panic DoS.
    /// 🛡️ **The Defense:** Explicit bounds checking against `buf.area.x` and `buf.area.y`.
    #[test]
    fn test_tension_bar_underflow_fixed(
        height in 1..20u16,
        tension in 1.1..=10.0f64,
        inner_y in 0..10u16,
    ) {
        // The buffer simulates the actual terminal screen area.
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 20));
        let widget = TensionBar::new(tension).block(Block::default().borders(Borders::ALL));

        // The area we render to may exceed the buffer bounds or be partially constrained.
        // This must not panic.
        widget.render(Rect::new(0, inner_y, 10, height), &mut buffer);
    }
}
