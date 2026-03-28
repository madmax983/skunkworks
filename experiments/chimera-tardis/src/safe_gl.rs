use macroquad::miniquad::gl;

/// A RAII guard for OpenGL scissor operations.
///
/// Enables the scissor test upon creation and restores the previous state (or disables it)
/// when dropped. This ensures exception safety and prevents state leaks.
pub struct ScopedScissor {
    parent: Option<(i32, i32, i32, i32)>,
}

impl Drop for ScopedScissor {
    fn drop(&mut self) {
        unsafe {
            if let Some((x, y, w, h)) = self.parent {
                gl::glEnable(gl::GL_SCISSOR_TEST);
                gl::glScissor(
                    x.clamp(-16384, 16384),
                    y.clamp(-16384, 16384),
                    w.clamp(0, 32768),
                    h.clamp(0, 32768),
                );
            } else {
                gl::glDisable(gl::GL_SCISSOR_TEST);
            }
        }
    }
}

/// Helper function to calculate the intersection of two rectangles (x, y, w, h).
/// Ensures widths and heights are treated as non-negative to prevent UB.
pub fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let ax1 = a.0 as i64;
    let ay1 = a.1 as i64;
    let ax2 = ax1 + a.2.max(0) as i64;
    let ay2 = ay1 + a.3.max(0) as i64;

    let bx1 = b.0 as i64;
    let by1 = b.1 as i64;
    let bx2 = bx1 + b.2.max(0) as i64;
    let by2 = by1 + b.3.max(0) as i64;

    let rx1 = ax1.max(bx1);
    let ry1 = ay1.max(by1);
    let rx2 = ax2.min(bx2);
    let ry2 = ay2.min(by2);

    if rx1 < rx2 && ry1 < ry2 {
        let rw = (rx2 - rx1).min(i32::MAX as i64) as i32;
        let rh = (ry2 - ry1).min(i32::MAX as i64) as i32;
        (rx1 as i32, ry1 as i32, rw, rh)
    } else {
        (0, 0, 0, 0)
    }
}

/// Executes the given closure `f` with a scissor rectangle applied.
/// The scissor test is enabled before `f` runs and disabled afterwards (even on panic).
///
/// `parent` is the previous scissor rect, if any. The applied rect will be the intersection
/// of `(x, y, w, h)` and `parent`.
pub fn with_scissor<F: FnOnce((i32, i32, i32, i32))>(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    parent: Option<(i32, i32, i32, i32)>,
    f: F,
) {
    // Sanitize parent to ensure safety for ScopedScissor.
    // Negative width/height can cause Undefined Behavior in glScissor.
    // Explicit clamp to avoid coordinates overflowing when driver calculates bounds.
    let safe_parent = parent.map(|(px, py, pw, ph)| {
        (
            px.clamp(-16384, 16384),
            py.clamp(-16384, 16384),
            pw.clamp(0, 32768),
            ph.clamp(0, 32768),
        )
    });

    let (final_x, final_y, final_w, final_h) = if let Some(p) = safe_parent {
        intersect_rect((x, y, w, h), p)
    } else {
        (
            x.clamp(-16384, 16384),
            y.clamp(-16384, 16384),
            w.clamp(0, 32768),
            h.clamp(0, 32768),
        )
    };

    unsafe {
        gl::glEnable(gl::GL_SCISSOR_TEST);
        gl::glScissor(final_x, final_y, final_w, final_h);
    }

    let _guard = ScopedScissor {
        parent: safe_parent,
    };
    f((final_x, final_y, final_w, final_h));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_rect_basic() {
        let r1 = (0, 0, 100, 100);
        let r2 = (50, 50, 100, 100);
        let intersection = intersect_rect(r1, r2);
        assert_eq!(intersection, (50, 50, 50, 50));
    }

    #[test]
    fn test_intersect_rect_contained() {
        let r1 = (0, 0, 100, 100);
        let r2 = (20, 20, 50, 50);
        let intersection = intersect_rect(r1, r2);
        assert_eq!(intersection, (20, 20, 50, 50));
    }

    #[test]
    fn test_intersect_rect_disjoint() {
        let r1 = (0, 0, 10, 10);
        let r2 = (20, 20, 10, 10);
        let intersection = intersect_rect(r1, r2);
        assert_eq!(intersection.2, 0); // width 0
        assert_eq!(intersection.3, 0); // height 0
    }

    #[test]
    fn test_intersect_rect_overflow() {
        // Test with large coordinates that would overflow standard addition
        let r1 = (i32::MAX - 100, 0, 200, 100);
        let r2 = (i32::MAX - 50, 0, 200, 100);
        // Using i64, r1 ends at MAX + 100.
        // r2 ends at MAX + 150.
        // Intersection x ranges from MAX - 50 to MAX + 100.
        // Width is 150.

        let intersection = intersect_rect(r1, r2);
        assert_eq!(intersection.0, i32::MAX - 50); // x1
        assert_eq!(intersection.2, 150); // width
    }
}
