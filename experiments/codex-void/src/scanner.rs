use image::{ImageBuffer, Rgb};
use crate::glyph::Glyph;
use crate::starmap::SPACING;

pub struct Scanner;

impl Scanner {
    pub fn decode_image(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
        let mut stars = Self::find_stars(img);

        // Sort stars to recover order
        // We assume a grid spacing of roughly SPACING pixels.
        // We group by rows.
        stars.sort_by(|a, b| {
            // Integer division by SPACING buckets Y coordinates into rows.
            let row_a = a.1 / SPACING;
            let row_b = b.1 / SPACING;
            if row_a != row_b {
                row_a.cmp(&row_b)
            } else {
                a.0.cmp(&b.0)
            }
        });

        stars.iter().map(|&(x, y)| {
            Glyph::decode(x, y, img)
        }).collect()
    }

    pub fn find_stars(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<(u32, u32)> {
        let mut stars = Vec::new();
        let (w, h) = img.dimensions();

        for y in 0..h {
            for x in 0..w {
                let p = img.get_pixel(x, y);
                // Check for core brightness (StarMap uses 255, 255, 255)
                // Salt noise is < 150.
                if p[0] > 200 && p[1] > 200 && p[2] > 200 {
                    stars.push((x, y));
                }
            }
        }
        stars
    }
}
