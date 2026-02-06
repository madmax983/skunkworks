mod opcode;
mod quipu;
mod vm;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};

use opcode::OpCode;
use quipu::{Cord, Quipu};
use vm::VM;

fn main() -> Result<()> {
    // 1. Setup Data (Sample Program)
    let mut quipu = Quipu::new();

    // Cord 0: Main
    // Push 10, Push 20, Add, Print, Call 1, Print, Ret
    let mut c0 = Cord::new(0);
    c0.push_op(OpCode::Push(10));
    c0.push_op(OpCode::Push(20));
    c0.push_op(OpCode::Add);
    c0.push_op(OpCode::Print);
    c0.push_op(OpCode::Call(1));
    c0.push_op(OpCode::Print);
    c0.push_op(OpCode::Ret);
    quipu.add_cord(c0);

    // Cord 1: Subroutine
    // Push 999, Ret
    let mut c1 = Cord::new(1);
    c1.push_op(OpCode::Push(999));
    c1.push_op(OpCode::Ret);
    quipu.add_cord(c1);

    let mut vm = VM::new(quipu);

    // 2. Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. Run Loop
    let res = run_app(&mut terminal, &mut vm);

    // 4. Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, vm: &mut VM) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, vm))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => vm.step(),
                        KeyCode::Char('r') => {
                             // Fast forward a bit
                             for _ in 0..10 {
                                 vm.step();
                             }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, vm: &VM) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(size);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[0]);

    draw_quipu(f, top_chunks[0], vm);
    draw_stack(f, top_chunks[1], vm);
    draw_output(f, chunks[1], vm);
}

fn draw_quipu(f: &mut Frame, area: Rect, vm: &VM) {
    let block = Block::default().title(" Quipu Genome ").borders(Borders::ALL);
    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    // We render cords horizontally distributed?
    // Let's render them as vertical columns.

    let cords_len = vm.quipu.cords.len();
    if cords_len == 0 { return; }

    let cord_width = inner_area.width / cords_len as u16;

    for (i, cord) in vm.quipu.cords.iter().enumerate() {
        let cord_x = inner_area.x + (i as u16 * cord_width);

        // Draw Cord Line
        // Assuming we can draw text.
        // We will just draw the Knots vertically.

        let mut spans = Vec::new();
        spans.push(Line::from(Span::styled(format!("Cord {}", i), Style::default().fg(Color::Cyan))));
        spans.push(Line::from("  |"));

        for (k_idx, knot) in cord.knots.iter().enumerate() {
            let is_active = !vm.halted && vm.pc_cord == i && vm.pc_knot == k_idx;

            let symbol = format!("{}", knot); // Uses Display impl from quipu.rs

            let style = if is_active {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(Color::White)
            };

            // Indent slightly to look like it's hanging
            let prefix = if is_active { "-> " } else { "   " };
            spans.push(Line::from(vec![
                Span::raw("  |"),
            ]));
            spans.push(Line::from(vec![
                Span::raw(prefix),
                Span::styled(symbol, style)
            ]));
        }

        // Render this cord's column
        // We can't easily split rects dynamically in a loop with ratatui nicely without Constraints.
        // So we'll render a Paragraph at the calculated coordinates.

        let cord_area = Rect {
            x: cord_x,
            y: inner_area.y,
            width: cord_width,
            height: inner_area.height,
        };

        let p = Paragraph::new(spans).block(Block::default()); // No borders to avoid clutter
        f.render_widget(p, cord_area);
    }
}

fn draw_stack(f: &mut Frame, area: Rect, vm: &VM) {
    let items: Vec<ListItem> = vm.stack.iter().rev().map(|v| {
        ListItem::new(format!("{}", v))
    }).collect();

    let list = List::new(items)
        .block(Block::default().title(" Stack ").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}

fn draw_output(f: &mut Frame, area: Rect, vm: &VM) {
    // Show last 10 lines
    let start = if vm.output.len() > 10 { vm.output.len() - 10 } else { 0 };
    let lines: Vec<Line> = vm.output[start..].iter().map(|s| Line::from(s.as_str())).collect();

    let p = Paragraph::new(lines)
        .block(Block::default().title(" Output Log ").borders(Borders::ALL));
    f.render_widget(p, area);
}
