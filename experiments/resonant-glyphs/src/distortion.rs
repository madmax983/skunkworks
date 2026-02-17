use crate::font::{GlyphOutline, Contour, Segment, Point};

pub fn distort(outline: &GlyphOutline, time: f64, freq: f32, amp: f32) -> GlyphOutline {
    let contours = outline.contours.iter().map(|c| {
        let start = distort_point(c.start, time, freq, amp);
        let segments = c.segments.iter().map(|s| {
            match s {
                Segment::Line(p) => Segment::Line(distort_point(*p, time, freq, amp)),
                Segment::Quad(p1, p2) => Segment::Quad(
                    distort_point(*p1, time, freq, amp),
                    distort_point(*p2, time, freq, amp),
                ),
                Segment::Cubic(p1, p2, p3) => Segment::Cubic(
                    distort_point(*p1, time, freq, amp),
                    distort_point(*p2, time, freq, amp),
                    distort_point(*p3, time, freq, amp),
                ),
            }
        }).collect();

        Contour {
            start,
            segments,
            closed: c.closed,
        }
    }).collect();

    GlyphOutline {
        contours,
        advance_width: outline.advance_width,
    }
}

fn distort_point(p: Point, time: f64, freq: f32, amp: f32) -> Point {
    // Wave propagates along Y (vertical) or X (horizontal)?
    // Let's do a wave traveling horizontally (X), affecting Y?
    // Or a radial wave?
    // Let's do:
    // x' = x + sin(y * freq + time) * amp
    // y' = y + cos(x * freq + time) * amp
    // This creates a weird swirl.

    // For "Audio Visualization" typically we want simpler waves.
    // Let's do:
    // y' = y + sin(x * freq + time) * amp
    // x' = x
    // But let's add some x distortion too.

    let offset_x = (p.y * freq + time as f32).sin() * amp * 0.5; // Less x distortion
    let offset_y = (p.x * freq + time as f32).cos() * amp;

    Point::new(p.x + offset_x, p.y + offset_y)
}

pub fn flatten(outline: &GlyphOutline, steps: usize) -> Vec<Vec<Point>> {
    let mut result = Vec::new();

    for contour in &outline.contours {
        let mut points = Vec::new();
        points.push(contour.start);

        let mut current = contour.start;
        for segment in &contour.segments {
            match segment {
                Segment::Line(p) => {
                    points.push(*p);
                    current = *p;
                }
                Segment::Quad(p1, p2) => {
                    for i in 1..=steps {
                        let t = i as f32 / steps as f32;
                        let p = eval_quad(current, *p1, *p2, t);
                        points.push(p);
                    }
                    current = *p2;
                }
                Segment::Cubic(p1, p2, p3) => {
                    for i in 1..=steps {
                        let t = i as f32 / steps as f32;
                        let p = eval_cubic(current, *p1, *p2, *p3, t);
                        points.push(p);
                    }
                    current = *p3;
                }
            }
        }

        if contour.closed {
            points.push(contour.start);
        }

        result.push(points);
    }

    result
}

fn eval_quad(p0: Point, p1: Point, p2: Point, t: f32) -> Point {
    let mt = 1.0 - t;
    let x = mt * mt * p0.x + 2.0 * mt * t * p1.x + t * t * p2.x;
    let y = mt * mt * p0.y + 2.0 * mt * t * p1.y + t * t * p2.y;
    Point::new(x, y)
}

fn eval_cubic(p0: Point, p1: Point, p2: Point, p3: Point, t: f32) -> Point {
    let mt = 1.0 - t;
    let x = mt * mt * mt * p0.x + 3.0 * mt * mt * t * p1.x + 3.0 * mt * t * t * p2.x + t * t * t * p3.x;
    let y = mt * mt * mt * p0.y + 3.0 * mt * mt * t * p1.y + 3.0 * mt * t * t * p2.y + t * t * t * p3.y;
    Point::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::{GlyphOutline, Contour, Segment, Point};

    #[test]
    fn test_distort() {
        let outline = GlyphOutline {
            contours: vec![Contour {
                start: Point::new(0.0, 0.0),
                segments: vec![Segment::Line(Point::new(10.0, 0.0))],
                closed: false,
            }],
            advance_width: 10.0,
        };

        let distorted = distort(&outline, 0.0, 0.1, 5.0);

        // Check if start point moved.
        // start (0,0):
        // offset_x = sin(0) * ... = 0
        // offset_y = cos(0) * 5 = 5
        // new start: (0, 5)

        assert!((distorted.contours[0].start.y - 5.0).abs() < 0.001);
    }
}
