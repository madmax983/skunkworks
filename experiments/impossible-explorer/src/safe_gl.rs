use macroquad::miniquad::gl;

/// A RAII guard for OpenGL scissor operations.
///
/// Enables the scissor test upon creation and restores the previous state (or disables it)
/// when dropped. This ensures exception safety and prevents state leaks.
pub struct ScopedScissor {
    restore: Option<(i32, i32, i32, i32)>,
}

impl ScopedScissor {
    /// Creates a new `ScopedScissor` with the given rectangle.
    ///
    /// The width and height are clamped to be non-negative.
    /// Automatically captures the current scissor state to restore on drop.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let w = w.max(0);
        let h = h.max(0);
        unsafe {
            // Constant for GL_SCISSOR_BOX if not defined in miniquad::gl
            const GL_SCISSOR_BOX: u32 = 0x0C10;

            // Capture current state
            let mut box_vals = [0i32; 4];
            gl::glGetIntegerv(GL_SCISSOR_BOX, box_vals.as_mut_ptr());

            // Check if enabled using glGetIntegerv with GL_SCISSOR_TEST
            // (GL_SCISSOR_TEST is 0x0C11)
            let mut enabled_val = [0i32];
            gl::glGetIntegerv(gl::GL_SCISSOR_TEST, enabled_val.as_mut_ptr());
            let was_enabled = enabled_val[0] != 0;

            // Set new state
            gl::glEnable(gl::GL_SCISSOR_TEST);
            gl::glScissor(x, y, w, h);

            Self {
                restore: if was_enabled {
                    Some((box_vals[0], box_vals[1], box_vals[2], box_vals[3]))
                } else {
                    None
                },
            }
        }
    }
}

impl Drop for ScopedScissor {
    fn drop(&mut self) {
        unsafe {
            if let Some((x, y, w, h)) = self.restore {
                gl::glScissor(x, y, w, h);
                gl::glEnable(gl::GL_SCISSOR_TEST);
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
