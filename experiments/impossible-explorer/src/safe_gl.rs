use macroquad::miniquad::gl;

/// A RAII guard for OpenGL scissor operations.
///
/// Enables the scissor test upon creation and restores the previous state (or disables it)
/// when dropped. This ensures exception safety and prevents state leaks.
pub struct ScopedScissor {
    parent: Option<(i32, i32, i32, i32)>,
}

impl ScopedScissor {
    /// Creates a new `ScopedScissor` with the given rectangle.
    ///
    /// The width and height are clamped to be non-negative.
    /// The parent scissor rect (if any) is stored to be restored on drop.
    pub fn new(x: i32, y: i32, w: i32, h: i32, parent: Option<(i32, i32, i32, i32)>) -> Self {
        let w = w.max(0);
        let h = h.max(0);

        // Sanitize parent to ensure safety for Drop.
        let safe_parent = parent.map(|(px, py, pw, ph)| (px, py, pw.max(0), ph.max(0)));

        unsafe {
            gl::glEnable(gl::GL_SCISSOR_TEST);
            gl::glScissor(x, y, w, h);
        }
        Self {
            parent: safe_parent,
        }
    }
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

/// Clears the depth buffer.
///
/// This function wraps the unsafe `gl::glClear(gl::GL_DEPTH_BUFFER_BIT)` call.
pub fn clear_depth_buffer() {
    unsafe {
        gl::glClear(gl::GL_DEPTH_BUFFER_BIT);
    }
}

#[cfg(test)]
mod havoc_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        #[should_panic]
        fn test_scoped_scissor_havoc(
            x in any::<i32>(), y in any::<i32>(), w in any::<i32>(), h in any::<i32>(),
            px in any::<i32>(), py in any::<i32>(), pw in any::<i32>(), ph in any::<i32>()
        ) {
            let _ = ScopedScissor::new(x, y, w, h, Some((px, py, pw, ph)));
        }
    }
}
