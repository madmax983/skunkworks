mod audio;
use audio::{AudioWriter, AudioEvent, Waveform};
use chimera_lang::{vm::{ChimeraVM, Value}, compiler};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph},
    style::{Color, Style},
    Terminal,
};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, mpsc, atomic::{AtomicBool, Ordering}};

fn main() -> anyhow::Result<()> {
    // 1. Audio Thread setup
    let (audio_tx, audio_rx) = mpsc::channel();
    let running = Arc::new(AtomicBool::new(true));
    let audio_running = running.clone();

    let audio_thread = thread::spawn(move || {
        // Create the writer. Note: In a real app we'd handle errors better.
        let mut writer = AudioWriter::new("chimera_groove_session.wav", 44100, audio_rx).expect("Failed to create audio writer");
        let chunk_duration = 0.05; // 50ms chunks
        while audio_running.load(Ordering::Relaxed) {
            if let Err(_) = writer.process_events() { break; }
            if let Err(_) = writer.generate_chunk(chunk_duration) { break; }
            thread::sleep(Duration::from_secs_f32(chunk_duration));
        }
        // Writer dropped here, finalizing WAV header.
    });

    // 2. TUI Setup
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. VM Setup
    // A script that reads 0,0 (Phase). If > 90 (Beat), it performs actions.
    // Logic:
    // 1. Read Grid(0,0) -> Stack
    // 2. Push 90
    // 3. Gt (>)
    // 4. Brz (If 0/False, jump over action)
    // 5. Action: Radiate (Visual), Photosynthesize (Energy)
    let code = r#"
    (
        0 0 g_read
        90 >
        (
            "ON BEAT!" print
            2 2 4 100 radiate
            photosynthesize
        )
        (
            consume
        )
        brz_s
    ) L
    "#;

    let dna = compiler::compile(code, None)?;
    let mut vm = ChimeraVM::new(dna);

    // Config
    let bpm = 120.0;
    let beat_duration = 60.0 / bpm; // 0.5s
    let start_time = Instant::now();
    let mut last_beat = 0;

    // 4. Main Loop
    loop {
        // Time & Rhythm
        let now = Instant::now();
        let elapsed = now.duration_since(start_time).as_secs_f32();
        let beat_count = (elapsed / beat_duration) as u64;
        let phase = (elapsed % beat_duration) / beat_duration;
        let phase_int = (phase * 100.0) as i64;

        // Inject Phase into Grid[0][0]
        // This is the "Phase-Locked" aspect. The environment pulses with time.
        vm.grid[0][0] = Value::Int(phase_int);

        // Beat Event (Audio/Visual Sync)
        if beat_count > last_beat {
            last_beat = beat_count;
            // Send Audio Kick
            let _ = audio_tx.send(AudioEvent {
                waveform: Waveform::Square,
                frequency: 100.0,
                duration: 0.1,
                volume: 0.5,
                start_time: 0.0,
            });
            // Visual Flash (Optional, maybe clear output log?)
        }

        // Step VM
        // We run multiple steps per frame to make it faster
        for _ in 0..5 {
            vm.step();
        }

        // Draw
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Groove Bar
                    Constraint::Min(0),    // Grid
                    Constraint::Length(12), // Output
                ].as_ref())
                .split(f.area());

            // Groove Bar
            let label = format!("BPM: {} | Phase: {:.2}%", bpm, phase * 100.0);
            let color = if phase > 0.9 { Color::Red } else { Color::Green };
            let gauge = Gauge::default()
                .block(Block::default().title("Rhythm Engine").borders(Borders::ALL))
                .gauge_style(Style::default().fg(color))
                .ratio(phase as f64)
                .label(label);
            f.render_widget(gauge, chunks[0]);

            // Grid Rendering
            let mut grid_str = String::new();
            for row in &vm.grid {
                for cell in row {
                    let s = match cell {
                        Value::Int(0) => " . ".to_string(),
                        Value::Int(n) => format!("{:3}", n % 999),
                        Value::Str(_) => " S ".to_string(),
                        _ => " ? ".to_string(),
                    };
                    grid_str.push_str(&s);
                }
                grid_str.push('\n');
            }
            let grid_block = Paragraph::new(grid_str)
                .block(Block::default().title("Chimera Grid (Phase at 0,0)").borders(Borders::ALL));
            f.render_widget(grid_block, chunks[1]);

            // Output Log
            // We join the output lines and take the last few
            let output_text = vm.output.join("\n");
            let lines: Vec<&str> = output_text.lines().collect();
            let start_idx = lines.len().saturating_sub(10);
            let final_text = lines.iter().skip(start_idx).cloned().collect::<Vec<&str>>().join("\n");

            let log = Paragraph::new(final_text)
                .block(Block::default().title("VM Output").borders(Borders::ALL));
            f.render_widget(log, chunks[2]);

        })?;

        // Input Handling
        if event::poll(Duration::from_millis(16))? { // ~60 FPS cap
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Cleanup
    running.store(false, Ordering::Relaxed);
    let _ = audio_thread.join();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("Simulation finished. Audio saved to chimera_groove_session.wav");
    Ok(())
}
