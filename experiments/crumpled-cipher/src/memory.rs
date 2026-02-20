use crate::starmap::StarMap;
use image::{Rgba, RgbaImage};

pub struct Memory {
    pub width: u32,
    pub height: u32,
    pub hidden_layer: RgbaImage,
    pub visible_layer: RgbaImage,
}

impl Memory {
    pub fn new(width: u32, height: u32) -> Self {
        // Generate Hidden Layer (StarMap)
        let message = "THE UNIVERSE IS FOLDED. LIGHT EMERGES FROM THE CREASE.";

        // Ensure starmap matches dimensions (resize if needed, but starmap generates based on payload)
        // Actually, starmap generates size based on payload length and width.
        // We want it to fill the texture.

        // Let's generate a starmap and then resize/crop or tile it?
        // Or just force starmap width.
        // StarMap::new takes width_glyphs. width_pixels = width_glyphs * 12.
        let width_glyphs = width / 12;
        let sm = StarMap::new(message.as_bytes(), width_glyphs);
        let hidden_generated = sm.generate();

        // Resize hidden_generated to match target width/height if needed, or just paste it
        let mut hidden_layer = RgbaImage::new(width, height);

        // Fill hidden layer with void
        for p in hidden_layer.pixels_mut() {
            *p = Rgba([5, 5, 15, 255]);
        }

        // Paste generated starmap
        for y in 0..hidden_generated.height().min(height) {
            for x in 0..hidden_generated.width().min(width) {
                hidden_layer.put_pixel(x, y, *hidden_generated.get_pixel(x, y));
            }
        }

        // Visible Layer: Start as Void
        let mut visible_layer = RgbaImage::new(width, height);
        for p in visible_layer.pixels_mut() {
            *p = Rgba([0, 0, 0, 255]); // Pitch black initially
        }

        Self {
            width,
            height,
            hidden_layer,
            visible_layer,
        }
    }
}
