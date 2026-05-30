use ratatui::style::Color;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_shared::Button;

#[test]
fn test_button_active() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
    let active_btn = Button::new("Test").active(true);
    active_btn.render(Rect::new(0, 0, 10, 10), &mut buf);
    assert_eq!(buf[(0, 0)].bg, Color::Yellow);

    let mut buf2 = Buffer::empty(Rect::new(0, 0, 10, 10));
    let inactive_btn = Button::new("Test").active(false);
    inactive_btn.render(Rect::new(0, 0, 10, 10), &mut buf2);
    assert_eq!(buf2[(0, 0)].bg, Color::Reset);
}

#[test]
fn test_button_states() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));

    // Testing specific states hitting branches
    let success = Button::new("S").success(true);
    success.render(Rect::new(0, 0, 10, 10), &mut buf);
    assert_eq!(buf[(0, 0)].bg, Color::Green);

    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
    let loading = Button::new("L").loading(true);
    loading.render(Rect::new(0, 0, 10, 10), &mut buf);
    assert_eq!(buf[(0, 0)].bg, Color::Yellow);

    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
    let clicked = Button::new("C").clicked(true);
    clicked.render(Rect::new(0, 0, 10, 10), &mut buf);
    assert_eq!(buf[(0, 0)].bg, Color::Red);

    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
    let hovered = Button::new("H").hovered(true);
    hovered.render(Rect::new(0, 0, 10, 10), &mut buf);
    assert_eq!(buf[(0, 0)].bg, Color::Cyan);
}

#[test]
fn test_button_out_of_bounds() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));
    let btn = Button::new("Test").block(ratatui::widgets::Block::default());
    // Provide a rendering area totally outside the buffer
    btn.render(Rect::new(10, 10, 10, 10), &mut buf);
}

#[test]
fn test_button_no_block_default_style() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    let widget = Button::new("Click Me");
    widget.render(Rect::new(0, 0, 10, 10), &mut buffer);
}

#[test]
fn test_button_active_false() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    let widget = Button::new("Click Me").active(false);
    widget.render(Rect::new(0, 0, 10, 10), &mut buffer);
}
