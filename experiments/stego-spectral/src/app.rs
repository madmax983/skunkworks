use crate::stego::StegoBuffer;
use anyhow::Result;
use rand::Rng;

pub enum ViewMode {
    Normal,
    BitPlane,
}

pub struct App {
    pub buffer: StegoBuffer,
    pub input: String,
    pub view_mode: ViewMode,
    pub status_message: String,
    pub time: f64,
}

impl App {
    pub fn new() -> Self {
        let width = 120;
        let height = 60;
        let mut buffer = StegoBuffer::new(width, height);
        let mut rng = rand::thread_rng();
        let time = rng.gen_range(0.0..100.0);

        buffer.generate_plasma(time);

        Self {
            buffer,
            input: String::new(),
            view_mode: ViewMode::Normal,
            status_message: "Ready. Type message and press Enter to Embed. [F5] New Pattern"
                .to_string(),
            time,
        }
    }

    pub fn regenerate_noise(&mut self) {
        let mut rng = rand::thread_rng();
        self.time += rng.gen_range(1.0..10.0);
        self.buffer.generate_plasma(self.time);
        self.status_message = "New Pattern Generated. Message wiped.".to_string();
    }

    pub fn embed_message(&mut self) {
        if self.input.is_empty() {
            self.status_message = "Error: Message is empty.".to_string();
            return;
        }

        // Regenerate noise slightly? Or just embed.
        match self.buffer.embed(self.input.as_bytes()) {
            Ok(_) => self.status_message = "Message Embedded successfully!".to_string(),
            Err(e) => self.status_message = format!("Error: {}", e),
        }
    }

    pub fn extract_message(&mut self) {
        match self.buffer.extract() {
            Ok(bytes) => {
                let msg = String::from_utf8_lossy(&bytes);
                self.status_message = format!("Extracted: {}", msg);
            }
            Err(e) => self.status_message = format!("Extract Error: {}", e),
        }
    }

    pub fn toggle_view(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Normal => ViewMode::BitPlane,
            ViewMode::BitPlane => ViewMode::Normal,
        };
    }
}
