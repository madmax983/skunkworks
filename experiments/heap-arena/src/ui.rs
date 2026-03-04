use crate::game::Game;
use crate::level_gen::BlockType;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(6)]) // Game area vs HUD
        .split(f.area());

    let game_area = chunks[0];
    let hud_area = chunks[1];

    // Calculate visible range based on scroll_offset
    let start_x = app.scroll_offset;
    let end_x = start_x + game_area.width as f64;

    // --- Draw Heap (Ground) ---
    // Ground Y position: Let's put ground at bottom of game area minus padding
    let ground_y = game_area.y + game_area.height - 2;

    // Iterate blocks and draw those in view
    for block in &app.heap.blocks {
        let b_start = block.start as f64;
        let b_end = (block.start + block.size) as f64;

        // Skip if out of view
        if b_end < start_x || b_start > end_x {
            continue;
        }

        // Clip to view
        let draw_start = b_start.max(start_x);
        let draw_end = b_end.min(end_x);
        let width = (draw_end - draw_start).max(0.0) as u16;

        if width == 0 {
            continue;
        }

        let screen_x = (game_area.x as f64 + (draw_start - start_x)) as u16;

        let style = match block.block_type {
            BlockType::Solid => Style::default().bg(Color::Green),
            BlockType::Hazard => Style::default().bg(Color::Red).fg(Color::Yellow),
            BlockType::Bouncy => Style::default().bg(Color::Blue),
            BlockType::Gap => Style::default(), // Invisible
        };

        if block.is_solid {
            f.render_widget(
                Block::default().style(style),
                Rect::new(screen_x, ground_y, width, 1),
            );

            // Draw code snippet below/inside?
            // If hazard, show warnings
            if matches!(block.block_type, BlockType::Hazard) {
                f.render_widget(
                    Paragraph::new("!").style(Style::default().fg(Color::Yellow)),
                    Rect::new(screen_x, ground_y, width.min(1), 1),
                );
            }
        }
    }

    // --- Draw Player ---
    if !app.player.is_dead {
        let p_screen_x = (game_area.x as f64 + (app.player.x - start_x)) as u16;
        let p_screen_y = (ground_y as f64 - app.player.y) as u16;

        // Ensure within bounds
        if p_screen_x >= game_area.x
            && p_screen_x < (game_area.x + game_area.width)
            && p_screen_y >= game_area.y
            && p_screen_y < (game_area.y + game_area.height)
        {
            f.render_widget(
                Paragraph::new("P").style(Style::default().fg(Color::Cyan).bold()),
                Rect::new(p_screen_x, p_screen_y, 1, 1),
            );
        }
    }

    // --- Draw Boss ---
    let b_screen_x = (game_area.x as f64 + (app.boss.x - start_x)) as u16;
    let b_screen_y = (ground_y as f64 - app.boss.y) as u16;

    if b_screen_x >= game_area.x
        && b_screen_x < (game_area.x + game_area.width)
        && b_screen_y >= game_area.y
        && b_screen_y < (game_area.y + game_area.height)
    {
        let boss_char = if app.boss.cooldown < 10 { 'O' } else { '@' };
        f.render_widget(
            Paragraph::new(boss_char.to_string()).style(Style::default().fg(Color::Magenta).bold()),
            Rect::new(b_screen_x, b_screen_y, 1, 1),
        );

        // Boss Name floating above
        let name_len = app.boss.stats.name.len() as u16;
        let name_x = b_screen_x.saturating_sub(name_len / 2);
        if b_screen_y > 1 {
            f.render_widget(
                Paragraph::new(app.boss.stats.name.clone())
                    .style(Style::default().fg(Color::Magenta)),
                Rect::new(name_x, b_screen_y - 1, name_len, 1),
            );
        }
    }

    // --- Draw Projectiles ---
    for proj in &app.projectiles {
        let pj_screen_x = (game_area.x as f64 + (proj.x - start_x)) as u16;
        let pj_screen_y = (ground_y as f64 - proj.y) as u16;

        if pj_screen_x >= game_area.x
            && pj_screen_x < (game_area.x + game_area.width)
            && pj_screen_y >= game_area.y
            && pj_screen_y < (game_area.y + game_area.height)
        {
            f.render_widget(
                Paragraph::new(proj.symbol.to_string()).style(Style::default().fg(Color::Red)),
                Rect::new(pj_screen_x, pj_screen_y, 1, 1),
            );
        }
    }

    // --- Draw HUD ---
    let hp_color = if app.player.hp < 30 {
        Color::Red
    } else {
        Color::Green
    };

    let stats_line = Line::from(vec![
        Span::raw("HP: "),
        Span::styled(
            format!("{} / {}", app.player.hp, app.player.max_hp),
            Style::default().fg(hp_color),
        ),
        Span::raw(" | Score: "),
        Span::styled(format!("{}", app.score), Style::default().fg(Color::Yellow)),
        Span::raw(" | Boss: "),
        Span::styled(
            format!(
                "{} (ATK: {}, DEF: {})",
                app.boss.stats.name, app.boss.stats.attack, app.boss.stats.defense
            ),
            Style::default().fg(Color::Magenta),
        ),
    ]);

    // Get current code line based on player position
    let mut current_code = " Void ".to_string();
    let cx = app.player.x as usize;
    for block in &app.heap.blocks {
        if cx >= block.start && cx < (block.start + block.size) {
            current_code = block.code.clone();
            break;
        }
    }

    let code_line = Line::from(vec![
        Span::raw("Current Instruction: "),
        Span::styled(
            current_code,
            Style::default().fg(Color::White).bg(Color::DarkGray),
        ),
    ]);

    // Messages (Logs)
    let logs: Vec<Span> = app
        .messages
        .iter()
        .rev()
        .take(3)
        .map(|s| Span::raw(format!(" > {}", s)))
        .collect();
    let log_line = Line::from(logs).style(Style::default().dim());

    let controls = Line::from(" [WASD/Arrows] Move/Jump | [q] Quit").style(Style::default().dim());

    let hud_text = vec![stats_line, code_line, log_line, controls];

    f.render_widget(
        Paragraph::new(hud_text).block(
            Block::default()
                .borders(Borders::TOP)
                .title(" Heap Arena 🏟️ "),
        ),
        hud_area,
    );

    // Win/Loss Overlay
    if app.game_over {
        let msg = if app.player.is_dead {
            "SEGMENTATION FAULT"
        } else {
            "GAME OVER"
        };
        let color = Color::Red;
        draw_centered_msg(f, game_area, msg, color);
    } else if app.win {
        draw_centered_msg(f, game_area, "MEMORY RECLAIMED!", Color::Green);
    }
}

fn draw_centered_msg(f: &mut Frame, area: Rect, msg: &str, color: Color) {
    let text = Paragraph::new(msg)
        .style(Style::default().fg(color).bold())
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let area = centered_rect(60, 20, area);
    f.render_widget(text, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
