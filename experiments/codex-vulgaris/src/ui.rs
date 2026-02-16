use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Left Pane: Source
    let source_block = Block::default()
        .title(" Proto-Code (Original) ")
        .borders(Borders::ALL);
    let source_text = Paragraph::new(app.source_code.as_str())
        .block(source_block)
        .wrap(Wrap { trim: false });
    f.render_widget(source_text, main_chunks[0]);

    // Right Pane: Split into Code and Glossary
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(main_chunks[1]);

    // Right Top: Evolved Code
    let evolved_block = Block::default()
        .title(format!(" Vulgar Code (Era {}) ", app.era))
        .borders(Borders::ALL);
    let evolved_text = Paragraph::new(app.evolved_code.as_str())
        .block(evolved_block)
        .wrap(Wrap { trim: false });
    f.render_widget(evolved_text, right_chunks[0]);

    // Right Bottom: Glossary
    let rows: Vec<Row> = app
        .evolved_map
        .iter()
        .map(|(orig, new)| Row::new(vec![orig.clone(), new.clone()]))
        .collect();

    let glossary = Table::new(
        rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .header(Row::new(vec!["Proto-Form", "Vulgar Form"]).style(Style::default().fg(Color::Yellow)))
    .block(
        Block::default()
            .title(" Etymological Dictionary ")
            .borders(Borders::ALL),
    );

    f.render_widget(glossary, right_chunks[1]);

    // Bottom Bar: Controls
    let help_text = "Left/Right: Change Era | ?: Help | q: Quit";
    let help_paragraph = Paragraph::new(help_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(help_paragraph, chunks[1]);

    if app.show_help {
        let area = centered_rect(60, 50, f.area());
        f.render_widget(Clear, area); // Clear underlying content
        let popup_block = Block::default().title(" Help ").borders(Borders::ALL);
        let text = "Controls:\n\
                    Left/Right: Travel through Time (Eras)\n\
                    q: Quit\n\
                    ?: Toggle Help\n\n\
                    Linguistic Laws Applied:\n\
                    1. Grimm's Law (P->F, T->Th, K->H...)\n\
                    2. Great Vowel Shift (A->E, E->I...)\n\
                    3. Lenition (Intervocalic voicing)\n\
                    4. Assimilation (N->M before P/B...)";
        let p = Paragraph::new(text)
            .block(popup_block)
            .wrap(Wrap { trim: true });
        f.render_widget(p, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
