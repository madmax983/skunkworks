use crate::math::Vec3;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

const MAX_STEPS: usize = 64;
const MAX_DIST: f64 = 20.0;
const SURF_DIST: f64 = 0.01;

pub struct SdfView<F>
where
    F: Fn(Vec3) -> f64,
{
    pub sdf: F,
    pub camera_pos: Vec3,
    pub camera_target: Vec3,
    pub time: f64,
}

impl<F> SdfView<F>
where
    F: Fn(Vec3) -> f64,
{
    pub fn new(sdf: F, camera_pos: Vec3, camera_target: Vec3, time: f64) -> Self {
        Self {
            sdf,
            camera_pos,
            camera_target,
            time,
        }
    }
}

fn ray_march(ro: Vec3, rd: Vec3, sdf_fn: &impl Fn(Vec3) -> f64) -> f64 {
    let mut d0 = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = sdf_fn(p);
        d0 += ds;
        if d0 > MAX_DIST || ds.abs() < SURF_DIST {
            break;
        }
    }
    d0
}

fn get_normal(p: Vec3, sdf_fn: &impl Fn(Vec3) -> f64) -> Vec3 {
    let e = 0.001;
    let dx = sdf_fn(Vec3::new(p.x + e, p.y, p.z)) - sdf_fn(Vec3::new(p.x - e, p.y, p.z));
    let dy = sdf_fn(Vec3::new(p.x, p.y + e, p.z)) - sdf_fn(Vec3::new(p.x, p.y - e, p.z));
    let dz = sdf_fn(Vec3::new(p.x, p.y, p.z + e)) - sdf_fn(Vec3::new(p.x, p.y, p.z - e));
    Vec3::new(dx, dy, dz).normalize()
}

fn get_ascii_char(intensity: f64) -> char {
    let chars = " .:-=+*#%@";
    let index = (intensity * (chars.len() as f64 - 1.0)).round() as usize;
    chars
        .chars()
        .nth(index.clamp(0, chars.len() - 1))
        .unwrap_or(' ')
}

fn get_color(intensity: f64) -> Color {
    if intensity < 0.2 {
        Color::Blue
    } else if intensity < 0.5 {
        Color::Cyan
    } else if intensity < 0.8 {
        Color::White
    } else {
        Color::Yellow
    }
}

impl<F> Widget for SdfView<F>
where
    F: Fn(Vec3) -> f64,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = area.width as f64;
        let height = area.height as f64;

        let forward = (self.camera_target - self.camera_pos).normalize();
        let right = forward.cross(Vec3::Y).normalize();
        let up = right.cross(forward).normalize();

        let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();

        for y in 0..area.height {
            let v_pixel = (area.height - y) as f64;
            for x in 0..area.width {
                let u = (x as f64 - width * 0.5) / height;
                let v = (v_pixel - height * 0.5) / height * 2.0;

                let dir = (forward + right * u + up * v).normalize();

                let dist = ray_march(self.camera_pos, dir, &self.sdf);

                let cell_x = area.left() + x;
                let cell_y = area.top() + y;

                if let Some(cell) = buf.cell_mut((cell_x, cell_y)) {
                    if dist < MAX_DIST {
                        let p = self.camera_pos + dir * dist;
                        let normal = get_normal(p, &self.sdf);
                        let diff = normal.dot(light_dir).max(0.1);

                        let char = get_ascii_char(diff);
                        let color = get_color(diff);

                        cell.set_char(char).set_fg(color);
                    } else {
                        cell.set_char(' ');
                    }
                }
            }
        }
    }
}
