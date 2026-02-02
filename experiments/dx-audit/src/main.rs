use tui_shared::Tui;
use std::io;
use ratatui::{widgets::{Block, Borders, Paragraph}, layout::Alignment};

fn main() -> io::Result<()> {
    // Initialize the terminal
    let mut tui = Tui::init()?;

    // Draw something to the terminal
    tui.terminal.draw(|f| {
        let size = f.area();
        let block = Block::default()
            .title(" DX Audit ")
            .borders(Borders::ALL);
        let p = Paragraph::new("If you can see this, tui-shared works!\n\n(Sleeping for 3 seconds...)")
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(p, size);
    })?;

    // Sleep so we can see the output before it closes
    std::thread::sleep(std::time::Duration::from_secs(3));

    // The terminal is automatically restored when `tui` goes out of scope
    Ok(())
}
