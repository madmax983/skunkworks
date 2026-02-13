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
                gl::glScissor(x, y, w, h);
            } else {
                gl::glDisable(gl::GL_SCISSOR_TEST);
            }
        }
    }
}

/// Helper function to calculate the intersection of two rectangles (x, y, w, h).
pub fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let x1 = a.0.max(b.0);
    let y1 = a.1.max(b.1);
    let x2 = (a.0 + a.2).min(b.0 + b.2);
    let y2 = (a.1 + a.3).min(b.1 + b.3);
    (x1, y1, (x2 - x1).max(0), (y2 - y1).max(0))
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
    let (final_x, final_y, final_w, final_h) = if let Some(p) = parent {
        intersect_rect((x, y, w, h), p)
    } else {
        (x, y, w, h)
    };

    unsafe {
        gl::glEnable(gl::GL_SCISSOR_TEST);
        gl::glScissor(final_x, final_y, final_w, final_h);
    }

    let _guard = ScopedScissor { parent };
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
}
