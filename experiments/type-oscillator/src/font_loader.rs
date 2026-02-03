use anyhow::Result;
use rusttype::Font;

pub fn load_font() -> Result<Font<'static>> {
    // Embed the font to avoid runtime path issues and ensure portability
    let font_data = include_bytes!("../assets/DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).ok_or_else(|| anyhow::anyhow!("Error constructing Font"))?;

    Ok(font)
}
