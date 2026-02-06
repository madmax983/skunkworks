use macroquad::miniquad::gl;

struct ScissorGuard;

impl Drop for ScissorGuard {
    fn drop(&mut self) {
        unsafe {
            gl::glDisable(gl::GL_SCISSOR_TEST);
        }
    }
}

/// Executes the given closure `f` with a scissor rectangle applied.
/// The scissor test is enabled before `f` runs and disabled afterwards (even on panic).
pub fn with_scissor<F: FnOnce()>(x: i32, y: i32, w: i32, h: i32, f: F) {
    unsafe {
        gl::glEnable(gl::GL_SCISSOR_TEST);
        gl::glScissor(x, y, w, h);
    }
    let _guard = ScissorGuard;
    f();
}
