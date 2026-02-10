mod archivist;
mod art;
mod stego;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "pixel-archivist")]
#[command(about = "Steganographic Archiver", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pack a directory into an image
    Pack {
        #[arg(value_name = "DIR")]
        dir: PathBuf,
        #[arg(value_name = "OUTPUT_IMAGE")]
        output: PathBuf,
    },
    /// Unpack an image into a directory
    Unpack {
        #[arg(value_name = "IMAGE")]
        image: PathBuf,
        #[arg(value_name = "OUTPUT_DIR")]
        output: PathBuf,
    },
    /// Visualize an archive image
    Gui {
        #[arg(value_name = "IMAGE")]
        image: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pack { dir, output } => {
            println!("Packing {:?}...", dir);
            let data = archivist::pack(&dir)?;
            println!("Compressed size: {} bytes", data.len());

            println!("Generating cover...");
            let cover = art::generate_cover(&data);

            println!("Embedding data...");
            let stego_image = stego::embed(cover, &data)?;

            stego_image.save(&output)?;
            println!("Saved to {:?}", output);
        }
        Commands::Unpack { image, output } => {
             println!("Opening {:?}...", image);
             let img = image::open(&image)?;

             println!("Extracting data...");
             let data = stego::extract(&img)?;
             println!("Extracted {} bytes", data.len());

             println!("Unpacking to {:?}...", output);
             archivist::unpack(&data, &output)?;
             println!("Done.");
        }
        Commands::Gui { image } => {
            // Start macroquad
            let future = gui_main(image);
            let mut conf = macroquad::conf::Conf::default();
            conf.miniquad_conf.window_title = "Pixel Archivist".to_string();
            conf.miniquad_conf.window_width = 800;
            conf.miniquad_conf.window_height = 600;

            macroquad::Window::from_config(conf, future);
        }
    }
    Ok(())
}

use macroquad::prelude::*;

async fn gui_main(path: Option<PathBuf>) {
    let mut texture: Option<Texture2D> = None;

    if let Some(p) = path {
         if let Ok(img) = image::open(&p) {
             let rgba = img.to_rgba8();
             let width = rgba.width();
             let height = rgba.height();
             let bytes = rgba.into_raw();

             // Convert to macroquad Image
             let mq_img = Image {
                 bytes,
                 width: width as u16,
                 height: height as u16,
             };

             texture = Some(Texture2D::from_image(&mq_img));
         }
    }

    loop {
        clear_background(BLACK);

        if let Some(tex) = &texture {
            draw_texture_ex(tex, 0.0, 0.0, WHITE, DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            });
            draw_text("Archive Loaded", 20.0, 30.0, 30.0, GREEN);
        } else {
            draw_text("Drag and Drop an Image (Not implemented yet)", 20.0, 300.0, 30.0, WHITE);
        }

        next_frame().await
    }
}
