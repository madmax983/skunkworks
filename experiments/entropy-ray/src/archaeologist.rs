use image::io::Reader as ImageReader;
use macroquad::texture::Image;
use std::io::Cursor;

pub fn recover_image(data: &[u8]) -> Option<Image> {
    // Try to decode as image format
    let cursor = Cursor::new(data);
    // with_guessed_format reads the first few bytes to guess.
    // If the header is corrupted, this might fail or return a generic error.
    match ImageReader::new(cursor).with_guessed_format() {
        Ok(reader) => match reader.decode() {
            Ok(dynamic_image) => {
                let rgba = dynamic_image.to_rgba8();
                let width = rgba.width() as u16;
                let height = rgba.height() as u16;
                let bytes = rgba.into_raw();
                Some(Image {
                    bytes,
                    width,
                    height,
                })
            }
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub fn raw_interpret(data: &[u8], width: u16, height: u16) -> Image {
    // Interpret bytes as raw RGB noise
    // The archaeologist makes a best guess: "It's just data".

    let num_pixels = (width as usize) * (height as usize);
    let mut bytes = Vec::with_capacity(num_pixels * 4);

    // We iterate through data. If we run out, wrap around.
    let mut data_iter = data.iter().cycle();

    for _ in 0..num_pixels {
        // R, G, B from data
        let r = *data_iter.next().unwrap_or(&0);
        let g = *data_iter.next().unwrap_or(&0);
        let b = *data_iter.next().unwrap_or(&0);
        // Alpha is always 255 to ensure visibility
        bytes.push(r);
        bytes.push(g);
        bytes.push(b);
        bytes.push(255);
    }

    Image {
        bytes,
        width,
        height,
    }
}
