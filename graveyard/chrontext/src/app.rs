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
        let content_str = fs::read_to_string(&path)?;
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
