use crate::model::{CodeConcerto, MusicalEvent, SectionKind};
use macroquad::prelude::*;

struct Building {
    position: Vec3,
    size: Vec3,
    color: Color,
    #[allow(dead_code)]
    section_index: usize,
    start_time: f64,
    duration: f64,
}

pub struct Visualizer {
    pub camera: Camera3D,
    pub concerto: CodeConcerto,
    buildings: Vec<Building>,
    pub current_time: f64,
    pub playhead_pos: Vec3,
}

impl Visualizer {
    pub fn new(concerto: CodeConcerto) -> Self {
        let mut buildings = Vec::new();
        let mut x_offset = 0.0;
        let mut current_start_time = 0.0;

        for (i, section) in concerto.sections.iter().enumerate() {
            // Width based on complexity
            let width = (section.events.len() as f32 * 0.5).clamp(4.0, 30.0);
            let height = ((section.depth as f32 + 1.0) * 5.0).clamp(5.0, 40.0);
            let depth = 5.0;

            let pos = vec3(x_offset, height / 2.0, 0.0);

            let color = match section.kind {
                SectionKind::Struct => Color::new(1.0, 0.3, 0.3, 1.0),
                SectionKind::Enum => Color::new(0.8, 0.3, 0.8, 1.0),
                SectionKind::Function => Color::new(0.3, 1.0, 0.3, 1.0),
                SectionKind::Impl => Color::new(0.3, 0.3, 1.0, 1.0),
                _ => Color::new(1.0, 1.0, 0.3, 1.0),
            };

            // Calculate duration
            let mut dur = 0.0;
            for event in &section.events {
                if let MusicalEvent::Wait(d) = event {
                    dur += d.as_secs_f64();
                }
            }
            if dur < 0.1 {
                dur = 1.0;
            } // Minimum duration fallback

            buildings.push(Building {
                position: pos,
                size: vec3(width, height, depth),
                color,
                section_index: i,
                start_time: current_start_time,
                duration: dur,
            });

            x_offset += width + 5.0; // Space between buildings
            current_start_time += dur;
        }

        Self {
            camera: Camera3D {
                position: vec3(0.0, 20.0, 30.0),
                target: vec3(0.0, 10.0, 0.0),
                up: vec3(0.0, 1.0, 0.0),
                fovy: 45.0,
                projection: Projection::Perspective,
                z_near: 0.1,
                z_far: 2000.0,
                ..Default::default()
            },
            concerto,
            buildings,
            current_time: 0.0,
            playhead_pos: vec3(0.0, 10.0, 0.0),
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.current_time += dt;

        let mut target_pos = self.playhead_pos;
        let mut active_building_idx = None;

        // Find active building
        for (idx, b) in self.buildings.iter().enumerate() {
            if self.current_time >= b.start_time && self.current_time < (b.start_time + b.duration)
            {
                let progress = (self.current_time - b.start_time) / b.duration;

                // Move across the building width
                let x_start = b.position.x - b.size.x / 2.0;
                let x_end = b.position.x + b.size.x / 2.0;
                let x = x_start + (x_end - x_start) * progress as f32;

                // Height: top of building
                let y = b.position.y + b.size.y / 2.0 + 2.0;

                target_pos = vec3(x, y, 0.0);
                active_building_idx = Some(idx);
                break;
            }
        }

        // If past the end, drift forward
        if active_building_idx.is_none() && !self.buildings.is_empty() {
            let last = self.buildings.last().unwrap();
            if self.current_time >= last.start_time + last.duration {
                let time_past = self.current_time - (last.start_time + last.duration);
                target_pos = last.position
                    + vec3(
                        last.size.x / 2.0 + time_past as f32 * 5.0,
                        last.size.y / 2.0 + 2.0,
                        0.0,
                    );
            }
        }

        // Smooth lerp for playhead
        self.playhead_pos = self.playhead_pos.lerp(target_pos, 0.1);

        // Camera logic
        // We want to see the playhead and the upcoming buildings
        let cam_target = self.playhead_pos + vec3(10.0, 0.0, 0.0);
        let cam_pos_offset = vec3(-10.0, 20.0, 40.0);

        self.camera.target = self.camera.target.lerp(cam_target, 0.05);
        self.camera.position = self.camera.position.lerp(cam_target + cam_pos_offset, 0.05);
    }

    pub fn draw(&self) {
        set_camera(&self.camera);

        draw_grid(200, 5.0, BLACK, GRAY);

        for b in &self.buildings {
            draw_cube(b.position, b.size, None, b.color);
            draw_cube_wires(b.position, b.size, BLACK);

            // Shadow/Reflection
            draw_cube(
                vec3(b.position.x, -0.5, b.position.z),
                vec3(b.size.x, 0.1, b.size.z),
                None,
                Color::new(b.color.r, b.color.g, b.color.b, 0.2),
            );

            // Name text? (3D text is hard in macroquad vanilla, skipping or using 2D overlay)
        }

        // Playhead
        draw_sphere(self.playhead_pos, 1.0, None, WHITE);
        // Beam down
        draw_line_3d(
            self.playhead_pos,
            vec3(self.playhead_pos.x, 0.0, self.playhead_pos.z),
            WHITE,
        );

        set_default_camera();

        // HUD
        draw_text(
            &format!("Time: {:.1}s", self.current_time),
            20.0,
            30.0,
            30.0,
            WHITE,
        );

        // Show current section name if any
        for b in &self.buildings {
            if self.current_time >= b.start_time && self.current_time < (b.start_time + b.duration)
            {
                if let Some(section) = self.concerto.sections.get(b.section_index) {
                    draw_text(
                        &format!("Now Playing: {}", section.name),
                        20.0,
                        60.0,
                        30.0,
                        YELLOW,
                    );
                    draw_text(
                        &format!("Type: {:?}", section.kind),
                        20.0,
                        90.0,
                        20.0,
                        LIGHTGRAY,
                    );
                }
                break;
            }
        }
    }
}
