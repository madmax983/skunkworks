pub mod fs;
pub mod layout;

use crossterm::event::{self, Event, KeyCode};
use num_complex::Complex;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use layout::LayoutNode;
use poincare_disk::{mobius_sub, Point};

struct App {
    layout_root: LayoutNode,
    view_center: Point,
    target_center: Point,

    // Navigation
    // The path to the node currently "centered" in view
    path_stack: Vec<usize>,
    // The index of the child currently selected (highlighted)
    selected_child_idx: usize,

    last_tick: Instant,
}

impl App {
    fn new() -> anyhow::Result<Self> {
        let root_path = std::env::current_dir()?;
        let root_fs = fs::scan_depth(&root_path, 4)?;
        let layout = layout::build_layout(root_fs);

        Ok(Self {
            layout_root: layout,
            view_center: Complex::new(0.0, 0.0),
            target_center: Complex::new(0.0, 0.0),
            path_stack: Vec::new(),
            selected_child_idx: 0,
            last_tick: Instant::now(),
        })
    }

    fn get_node_at_path(&self, path: &[usize]) -> &LayoutNode {
        let mut node = &self.layout_root;
        for &idx in path {
            if idx < node.children.len() {
                node = &node.children[idx];
            } else {
                break;
            }
        }
        node
    }

    fn current_focused_node(&self) -> &LayoutNode {
        self.get_node_at_path(&self.path_stack)
    }

    fn update_target(&mut self) {
        let (pos, child_count) = {
            let node = self.current_focused_node();
            (node.global_pos, node.children.len())
        };

        self.target_center = pos;

        // Clamp selection
        if child_count > 0 {
            if self.selected_child_idx >= child_count {
                self.selected_child_idx = child_count - 1;
            }
        } else {
            self.selected_child_idx = 0;
        }
    }

    fn on_tick(&mut self) {
        // Smooth interpolation
        let diff = self.target_center - self.view_center;
        if diff.norm() > 0.001 {
            self.view_center += diff * 0.1;
        } else {
            self.view_center = self.target_center;
        }
    }

    fn move_selection(&mut self, delta: i32) {
        let node = self.current_focused_node();
        let count = node.children.len();
        if count == 0 {
            return;
        }
        let new_idx = (self.selected_child_idx as i32 + delta).rem_euclid(count as i32);
        self.selected_child_idx = new_idx as usize;
    }

    fn enter_child(&mut self) {
        let node = self.current_focused_node();
        if !node.children.is_empty() {
            self.path_stack.push(self.selected_child_idx);
            self.selected_child_idx = 0;
            self.update_target();
        }
    }

    fn go_up(&mut self) {
        if !self.path_stack.is_empty() {
            // Restore selection of the child we just exited
            let old_idx = self.path_stack.pop().unwrap();
            self.selected_child_idx = old_idx;
            self.update_target();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left | KeyCode::Char('a') => app.move_selection(-1),
                    KeyCode::Right | KeyCode::Char('d') => app.move_selection(1),
                    KeyCode::Enter | KeyCode::Down | KeyCode::Char('s') => app.enter_child(),
                    KeyCode::Backspace | KeyCode::Up | KeyCode::Char('w') | KeyCode::Esc => {
                        app.go_up()
                    }
                    _ => {}
                }
            }
        }

        if app.last_tick.elapsed() >= Duration::from_millis(16) {
            app.on_tick();
            app.last_tick = Instant::now();
        }
    }

    tui.exit()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    let current_node = app.current_focused_node();
    let path_str = current_node.fs.path.display().to_string();

    // Draw Info
    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Path: ", Style::default().fg(Color::Cyan)),
            Span::raw(path_str),
        ]),
        Line::from(vec![
            Span::raw("Controls: Arrow Keys / WASD to move, Enter to zoom in, Backspace to zoom out, 'q' to quit."),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Hyperbolic File System"));

    f.render_widget(info, info_area);

    // Draw Canvas
    // We map the unit disk (-1..1) to the canvas.
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Poincaré Disk"),
        )
        .x_bounds([-1.1, 1.1])
        .y_bounds([-1.1, 1.1])
        .paint(|ctx| {
            // Draw boundary
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                color: Color::White,
            });

            // Recursive draw
            draw_node(
                ctx,
                &app.layout_root,
                app.view_center,
                &app.path_stack,
                &[],
                app.selected_child_idx,
            );
        });

    f.render_widget(canvas, canvas_area);
}

fn draw_node(
    ctx: &mut ratatui::widgets::canvas::Context,
    node: &LayoutNode,
    view_center: Point,
    target_path: &[usize],  // The path to the currently focused node
    current_path: &[usize], // The path to *this* node
    selected_child_idx: usize,
) {
    // Transform position
    let screen_pos = mobius_sub(node.global_pos, view_center);

    // Visibility cull
    if screen_pos.norm_sqr() > 1.0 {
        return;
    }

    // Determine if this node is the focused one, or a child of focused, etc.
    let is_focused = target_path == current_path;
    let is_ancestor =
        target_path.starts_with(current_path) && target_path.len() > current_path.len();

    // Color logic
    let color = if is_focused {
        Color::Yellow
    } else if is_ancestor {
        Color::Blue
    } else if node.fs.is_dir {
        Color::Green
    } else {
        Color::Gray
    };

    // Highlight selected child
    // If this node is a child of the focused node, and its index matches selection
    let is_selected_child = if let Some((last, parent_path)) = current_path.split_last() {
        parent_path == target_path && *last == selected_child_idx
    } else {
        false
    };

    let draw_color = if is_selected_child { Color::Red } else { color };

    // Draw Point
    let radius = if is_focused { 0.05 } else { 0.02 };
    ctx.draw(&Circle {
        x: screen_pos.re,
        y: screen_pos.im,
        radius,
        color: draw_color,
    });

    // Draw Label if close to center
    if screen_pos.norm() < 0.8 {
        let label = node.fs.name();
        ctx.print(
            screen_pos.re,
            screen_pos.im + radius + 0.02,
            Span::styled(label, Style::default().fg(draw_color)),
        );
    }

    // Draw Lines to children
    for (i, child) in node.children.iter().enumerate() {
        let child_screen_pos = mobius_sub(child.global_pos, view_center);

        ctx.draw(&CanvasLine {
            x1: screen_pos.re,
            y1: screen_pos.im,
            x2: child_screen_pos.re,
            y2: child_screen_pos.im,
            color: Color::DarkGray,
        });

        // Recurse
        // Construct new path
        let mut new_path = Vec::from(current_path);
        new_path.push(i);
        draw_node(
            ctx,
            child,
            view_center,
            target_path,
            &new_path,
            selected_child_idx,
        );
    }
}
