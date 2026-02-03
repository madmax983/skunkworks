use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};
use std::io;
use tui_shared::Tui;

fn main() -> io::Result<()> {
    // 1. Initialize the terminal
    // This enables raw mode, enters alternate screen, and captures mouse.
    let mut tui = Tui::init()?;

    // 2. Draw something to the terminal
    tui.terminal.draw(|f| {
        let size = f.area();
        let block = Block::default().title(" My TUI App ").borders(Borders::ALL);
        let p = Paragraph::new("Hello, World!")
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(p, size);
    })?;

    // 3. Application logic here...
    // std::thread::sleep(std::time::Duration::from_secs(3));

    // 4. Cleanup is automatic!
    // When `tui` goes out of scope, it automatically:
    // - Disables raw mode
    // - Leaves alternate screen
    // - Shows cursor
    Ok(())
}
