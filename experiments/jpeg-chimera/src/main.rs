use macroquad::prelude::*;
use image::{Rgba, RgbaImage, Pixel};
use ::rand::{Rng, thread_rng};
use chimera_lang::prelude::*;

mod dct;
use dct::{dct_2d, idct_2d};

#[derive(Clone, Copy)]
pub struct Block {
    pub y: [f32; 64],
    pub cb: [f32; 64],
    pub cr: [f32; 64],
}

impl Block {
    pub fn new() -> Self {
        Self {
            y: [0.0; 64],
            cb: [0.0; 64],
            cr: [0.0; 64],
        }
    }
}

struct Agent {
    vm: ChimeraVM,
    bx: u32,
    by: u32,
    color: Color,
}

pub struct JpegChimera {
    width_blocks: u32,
    height_blocks: u32,
    blocks: Vec<Block>,
    agents: Vec<Agent>,
    texture: Texture2D,
    image: Image,
}

impl JpegChimera {
    pub fn new(img: RgbaImage) -> Self {
        let (w, h) = img.dimensions();
        // Ensure dimensions are multiples of 16 for 2x2 block mapping
        let w = (w / 16) * 16;
        let h = (h / 16) * 16;
        let img_buffer = image::imageops::crop_imm(&img, 0, 0, w, h).to_image();

        let width_blocks = w / 8;
        let height_blocks = h / 8;
        let count = (width_blocks * height_blocks) as usize;

        let mut blocks = vec![Block::new(); count];

        // DCT Encoding
        for by in 0..height_blocks {
            for bx in 0..width_blocks {
                let mut y_block = [0.0; 64];
                let mut cb_block = [0.0; 64];
                let mut cr_block = [0.0; 64];

                for y in 0..8 {
                    for x in 0..8 {
                        let px = img_buffer.get_pixel(bx * 8 + x, by * 8 + y);
                        let r = px[0] as f32;
                        let g = px[1] as f32;
                        let b = px[2] as f32;

                        let y_val = 0.299 * r + 0.587 * g + 0.114 * b - 128.0;
                        let cb_val = -0.1687 * r - 0.3313 * g + 0.5 * b;
                        let cr_val = 0.5 * r - 0.4187 * g - 0.0813 * b;

                        y_block[(y * 8 + x) as usize] = y_val;
                        cb_block[(y * 8 + x) as usize] = cb_val;
                        cr_block[(y * 8 + x) as usize] = cr_val;
                    }
                }

                let idx = (by * width_blocks + bx) as usize;
                blocks[idx] = Block {
                    y: dct_2d(&y_block),
                    cb: dct_2d(&cb_block),
                    cr: dct_2d(&cr_block),
                };
            }
        }

        let texture = Texture2D::from_image(&Image::gen_image_color(w as u16, h as u16, WHITE));
        // We create an internal Image buffer for Macroquad to use for updates
        // We initialize it with black or noise, it will be overwritten.
        let image = Image {
            bytes: vec![0u8; (w * h * 4) as usize],
            width: w as u16,
            height: h as u16,
        };

        Self {
            width_blocks,
            height_blocks,
            blocks,
            agents: Vec::new(),
            texture,
            image,
        }
    }

    pub fn spawn_agents(&mut self, count: usize) {
        let mut rng = thread_rng();
        for _ in 0..count {
            // Agents live on 2x2 block boundaries (16x16 pixels)
            let max_bx = self.width_blocks / 2;
            let max_by = self.height_blocks / 2;

            let grid_x = rng.gen_range(0..max_bx);
            let grid_y = rng.gen_range(0..max_by);

            let bx = grid_x * 2;
            let by = grid_y * 2;

            // DNA: A simple loop that reads/writes to grid randomly
            let genes = vec![
                // 0: Push 100 (Loop Limit)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
                // 1: Push random X (0-15)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
                // 2: Push random Y (0-15)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
                // 3: Read
                Gene { op: OpCode::GRead, args: vec![] },
                // 4: Push Mutation Factor
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                // 5: Add
                Gene { op: OpCode::Add, args: vec![] },
                // 6: Push random X
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
                // 7: Push random Y
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
                // 8: Write
                Gene { op: OpCode::GWrite, args: vec![] },
                // 9: Jump to start
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] },
            ];

            let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };

            let mut vm = ChimeraVM::new(dna);
            vm.chaos_mode = true;
            vm.energy = 1000;

            self.agents.push(Agent {
                vm,
                bx,
                by,
                color: Color::new(rng.gen(), rng.gen(), rng.gen(), 0.8),
            });
        }
    }

    pub fn update(&mut self) {
        let width = self.width_blocks;
        let height = self.height_blocks;

        let blocks = &mut self.blocks;
        let agents = &mut self.agents;

        // Run agents
        for agent in agents {
            // 1. Sync VM grid from DCT blocks
            Self::sync_dct_to_vm(blocks, width, height, agent);

            // 2. Step VM
            for _ in 0..10 {
                agent.vm.step();
            }

            // 3. Sync VM grid back to DCT blocks
            Self::sync_vm_to_dct(blocks, width, height, agent);

            // Keep alive
            agent.vm.energy = 1000;
        }

        // Reconstruct Image
        self.reconstruct_image();
    }

    fn sync_dct_to_vm(blocks: &[Block], width_blocks: u32, height_blocks: u32, agent: &mut Agent) {
        let offsets = [(0, 0), (1, 0), (0, 1), (1, 1)];

        for (_idx, (ox, oy)) in offsets.iter().enumerate() {
            let bx = agent.bx + ox;
            let by = agent.by + oy;

            if bx >= width_blocks || by >= height_blocks { continue; }

            let block_idx = (by * width_blocks + bx) as usize;
            let block = &blocks[block_idx];

            let start_y = oy * 8;
            let start_x = ox * 8;

            for y in 0..8 {
                for x in 0..8 {
                    let val = block.y[(y * 8 + x) as usize];
                    // Map f32 to Int for VM (scaled)
                    let vm_val = (val * 10.0) as i64;
                    agent.vm.grid[(start_y + y) as usize][(start_x + x) as usize] = Value::Int(vm_val);
                }
            }
        }
    }

    fn sync_vm_to_dct(blocks: &mut [Block], width_blocks: u32, height_blocks: u32, agent: &Agent) {
        let offsets = [(0, 0), (1, 0), (0, 1), (1, 1)];

        for (_idx, (ox, oy)) in offsets.iter().enumerate() {
            let bx = agent.bx + ox;
            let by = agent.by + oy;

            if bx >= width_blocks || by >= height_blocks { continue; }

            let block_idx = (by * width_blocks + bx) as usize;
            let block = &mut blocks[block_idx];

            let start_y = oy * 8;
            let start_x = ox * 8;

            for y in 0..8 {
                for x in 0..8 {
                    if let Value::Int(vm_val) = agent.vm.grid[(start_y + y) as usize][(start_x + x) as usize] {
                        let val = vm_val as f32 / 10.0;
                        block.y[(y * 8 + x) as usize] = val;
                    }
                }
            }
        }
    }

    fn reconstruct_image(&mut self) {
        let width_pixels = (self.width_blocks * 8) as usize;

        for by in 0..self.height_blocks {
            for bx in 0..self.width_blocks {
                let idx = (by * self.width_blocks + bx) as usize;
                let block = &self.blocks[idx];

                let y_spatial = idct_2d(&block.y);
                let cb_spatial = idct_2d(&block.cb);
                let cr_spatial = idct_2d(&block.cr);

                for y in 0..8 {
                    for x in 0..8 {
                        let i = (y * 8 + x) as usize;
                        let y_val = y_spatial[i] + 128.0;
                        let cb_val = cb_spatial[i];
                        let cr_val = cr_spatial[i];

                        let r = (y_val + 1.402 * cr_val).clamp(0.0, 255.0) as u8;
                        let g = (y_val - 0.34414 * cb_val - 0.71414 * cr_val).clamp(0.0, 255.0) as u8;
                        let b = (y_val + 1.772 * cb_val).clamp(0.0, 255.0) as u8;

                        let img_x = bx * 8 + x;
                        let img_y = by * 8 + y;
                        let pixel_idx = ((img_y as usize * width_pixels + img_x as usize) * 4) as usize;

                        self.image.bytes[pixel_idx] = r;
                        self.image.bytes[pixel_idx + 1] = g;
                        self.image.bytes[pixel_idx + 2] = b;
                        self.image.bytes[pixel_idx + 3] = 255;
                    }
                }
            }
        }

        self.texture.update(&self.image);
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, 0.0, 0.0, WHITE);

        // Draw agents
        for agent in &self.agents {
            draw_rectangle(
                (agent.bx * 8) as f32,
                (agent.by * 8) as f32,
                16.0,
                16.0,
                agent.color
            );
            draw_rectangle_lines(
                (agent.bx * 8) as f32,
                (agent.by * 8) as f32,
                16.0,
                16.0,
                1.0,
                BLACK
            );
        }
    }
}

#[macroquad::main("JPEG Chimera")]
async fn main() {
    // Generate a noise image
    let w = 512;
    let h = 512;
    let mut img = RgbaImage::new(w, h);
    let mut rng = thread_rng();

    for y in 0..h {
        for x in 0..w {
            let val = rng.gen_range(0..255);
            img.put_pixel(x, y, Rgba([val, val, val, 255]));
        }
    }

    let mut simulation = JpegChimera::new(img);
    simulation.spawn_agents(20);

    loop {
        clear_background(LIGHTGRAY);

        simulation.update();
        simulation.draw();

        draw_text("JPEG Chimera", 10.0, 20.0, 30.0, BLACK);
        draw_text(&format!("Agents: {}", simulation.agents.len()), 10.0, 50.0, 20.0, DARKGRAY);

        next_frame().await;
    }
}
