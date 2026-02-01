use anyhow::Result;
use glam::Quat;
use std::path::Path;

use crate::fs::{self, FileNode};

pub struct App {
    pub files: Vec<FileNode>,
    pub rotation: Quat,
    pub zoom: f32,
    pub camera_dist: f32,
    pub auto_rotate: bool,
    pub running: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let files = fs::scan_directory(Path::new("."), 5)?;

        Ok(Self {
            files,
            rotation: Quat::IDENTITY,
            zoom: 40.0, // Scale factor for terminal chars
            camera_dist: 5.0,
            auto_rotate: true,
            running: true,
        })
    }

    pub fn on_tick(&mut self) {
        if self.auto_rotate {
            // Slowly rotate around Y and Z
            let delta = Quat::from_euler(glam::EulerRot::XYZ, 0.01, 0.02, 0.0);
            self.rotation *= delta;
        }
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        let delta = Quat::from_euler(glam::EulerRot::XYZ, dy, dx, 0.0);
        self.rotation *= delta;
    }
}
