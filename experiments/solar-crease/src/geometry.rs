use macroquad::prelude::*;

pub struct MiuraTopology {
    pub rows: usize,
    pub cols: usize,
    pub indices: Vec<u16>,
}

pub fn generate_miura_topology(rows: usize, cols: usize) -> MiuraTopology {
    let mut indices = Vec::new();
    let width = cols + 1;

    for r in 0..rows {
        for c in 0..cols {
            // Vertices for the quad at (r, c)
            let tl = (r * width + c) as u16;
            let tr = (r * width + (c + 1)) as u16;
            let bl = ((r + 1) * width + c) as u16;
            let br = ((r + 1) * width + (c + 1)) as u16;

            // Triangle 1 (Top-Left, Bottom-Left, Top-Right)
            indices.push(tl);
            indices.push(bl);
            indices.push(tr);

            // Triangle 2 (Top-Right, Bottom-Left, Bottom-Right)
            indices.push(tr);
            indices.push(bl);
            indices.push(br);
        }
    }

    MiuraTopology { rows, cols, indices }
}
