use crate::parser::{CodeEntity, EntityKind};
use resonance_audio::physics::PhysicsGrid;

pub fn map_to_grid(entities: &[CodeEntity], lines: &[String], grid: &mut PhysicsGrid) {
    grid.clear_walls();

    for entity in entities {
        // Line indices are 1-based in syn, 0-based in grid.
        let start_y = entity.start.line.saturating_sub(1);
        let end_y = entity.end.line.saturating_sub(1);

        if start_y >= grid.height {
            continue;
        }
        // Clamp end_y
        let end_y = end_y.min(grid.height - 1);

        // Find max width of lines in this range
        let mut max_width = 0;
        for y in start_y..=end_y {
            if let Some(line) = lines.get(y) {
                max_width = max_width.max(line.len());
            }
        }

        // Clamp width
        let max_width = max_width.min(grid.width - 1);
        let start_x = entity.start.column.min(grid.width - 1);

        match entity.kind {
            EntityKind::Function => {
                // Hollow room
                // Top
                draw_line(grid, start_x, start_y, max_width, start_y);
                // Bottom
                draw_line(grid, start_x, end_y, max_width, end_y);
                // Left
                draw_line(grid, start_x, start_y, start_x, end_y);
                // Right
                draw_line(grid, max_width, start_y, max_width, end_y);

                // Add an opening for entry/exit (e.g. arguments)
                if max_width > start_x + 10 {
                    grid.remove_wall(start_x + 5, start_y);
                    grid.remove_wall(start_x + 6, start_y);
                }
            }
            EntityKind::Struct => {
                // Solid block - reflects rigidity of data layout
                for y in start_y..=end_y {
                    for x in start_x..=max_width {
                        grid.add_wall(x, y);
                    }
                }
            }
            EntityKind::Impl => {
                // Large container
                draw_line(grid, start_x, start_y, max_width, start_y);
                draw_line(grid, start_x, end_y, max_width, end_y);
                draw_line(grid, start_x, start_y, start_x, end_y);
                draw_line(grid, max_width, start_y, max_width, end_y);

                // Impls are permeable? Maybe openings on the side?
                 if (end_y - start_y) > 5 {
                    grid.remove_wall(start_x, start_y + 2);
                    grid.remove_wall(start_x, start_y + 3);
                 }
            }
            _ => {}
        }
    }
}

fn draw_line(grid: &mut PhysicsGrid, x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 == x2 {
        for y in y1.min(y2)..=y1.max(y2) {
            grid.add_wall(x1, y);
        }
    } else if y1 == y2 {
        for x in x1.min(x2)..=x1.max(x2) {
            grid.add_wall(x, y1);
        }
    } else {
        // Simple diagonal (bresenham-ish but lazy)
        // Not needed for rects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{CodeEntity, EntityKind};
    use syn::spanned::Spanned;

    #[test]
    fn test_map_function() {
        let code = "fn test() {}";
        let file = syn::parse_file(code).unwrap();
        let item = &file.items[0];
        let span = item.span();

        let entity = CodeEntity {
            kind: EntityKind::Function,
            name: "test".to_string(),
            start: span.start(),
            end: span.end(),
        };

        let lines = vec!["fn test() {}".to_string()];
        let mut grid = PhysicsGrid::new(20, 10);

        map_to_grid(&[entity], &lines, &mut grid);

        // Start line 1 -> grid Y 0.
        // End line 1 -> grid Y 0.
        // Start col 0 -> grid X 0.
        // End col 12?

        // Walls should be at Y=0.
        // Check grid.walls index 0 (0,0)
        // Since it's a function, it draws outline.
        // With height 1 (start_y == end_y), it draws top and bottom at same line.
        assert!(grid.walls[0]);
    }
}
