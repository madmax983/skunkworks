use rusttype::{Font, OutlineBuilder, Scale};

struct PointCollector {
    points: Vec<(f64, f64)>,
    current: (f64, f64),
}

impl PointCollector {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            current: (0.0, 0.0),
        }
    }

    fn push(&mut self, x: f32, y: f32) {
        // Rusttype coordinates: y is up? No, y is up usually in fonts, but often screen coords y is down.
        // Rusttype documentation says: "The coordinate system is such that the baseline is at y=0, and y increases upwards."
        // We might need to flip it for TUI where y increases downwards.
        // Also we should scale it.
        self.points.push((x as f64, y as f64));
        self.current = (x as f64, y as f64);
    }
}

impl OutlineBuilder for PointCollector {
    fn move_to(&mut self, x: f32, y: f32) {
        self.push(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.push(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // De Casteljau's algorithm or just sampling
        let steps = 5;
        let (x0, y0) = self.current;

        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let mt = 1.0 - t;

            // Quadratic Bezier formula
            let bx = mt * mt * x0 + 2.0 * mt * t * (x1 as f64) + t * t * (x as f64);
            let by = mt * mt * y0 + 2.0 * mt * t * (y1 as f64) + t * t * (y as f64);

            self.points.push((bx, by));
        }
        self.current = (x as f64, y as f64);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // Cubic Bezier
        let steps = 10;
        let (x0, y0) = self.current;

        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let t2 = t * t;

            let bx = mt * mt2 * x0
                   + 3.0 * mt2 * t * (x1 as f64)
                   + 3.0 * mt * t2 * (x2 as f64)
                   + t * t2 * (x as f64);

            let by = mt * mt2 * y0
                   + 3.0 * mt2 * t * (y1 as f64)
                   + 3.0 * mt * t2 * (y2 as f64)
                   + t * t2 * (y as f64);

            self.points.push((bx, by));
        }
        self.current = (x as f64, y as f64);
    }

    fn close(&mut self) {
        // Optional: Close the loop explicitly?
        // For particle/point rendering, maybe not needed.
    }
}

pub fn extract_outline(font: &Font, c: char) -> Vec<(f64, f64)> {
    let scale = Scale::uniform(100.0); // Arbitrary scale
    let glyph = font.glyph(c).scaled(scale);

    // Position it at 0,0?
    let positioned = glyph.positioned(rusttype::point(0.0, 0.0));

    let mut collector = PointCollector::new();

    // Ensure we have a bounding box (glyph is visible)
    if positioned.pixel_bounding_box().is_some() {
        positioned.build_outline(&mut collector);
    }

    // Normalize or center?
    // Let's just return raw relative coordinates for now
    collector.points
}
