use crate::layout::Vec3;
use crate::simulation::{VoxelGrid, Voxel, GRID_SIZE};
use crate::graph::CrateGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use ratatui::widgets::{Widget, Block, Borders};
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Style, Modifier};

pub struct TuiState {
    pub slice_z: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl TuiState {
    pub fn new() -> Self {
        Self {
            slice_z: GRID_SIZE / 2,
            cursor_x: GRID_SIZE / 2,
            cursor_y: GRID_SIZE / 2,
        }
    }
}

pub struct CaveWidget<'a> {
    grid: &'a VoxelGrid,
    state: &'a TuiState,
    positions: &'a HashMap<NodeIndex, Vec3>,
    graph: &'a CrateGraph,
}

impl<'a> CaveWidget<'a> {
    pub fn new(
        grid: &'a VoxelGrid,
        state: &'a TuiState,
        positions: &'a HashMap<NodeIndex, Vec3>,
        graph: &'a CrateGraph,
    ) -> Self {
        Self { grid, state, positions, graph }
    }
}

impl<'a> Widget for CaveWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(format!(" Slice Z: {}/{} ", self.state.slice_z, GRID_SIZE))
            .borders(Borders::ALL);
        let inner = block.inner(area);
        block.render(area, buf);

        let z = self.state.slice_z;

        for y in 0..self.grid.height.min(inner.height as usize) {
            for x in 0..self.grid.width.min(inner.width as usize) {
                let voxel = self.grid.get(x, y, z);
                let char_sym = match voxel {
                    Voxel::Rock => "█",
                    Voxel::Air => " ",
                    Voxel::Crack => "░",
                };
                let style = match voxel {
                    Voxel::Rock => Style::default().fg(Color::DarkGray),
                    Voxel::Air => Style::default(),
                    Voxel::Crack => Style::default().fg(Color::Red),
                };

                if let Some(c) = buf.cell_mut((inner.x + x as u16, inner.y + y as u16)) {
                       c.set_symbol(char_sym);
                       c.set_style(style);
                }
            }
        }

        // Render Nodes that are on this slice (or near it)
        for (node, pos) in self.positions {
             let px = (pos.x * self.grid.width as f32) as usize;
             let py = (pos.y * self.grid.height as f32) as usize;
             let pz = (pos.z * self.grid.depth as f32) as usize;

             // Show nodes within +/- 1 Z level
             if pz.abs_diff(z) <= 1 {
                 if px < inner.width as usize && py < inner.height as usize {
                      let name = &self.graph[*node];
                      // Just show first char
                      let symbol = &name[0..1];
                      let color = if pz == z { Color::Yellow } else { Color::Rgb(100, 100, 0) }; // Dimmer if not exact Z
                      if let Some(c) = buf.cell_mut((inner.x + px as u16, inner.y + py as u16)) {
                             c.set_symbol(symbol);
                             c.set_style(Style::default().fg(color).add_modifier(Modifier::BOLD));
                      }
                 }
             }
        }

        // Render Cursor
        if self.state.cursor_x < inner.width as usize && self.state.cursor_y < inner.height as usize {
             if let Some(c) = buf.cell_mut((inner.x + self.state.cursor_x as u16, inner.y + self.state.cursor_y as u16)) {
                    c.set_style(Style::default().bg(Color::Blue));
             }
        }
    }
}
