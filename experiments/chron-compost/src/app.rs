use crate::blame::LineInfo;
use anyhow::Result;
use std::fs;

pub struct App {
    pub path: String,
    pub content: Vec<String>,
    #[allow(dead_code)]
    pub blame_info: Vec<LineInfo>,
    #[allow(dead_code)]
    pub scroll: usize,
    #[allow(dead_code)]
    pub selected_line: usize,
}

impl App {
    pub fn new(path: String, blame_info: Vec<LineInfo>) -> Result<Self> {
        let file = fs::File::open(&path)?;
        let mut content_str = String::new();
        let limit = 10 * 1024 * 1024; // 10MB limit
        let bytes_read = std::io::Read::read_to_string(&mut std::io::Read::take(file, limit + 1), &mut content_str)?;
        if bytes_read > limit as usize {
            anyhow::bail!("File too large");
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

    #[allow(dead_code)]
    pub fn next_line(&mut self) {
        if self.selected_line < self.content.len().saturating_sub(1) {
            self.selected_line += 1;
        }
    }

    #[allow(dead_code)]
    pub fn previous_line(&mut self) {
        if self.selected_line > 0 {
            self.selected_line -= 1;
        }
    }

    #[allow(dead_code)]
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
