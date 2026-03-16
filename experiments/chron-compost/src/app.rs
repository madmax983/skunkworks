use crate::blame::LineInfo;
use anyhow::Result;
use std::fs;

pub struct App {
    pub path: String,
    pub content: Vec<String>,
    pub blame_info: Vec<LineInfo>,
    pub scroll: usize,
    pub selected_line: usize,
}

impl App {
    pub fn new(path: String, blame_info: Vec<LineInfo>) -> Result<Self> {
        let mut file = std::fs::File::open(&path)?;
        let mut content_str = String::new();
        use std::io::Read;
        let limit = 10 * 1024 * 1024;
        let bytes_read = file.take(limit + 1).read_to_string(&mut content_str)?;
        if bytes_read > limit as usize {
            anyhow::bail!("File is too large! Maximum allowed size is 10MB.");
        }
        let content: Vec<String> = content_str.lines().map(|s| s.to_string()).collect();

        Ok(Self {
            path,
            content,
            blame_info,
            scroll: 0,
            selected_line: 0,
        })
    }

    pub fn next_line(&mut self) {
        if self.selected_line < self.content.len().saturating_sub(1) {
            self.selected_line += 1;
        }
    }

    pub fn previous_line(&mut self) {
        if self.selected_line > 0 {
            self.selected_line -= 1;
        }
    }

    pub fn update_scroll(&mut self, height: usize) {
        if height == 0 {
            return;
        }
        // Keep selected_line in view [scroll, scroll + height)
        if self.selected_line >= self.scroll + height {
            self.scroll = self.selected_line + 1 - height;
        } else if self.selected_line < self.scroll {
            self.scroll = self.selected_line;
        }
    }
}
