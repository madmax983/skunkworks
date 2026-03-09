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
                gl::glScissor(x, y, w, h);
            } else {
                gl::glDisable(gl::GL_SCISSOR_TEST);
            }
        }
    }
}

/// Helper function to calculate the intersection of two rectangles (x, y, w, h).
/// Ensures widths and heights are treated as non-negative to prevent UB.
pub fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let x1 = a.0.max(b.0);
    let y1 = a.1.max(b.1);
    let ax2 = a.0.saturating_add(a.2.max(0));
    let bx2 = b.0.saturating_add(b.2.max(0));
    let ay2 = a.1.saturating_add(a.3.max(0));
    let by2 = b.1.saturating_add(b.3.max(0));
    let x2 = ax2.min(bx2);
    let y2 = ay2.min(by2);
    (
        x1,
        y1,
        x2.saturating_sub(x1).max(0),
        y2.saturating_sub(y1).max(0),
    )
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
    let safe_parent = parent.map(|(px, py, pw, ph)| (
        px.clamp(-16384, 16384),
        py.clamp(-16384, 16384),
        pw.max(0),
        ph.max(0)
    ));

    let (final_x, final_y, final_w, final_h) = if let Some(p) = safe_parent {
        intersect_rect((x, y, w, h), p)
    } else {
        (x.clamp(-16384, 16384), y.clamp(-16384, 16384), w.max(0), h.max(0))
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
        // r1 ends at MAX - 100 + 200 = MAX + 100 (Saturated to MAX)
        // r2 starts at MAX - 50.
        // Intersection should start at MAX - 50.
        // End at MAX.
        // Width = 50.

        let intersection = intersect_rect(r1, r2);
        assert_eq!(intersection.0, i32::MAX - 50); // x1
        assert_eq!(intersection.2, 50); // width
    }
}
