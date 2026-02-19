use image::{Rgba, RgbaImage};

pub struct Memory {
    pub width: u32,
    pub height: u32,
    pub ground_truth: RgbaImage,
    pub perceived: RgbaImage,
}

impl Memory {
    pub fn new(width: u32, height: u32) -> Self {
        let mut ground_truth = RgbaImage::new(width, height);

        // Generate a test pattern: Gradient + Shapes
        for y in 0..height {
            for x in 0..width {
                let r = ((x as f32 / width as f32) * 255.0) as u8;
                let g = ((y as f32 / height as f32) * 255.0) as u8;
                let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u8;
                ground_truth.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        // Add some shapes
        let center_x = width / 2;
        let center_y = height / 2;
        let radius = width.min(height) / 4;

        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                if dx * dx + dy * dy < (radius * radius) as i32 {
                    let p = ground_truth.get_pixel_mut(x, y);
                    p[0] = 255 - p[0];
                    p[1] = 255 - p[1];
                    p[2] = 255 - p[2];
                }
            }
        }

        let perceived = ground_truth.clone();

        Self {
            width,
            height,
            ground_truth,
            perceived,
        }
    }

    pub fn from_image(img: RgbaImage) -> Self {
        let width = img.width();
        let height = img.height();
        let ground_truth = img.clone();
        let perceived = img.clone();
        Self {
            width,
            height,
            ground_truth,
            perceived,
        }
    }
}
