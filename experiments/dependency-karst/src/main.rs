use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use dependency_karst::graph::load_graph;
use dependency_karst::layout::Layout;
use dependency_karst::simulation::{VoxelGrid, GRID_SIZE};
use dependency_karst::tui::{CaveWidget, TuiState};
use tui_shared::Tui;

fn main() -> Result<()> {
    // Setup Terminal
    let mut tui = Tui::init()?;

    // Load Data
    let graph = load_graph()?;
    let mut layout = Layout::new(&graph);
    // Optimize layout a bit more
    layout.optimize(&graph, 200);

    // Init Simulation
    let mut grid = VoxelGrid::new();
    let mut state = TuiState::new();

    // App Loop
    let mut running = true;
    while running {
        tui.terminal.draw(|f| {
            let size = f.area();
            let widget = CaveWidget::new(&grid, &state, &layout.positions, &graph);
            f.render_widget(widget, size);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => running = false,
                        KeyCode::Char('w') | KeyCode::Up => {
                            if state.cursor_y > 0 {
                                state.cursor_y -= 1;
                            }
                        }
                        KeyCode::Char('s') | KeyCode::Down => {
                            if state.cursor_y < GRID_SIZE - 1 {
                                state.cursor_y += 1;
                            }
                        }
                        KeyCode::Char('a') | KeyCode::Left => {
                            if state.cursor_x > 0 {
                                state.cursor_x -= 1;
                            }
                        }
                        KeyCode::Char('d') | KeyCode::Right => {
                            if state.cursor_x < GRID_SIZE - 1 {
                                state.cursor_x += 1;
                            }
                        }
                        KeyCode::Char('e') | KeyCode::PageUp => {
                            if state.slice_z < GRID_SIZE - 1 {
                                state.slice_z += 1;
                            }
                        }
                        KeyCode::Char('q') | KeyCode::PageDown => {
                            if state.slice_z > 0 {
                                state.slice_z -= 1;
                            }
                        }
                        KeyCode::Char('r') | KeyCode::Enter => {
                            // Run erosion step
                            grid.erode(&graph, &layout.positions);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
