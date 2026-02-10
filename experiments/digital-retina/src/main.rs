use macroquad::prelude::*;
use ::rand::Rng; // Use root rand to avoid collision with macroquad::rand

const GRID_W: usize = 120;
const GRID_H: usize = 80;

#[derive(Clone, Copy, Debug)]
struct LIFNeuron {
    v: f32,
    v_th: f32,
    v_reset: f32,
    tau: f32,
    refractory: i32,
    refractory_period: i32,
}

impl LIFNeuron {
    fn new() -> Self {
        Self {
            v: 0.0,
            v_th: 1.0,
            v_reset: 0.0,
            tau: 0.2, // Faster response
            refractory: 0,
            refractory_period: 5,
        }
    }

    fn update(&mut self, input: f32) -> bool {
        if self.refractory > 0 {
            self.refractory -= 1;
            return false;
        }

        // LIF dynamics: V += (Input - V) * tau
        // Input is the driving current. If Input > V, V increases.
        self.v += (input - self.v) * self.tau;

        if self.v >= self.v_th {
            self.v = self.v_reset;
            self.refractory = self.refractory_period;
            return true;
        }
        false
    }
}

struct Retina {
    width: usize,
    height: usize,
    neurons: Vec<LIFNeuron>,
    photoreceptors: Vec<f32>,   // Layer 1: Raw Input
    horizontal_cells: Vec<f32>, // Layer 2: Lateral Inhibition (Smoothed)
    spikes: Vec<u8>,            // Layer 3: Output Spikes (Visual Decay)
    time: f32,
}

impl Retina {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            neurons: vec![LIFNeuron::new(); size],
            photoreceptors: vec![0.0; size],
            horizontal_cells: vec![0.0; size],
            spikes: vec![0; size],
            time: 0.0,
        }
    }

    fn update(&mut self, dt: f32, hallucinate: bool) {
        self.time += dt;
        let mut rng = ::rand::thread_rng();

        // 1. Update Photoreceptors (Input Pattern)
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                // Drifting Sine Grating
                let wave1 = ((x as f32 * 0.1) + self.time * 2.0).sin();
                let wave2 = ((y as f32 * 0.1) + self.time * 1.5).sin();

                // Add some noise
                let noise = rng.gen_range(-0.1..0.1);

                let mut intensity = (wave1 + wave2) * 0.5 + 0.5 + noise;

                if hallucinate {
                    // Feedback loop: Add previous horizontal cell state to input
                    intensity += self.horizontal_cells[idx] * 0.5;
                }

                self.photoreceptors[idx] = intensity.clamp(0.0, 2.0);
            }
        }

        // 2. Update Horizontal Cells (Lateral Inhibition / Blur)
        // Simple Box Blur
        let mut new_horizontal = self.horizontal_cells.clone();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let idx = y * self.width + x;
                let mut sum = 0.0;
                // 3x3 kernel
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let neighbor_idx = ((y as i32 + dy) as usize) * self.width + ((x as i32 + dx) as usize);
                        sum += self.photoreceptors[neighbor_idx];
                    }
                }
                new_horizontal[idx] = sum / 9.0;
            }
        }
        self.horizontal_cells = new_horizontal;

        // 3. Update Ganglion Cells (Difference of Gaussians -> LIF)
        for i in 0..self.neurons.len() {
            let center = self.photoreceptors[i];
            let surround = self.horizontal_cells[i];

            // On-Center / Off-Surround
            // Input current = Center - k * Surround
            // k > 1.0 enhances edges (lateral inhibition)
            let inhibition_strength = if hallucinate { 0.5 } else { 1.2 };
            let current = (center - surround * inhibition_strength) * 5.0; // Amplify for spiking

            let spiked = self.neurons[i].update(current);

            if spiked {
                self.spikes[i] = 255;
            } else {
                // Decay visual spike
                self.spikes[i] = self.spikes[i].saturating_sub(15);
            }
        }
    }
}

#[macroquad::main("Digital Retina")]
async fn main() {
    let mut retina = Retina::new(GRID_W, GRID_H);

    let mut input_image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLACK);
    let mut output_image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLACK);

    let input_texture = Texture2D::from_image(&input_image);
    let output_texture = Texture2D::from_image(&output_image);

    input_texture.set_filter(FilterMode::Nearest);
    output_texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();
        let hallucinate = is_key_down(KeyCode::Space);

        retina.update(dt, hallucinate);

        clear_background(BLACK);

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Update Images
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let idx = y * GRID_W + x;

                // Input
                let val = retina.photoreceptors[idx];
                input_image.set_pixel(x as u32, y as u32, Color::new(val, val, val, 1.0));

                // Output
                let spike_val = retina.spikes[idx] as f32 / 255.0;
                output_image.set_pixel(x as u32, y as u32, Color::new(0.0, spike_val, 0.0, 1.0));
            }
        }

        input_texture.update(&input_image);
        output_texture.update(&output_image);

        // Render Textures
        let view_w = screen_w / 2.0;
        let view_h = screen_h; // Full height, or maintain aspect ratio?

        // Maintain aspect ratio
        let scale = (view_w / GRID_W as f32).min(view_h / GRID_H as f32);
        let draw_w = GRID_W as f32 * scale;
        let draw_h = GRID_H as f32 * scale;
        let offset_y = (screen_h - draw_h) / 2.0;

        // Draw Input (Left)
        draw_texture_ex(
            &input_texture,
            (view_w - draw_w) / 2.0,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );

        // Draw Output (Right)
        draw_texture_ex(
            &output_texture,
            view_w + (view_w - draw_w) / 2.0,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );

        // UI
        draw_text("INPUT (Photoreceptors)", 10.0, 20.0, 20.0, RED);
        draw_text("OUTPUT (Ganglion Spikes)", view_w + 10.0, 20.0, 20.0, GREEN);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, GRAY);

        if hallucinate {
             draw_text("HALLUCINATING (Feedback Loop Active)", screen_w/2.0 - 150.0, screen_h - 30.0, 30.0, MAGENTA);
        } else {
             draw_text("Hold SPACE to Hallucinate", screen_w/2.0 - 100.0, screen_h - 30.0, 20.0, WHITE);
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_spike() {
        let mut neuron = LIFNeuron::new();
        let mut spiked = false;

        // Feed strong current for 20 steps
        for _ in 0..20 {
            if neuron.update(2.0) {
                spiked = true;
                break;
            }
        }

        assert!(spiked, "Neuron should spike given strong input");
        assert!(neuron.v < neuron.v_th, "Voltage should reset after spike");
    }
}
