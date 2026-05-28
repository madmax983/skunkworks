use ratatui::style::Color;
use tui_shared::{Action, Entity, LogList, Region};

#[test]
fn test_entity_coverage() {
    let e = Entity::new("player")
        .with_id("player_1")
        .display("@")
        .moving(1.0, 0.0)
        .with_prop("hp", 100);

    assert_eq!(e.kind, "player");
    assert_eq!(e.id.unwrap(), "player_1");
    assert_eq!(e.display.unwrap(), "@");
    assert_eq!(e.props.len(), 1);
}

#[test]
fn test_action_coverage() {
    let a = Action::new(String::from("attack"))
        .describe("Attack the enemy")
        .key("Enter");

    assert_eq!(a.name, "attack");
    assert_eq!(a.description.unwrap(), "Attack the enemy");
    assert_eq!(a.key.unwrap(), "Enter");
}

#[test]
fn test_region_coverage() {
    let r = Region::new(String::from("test"), 10, 10, 20, 20).describe("A test region");

    assert_eq!(r.name, "test");
    assert_eq!(r.description.unwrap(), "A test region");
}

#[test]
fn test_log_list_coverage() {
    let items = vec![String::from("log 1"), String::from("error 2")];
    let list = LogList::new(items.clone()).with_title("Test Logs");

    use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
    list.render(Rect::new(0, 0, 10, 10), &mut buf);

    assert_eq!(buf[(0, 0)].fg, Color::Reset);
}
