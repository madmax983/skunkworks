use crate::outline::{GlyphCurve, BezierSegment};
use crate::audio::Spectrum;
use macroquad::prelude::*;

pub fn deform(curve: &GlyphCurve, spectrum: &Spectrum, time: f32) -> GlyphCurve {
    let mut new_curve = GlyphCurve::new();
    new_curve.advance_width = curve.advance_width * (1.0 + spectrum.low * 0.1);

    // If bounds are invalid (empty glyph), just return empty
    if curve.min.x > curve.max.x {
        return new_curve;
    }

    let center = (curve.min + curve.max) * 0.5;

    for seg in &curve.segments {
        match seg {
            BezierSegment::Line { start, end } => {
                let s = distort(*start, center, spectrum, time, false);
                let e = distort(*end, center, spectrum, time, false);
                new_curve.segments.push(BezierSegment::Line { start: s, end: e });
                new_curve.update_bounds(s);
                new_curve.update_bounds(e);
            }
            BezierSegment::Quad { start, ctrl, end } => {
                let s = distort(*start, center, spectrum, time, false);
                let c = distort(*ctrl, center, spectrum, time, true);
                let e = distort(*end, center, spectrum, time, false);
                new_curve.segments.push(BezierSegment::Quad { start: s, ctrl: c, end: e });
                new_curve.update_bounds(s);
                new_curve.update_bounds(c);
                new_curve.update_bounds(e);
            }
            BezierSegment::Cubic { start, ctrl1, ctrl2, end } => {
                let s = distort(*start, center, spectrum, time, false);
                let c1 = distort(*ctrl1, center, spectrum, time, true);
                let c2 = distort(*ctrl2, center, spectrum, time, true);
                let e = distort(*end, center, spectrum, time, false);
                new_curve.segments.push(BezierSegment::Cubic { start: s, ctrl1: c1, ctrl2: c2, end: e });
                new_curve.update_bounds(s);
                new_curve.update_bounds(c1);
                new_curve.update_bounds(c2);
                new_curve.update_bounds(e);
            }
        }
    }
    new_curve
}

fn distort(p: Vec2, center: Vec2, spectrum: &Spectrum, time: f32, is_ctrl: bool) -> Vec2 {
    let mut p = p;

    // Bass: Scale from center (Puff out)
    let dir = p - center;
    let dist = dir.length();
    if dist > 0.0 {
        p += dir * spectrum.low * 0.3;
    }

    // Mid: Ripple effect
    // Freq based on distance from center
    let ripple = ((dist * 0.05) - time * 8.0).sin();
    p += dir.normalize_or_zero() * ripple * spectrum.mid * 5.0;

    // High: Jitter/Glitch
    // Use simple pseudo-random based on position
    let noise_x = (p.x * 0.1 + time * 10.0).sin();
    let noise_y = (p.y * 0.1 + time * 13.0).cos();

    p.x += noise_x * spectrum.high * 4.0;
    p.y += noise_y * spectrum.high * 4.0;

    if is_ctrl {
        // Exaggerate control points for wilder curves
        p += dir.normalize_or_zero() * spectrum.mid * 10.0;
    }

    p
}
