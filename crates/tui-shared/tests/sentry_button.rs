use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};
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
    // Buffer should remain empty as it was an out-of-bounds zero area render
    assert_eq!(buffer[(0, 0)].symbol(), " ");
}

#[test]
fn test_button_inner_area_zero() {
    let button = Button::new("ZeroInner");
    // border takes 1 px each side -> inner is 0 width, 0 height
    let area = Rect::new(0, 0, 2, 2);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);

    // We expect the border to be rendered at the corners
    // Top left border character is typically '┌'
    assert_eq!(buffer[(0, 0)].symbol(), "┌");
    // However, text shouldn't be rendered because inner area is zero
    assert_ne!(buffer[(0, 1)].symbol(), "Z");
}

#[test]
fn test_button_text_area_outside_bottom() {
    let button = Button::new("OutsideBottom");
    let area = Rect::new(0, 0, 10, 5);
    // give buffer a smaller area than the rect, meaning text_area.y (inner + 1) might be below safe_area bottom
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 2));
    button.render(area, &mut buffer);

    // The top border should render
    assert_eq!(buffer[(0, 0)].symbol(), "┌");
    // The text would be at y=2 or 3, but buffer is only height 2.
    // The test ensures `if text_area.y < buf.area.bottom()` boundary check protects us
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

#[test]
fn test_button_icon_method() {
    let button = Button::new("Save").icon("💾");
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    // Space is 1, Save is 4. Total text length depends on exact icon width logic.
    // We expect the first character of the inner area to be the icon, roughly at (7, 1) or similar.
    let mut found_icon = false;
    for x in 0..20 {
        if buffer[(x, 1)].symbol() == "💾" {
            found_icon = true;
            break;
        }
    }
    assert!(found_icon, "Icon was not rendered on the button");
}

#[test]
fn test_button_block_method() {
    let block = Block::default().borders(Borders::NONE);
    let button = Button::new("Block").block(block);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);

    // With no borders, inner area matches full area, so text should start around (7, 1)
    // There shouldn't be a border character at (0,0)
    assert_ne!(buffer[(0, 0)].symbol(), "┌");
}

#[test]
fn test_button_active_method_true() {
    let button = Button::new("Active").active(true);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    assert_eq!(buffer[(0, 0)].bg, Color::Yellow);
    assert_eq!(buffer[(0, 0)].fg, Color::Black);
}

#[test]
fn test_button_active_method_false() {
    let button = Button::new("Inactive").active(false);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    assert_eq!(buffer[(0, 0)].fg, Color::Gray);
    assert_eq!(buffer[(0, 0)].bg, Color::Reset);
}

#[test]
fn test_button_hovered_false() {
    let button = Button::new("Hover").hovered(false);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    assert_ne!(buffer[(0,0)].bg, Color::Cyan);
}

#[test]
fn test_button_clicked_false() {
    let button = Button::new("Click").clicked(false);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    assert_ne!(buffer[(0,0)].bg, Color::Red);
}

#[test]
fn test_button_loading_false() {
    let button = Button::new("Load").loading(false);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    assert_ne!(buffer[(0,0)].bg, Color::Yellow);
}

#[test]
fn test_button_success_false() {
    let button = Button::new("Win").success(false);
    let area = Rect::new(0, 0, 20, 3);
    let mut buffer = Buffer::empty(area);
    button.render(area, &mut buffer);
    assert_ne!(buffer[(0,0)].bg, Color::Green);
}
