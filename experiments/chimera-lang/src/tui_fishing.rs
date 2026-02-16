use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    widgets::canvas::{Canvas, Rectangle},
    Frame,
};
use crate::vm::ChimeraVM;
use crate::tui::AppState;

/// Helper function to draw a gauge with color based on value
fn draw_tension_gauge<'a>(label: &'a str, ratio: f64) -> Gauge<'a> {
    let title = Span::styled(
        label,
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Yellow),
    );
    let tension_color = if ratio > 0.8 {
        Color::Red
    } else if ratio > 0.5 {
        Color::Yellow
    } else {
        Color::Green
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .gauge_style(Style::default().fg(tension_color))
        .ratio(ratio.clamp(0.0, 1.0));

    if ratio > 0.9 {
        gauge.label(format!("CRITICAL TENSION ({:.0}%)", ratio * 100.0))
            .use_unicode(true)
            .style(Style::default().fg(Color::Red).bg(Color::Black).add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK))
    } else {
        gauge.label(format!("{} ({:.0}%)", label, ratio * 100.0))
    }
}

pub(crate) fn render_fishing(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(3), // Tension Bar
                Constraint::Length(3), // Info
            ]
            .as_ref(),
        )
        .split(f.area());

    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Fishing Minigame");

    // Flash background if tension is critical
    if app_state.fishing_tension > 0.9 && vm.tick_counter % 4 < 2 {
        block = block.style(Style::default().bg(Color::Red));
    }

    let canvas = Canvas::default()
        .block(block)
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Sky Gradient (Simulated via layered rectangles)
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 50.0,
                width: 100.0,
                height: 50.0,
                color: Color::Cyan,
            });
            // Top Sky (Darker Blue)
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 75.0,
                width: 100.0,
                height: 25.0,
                color: Color::Blue,
            });

            // Water Gradient
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 50.0,
                color: Color::Blue,
            });
            // Deep Water (Dark Gray)
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 25.0,
                color: Color::DarkGray,
            });

            // Water Ripples Animation
            for i in 0..20 {
                let speed = vm.tick_counter as f64 * 0.2;
                let rx = (speed + i as f64 * 13.0) % 95.0 + 2.0;
                let ry = 5.0 + (i as f64 * 7.0) % 40.0;
                let ch = if i % 2 == 0 { "~" } else { "-" };
                ctx.print(rx, ry, ch);
            }

            if app_state.fishing_cast {
                // Splash / Ripple around bobber
                if app_state.fishing_bobber_y < 50.0 {
                    // Bobber is underwater/surface
                    let phase = (vm.tick_counter % 6) / 2;
                    let (left, right) = match phase {
                        0 => ("(", ")"),
                        1 => ("<", ">"),
                        _ => ("{", "}"),
                    };
                    ctx.print(48.0, app_state.fishing_bobber_y, left);
                    ctx.print(51.0, app_state.fishing_bobber_y, right);
                }

                // Fishing Line
                let line_color = if app_state.fishing_tension > 0.8 {
                    Color::Red
                } else if app_state.fishing_tension > 0.5 {
                    Color::Yellow
                } else {
                    Color::White
                };

                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: 50.0,
                    y1: 100.0, // Top center (approx rod tip)
                    x2: 50.0,
                    y2: app_state.fishing_bobber_y,
                    color: line_color,
                });

                // Bobber (Visual)
                let bobber_icon = if app_state.fishing_hooked { "🔴" } else { "⚪" };
                ctx.print(50.0, app_state.fishing_bobber_y, bobber_icon);

                // Center detail
                ctx.print(
                    49.5,
                    app_state.fishing_bobber_y - 0.5,
                    if app_state.fishing_hooked { "!" } else { "." },
                );

                if app_state.fishing_hooked {
                    // Splash effect
                    ctx.print(48.0, app_state.fishing_bobber_y, "💦");
                    ctx.print(51.0, app_state.fishing_bobber_y, "💦");

                    // Flash text
                    if vm.tick_counter % 10 < 5 {
                         ctx.print(52.0, app_state.fishing_bobber_y + 2.0, "HOOKED!");
                    }
                }

                // Fish (Icon)
                if app_state.fishing_fish_y > 0.0 && app_state.fishing_fish_y < 100.0 {
                    let fish_icon = if app_state.fishing_tension > 0.8 {
                        "🦈"
                    } else if app_state.fishing_tension > 0.5 {
                        "🐟"
                    } else {
                        "🐠"
                    };
                    ctx.print(49.0, app_state.fishing_fish_y, fish_icon);
                }

                // Instructions Overlay (Top Right)
                ctx.print(60.0, 90.0, "SPACE: Reel");
            } else {
                ctx.print(35.0, 90.0, "Press SPACE to Cast");
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Tension Bar
    let tension = app_state.fishing_tension;
    let gauge = draw_tension_gauge("Line Tension", tension);
    f.render_widget(gauge, chunks[1]);

    let info_text = if app_state.fishing_hooked {
        Span::styled("REEL IT IN! WATCH THE TENSION!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else if app_state.fishing_cast {
        Span::styled("Waiting for a bite...", Style::default().fg(Color::Cyan))
    } else {
        Span::styled("Ready to cast.", Style::default().fg(Color::White))
    };

    let info = Paragraph::new(Line::from(vec![
            Span::raw("Status: "),
            info_text,
            Span::raw(" | Controls: Space to Cast/Reel"),
        ]))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[2]);
}
