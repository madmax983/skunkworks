pub mod glyph;
pub mod scanner;
pub mod starmap;

pub use glyph::Glyph;
pub use scanner::Scanner;
pub use starmap::StarMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_cycle() {
        let text = "Hello Codex Void!";
        let bytes = text.as_bytes();
        let width = 10; // width in glyphs

        // Encode
        let starmap = StarMap::new(bytes, width);
        let img = starmap.generate();

        // Decode
        let decoded_bytes = Scanner::decode_image(&img);
        let decoded_text =
            String::from_utf8(decoded_bytes).expect("Decoded bytes should be valid UTF-8");

        assert_eq!(
            text, decoded_text,
            "Decoded text should match original text"
        );
    }
}
