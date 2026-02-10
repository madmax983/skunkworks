use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Cursor;
use std::path::Path;
use tar::Archive;

pub fn pack(source_dir: &Path) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    {
        let mut tar = tar::Builder::new(&mut encoder);
        // We pack everything relative to the source directory
        tar.append_dir_all(".", source_dir)
            .context("Failed to append directory to tar")?;
        tar.finish().context("Failed to finish tar archive")?;
    }
    encoder.finish().context("Failed to finish GzEncoder")
}

pub fn unpack(data: &[u8], dest_dir: &Path) -> Result<()> {
    let cursor = Cursor::new(data);
    let decoder = GzDecoder::new(cursor);
    let mut archive = Archive::new(decoder);
    archive
        .unpack(dest_dir)
        .context("Failed to unpack archive")?;
    Ok(())
}
