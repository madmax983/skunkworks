use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;
use tui_shared::Button;

#[test]
fn test_button_hovered_state() {
    let button = Button::new("Hover").hovered(true);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Cyan);
}

#[test]
fn test_button_clicked_state() {
    let button = Button::new("Click").clicked(true);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Red);
}

#[test]
fn test_button_out_of_bounds_empty_area() {
    let button = Button::new("OOB");
    let area = Rect::new(0, 0, 0, 0); // width 0, height 0
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    button.render(area, &mut buffer);
    // Shouldn't panic, and shouldn't render anything
}

#[test]
fn test_button_inner_area_zero() {
    let button = Button::new("ZeroInner");
    let area = Rect::new(0, 0, 2, 2); // border takes 1 px each side -> inner is 0 width, 0 height
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    // Shouldn't panic.
}

#[test]
fn test_button_text_area_outside_bottom() {
    let button = Button::new("OutsideBottom");
    let area = Rect::new(0, 0, 10, 5);
    // give buffer a smaller area than the rect, meaning text_area.y (inner + 1) might be below safe_area bottom
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 2));
    button.render(area, &mut buffer);
}

#[test]
fn test_button_loading_state_coverage() {
    let button = Button::new("Load").loading(true);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Yellow);
}

#[test]
fn test_button_success_state_coverage() {
    let button = Button::new("Win").success(true);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Green);
}

#[test]
fn test_button_style_method() {
    let style = Style::default().bg(Color::Magenta);
    let button = Button::new("Style").style(style);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Magenta);
}
