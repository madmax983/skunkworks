use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Chart, Dataset, Row, Table, Axis, GraphType,
    },
    Frame,
};

use crate::simulation::Simulation;
use market_sim::Particle;

pub fn draw_ui(f: &mut Frame, sim: &Simulation) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[0]);

    draw_market(f, sim, left_chunks[0]);
    draw_price_chart(f, sim, left_chunks[1]);
    draw_process_table(f, sim, chunks[1]);
}

fn draw_market(f: &mut Frame, sim: &Simulation, area: Rect) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Market Grid (Asks: Top, Bids: Bottom)"))
        .x_bounds([0.0, sim.market.width as f64])
        .y_bounds([0.0, sim.market.height as f64])
        .paint(|ctx| {
            for y in 0..sim.market.height {
                for x in 0..sim.market.width {
                    match sim.market.get(x, y) {
                        Particle::Bid(_) => {
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: (sim.market.height - 1 - y) as f64, // Invert Y for visual
                                width: 1.0,
                                height: 1.0,
                                color: Color::Green,
                            });
                        }
                        Particle::Ask(_) => {
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: (sim.market.height - 1 - y) as f64,
                                width: 1.0,
                                height: 1.0,
                                color: Color::Red,
                            });
                        }
                        Particle::Trade { age } => {
                             let val = (age as u8).saturating_mul(50);
                             ctx.draw(&Rectangle {
                                x: x as f64,
                                y: (sim.market.height - 1 - y) as f64,
                                width: 1.0,
                                height: 1.0,
                                color: Color::Rgb(val, val, val),
                            });
                        }
                        _ => {}
                    }
                }
            }
        });
    f.render_widget(canvas, area);
}

fn draw_price_chart(f: &mut Frame, sim: &Simulation, area: Rect) {
    let data: Vec<(f64, f64)> = sim.history.iter().enumerate().map(|(i, &p)| (i as f64, p)).collect();
    let datasets = vec![Dataset::default()
        .name("Price")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&data)];

    let x_max = sim.history.len().max(10) as f64;
    let y_max = sim.market.height as f64;

    let chart = Chart::new(datasets)
        .block(Block::default().title("Price History").borders(Borders::ALL))
        .x_axis(Axis::default().bounds([0.0, x_max]))
        .y_axis(Axis::default().bounds([0.0, y_max]));

    f.render_widget(chart, area);
}

fn draw_process_table(f: &mut Frame, sim: &Simulation, area: Rect) {
    let header = Row::new(vec!["ID", "Name", "Work", "Credits", "Col"]).style(Style::default().fg(Color::Yellow));
    let rows: Vec<Row> = sim.processes.iter().map(|p| {
        Row::new(vec![
            format!("{}", p.id),
            p.name.clone(),
            format!("{}", p.work_remaining),
            format!("{:.1}", p.credits),
            format!("{}", p.column),
        ])
        .style(if p.credits < 0.0 { Style::default().fg(Color::Red) } else { Style::default() })
    }).collect();

    let table = Table::new(rows, [
        Constraint::Length(4),
        Constraint::Length(20),
        Constraint::Length(6),
        Constraint::Length(8),
        Constraint::Length(4),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Processes"));

    f.render_widget(table, area);
}
