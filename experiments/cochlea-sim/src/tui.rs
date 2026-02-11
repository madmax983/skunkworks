use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{
        canvas::{Canvas, Points},
        Axis, Block, Borders, Chart, Dataset, Sparkline,
    },
    Frame,
};

pub fn draw(
    f: &mut Frame,
    waveform: &[f32],
    spikes: &[(f64, usize)],
    potentials: &[f32],
    channels: usize,
    window_start: f64,
    window_end: f64,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // Waveform
            Constraint::Percentage(50), // Spike Raster
            Constraint::Percentage(30), // Spectrum (Potentials)
        ])
        .split(f.area());

    draw_waveform(f, chunks[0], waveform, window_start, window_end);
    draw_raster(f, chunks[1], spikes, channels, window_start, window_end);
    draw_spectrum(f, chunks[2], potentials);
}

fn draw_waveform(f: &mut Frame, area: Rect, data: &[f32], start: f64, end: f64) {
    let width = area.width as usize;
    if width == 0 || data.is_empty() { return; }

    // Simple downsampling
    let step = (data.len() / width).max(1);
    let dt = (end - start) / data.len() as f64;

    let points: Vec<(f64, f64)> = data.iter()
        .step_by(step)
        .enumerate()
        .map(|(i, &v)| {
            let index = i * step;
            let t = start + index as f64 * dt;
            (t, v as f64)
        })
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("Audio")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&points),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Waveform").borders(Borders::ALL))
        .x_axis(Axis::default().bounds([start, end])) // Using absolute time
        .y_axis(Axis::default().bounds([-1.0, 1.0]));

    f.render_widget(chart, area);
}

fn draw_raster(
    f: &mut Frame,
    area: Rect,
    spikes: &[(f64, usize)],
    channels: usize,
    start: f64,
    end: f64
) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Neurogram (Spike Raster)"))
        .x_bounds([start, end])
        .y_bounds([0.0, channels as f64])
        .paint(|ctx| {
             let points: Vec<(f64, f64)> = spikes.iter()
                .map(|&(t, ch)| (t, ch as f64))
                .collect();

             ctx.draw(&Points {
                 coords: &points,
                 color: Color::Green,
             });
        });

    f.render_widget(canvas, area);
}

fn draw_spectrum(f: &mut Frame, area: Rect, data: &[f32]) {
    let data_u64: Vec<u64> = data.iter().map(|&v| (v * 100.0) as u64).collect();

    let sparkline = Sparkline::default()
        .block(Block::default().title("Cochlear Activity").borders(Borders::ALL))
        .data(&data_u64)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(sparkline, area);
}
