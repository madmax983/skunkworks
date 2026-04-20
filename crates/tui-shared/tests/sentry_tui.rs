use tui_shared::Tui;

#[test]
#[ignore = "TUI-101: Modifies global terminal state"]
fn test_tui_init_and_exit() {
    let mut tui = Tui::init().expect("Failed to init TUI");
    tui.exit().expect("Failed to exit TUI");
}
