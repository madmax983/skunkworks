use crate::math::Vec4;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct TurtleState {
    pub pos: Vec4,
    pub dir: Vec4,
}

pub struct Turtle {
    pub state: TurtleState,
    pub stack: Vec<TurtleState>,
    pub step_size: f32,
    pub angle_step: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Line4D {
    pub start: Vec4,
    pub end: Vec4,
}

impl Turtle {
    pub fn new(pos: Vec4, dir: Vec4, step_size: f32, angle_step: f32) -> Self {
        Self {
            state: TurtleState { pos, dir },
            stack: Vec::new(),
            step_size,
            angle_step,
        }
    }

    pub fn interpret(&mut self, instructions: &str) -> Vec<Line4D> {
        let mut lines = Vec::new();
        // Start fresh or continue? Usually start fresh for rendering a frame,
        // but here we might want to just interpret.
        // Let's assume the caller resets state if needed.

        for c in instructions.chars() {
            match c {
                'F' => {
                    let new_pos = self.state.pos + self.state.dir * self.step_size;
                    lines.push(Line4D {
                        start: self.state.pos,
                        end: new_pos,
                    });
                    self.state.pos = new_pos;
                }
                'f' => {
                    self.state.pos = self.state.pos + self.state.dir * self.step_size;
                }
                '+' => {
                    // Rotate XY
                    let theta = self.angle_step;
                    let c = theta.cos();
                    let s = theta.sin();
                    let x = self.state.dir.x;
                    let y = self.state.dir.y;
                    self.state.dir.x = x * c - y * s;
                    self.state.dir.y = x * s + y * c;
                }
                '-' => {
                    let theta = -self.angle_step;
                    let c = theta.cos();
                    let s = theta.sin();
                    let x = self.state.dir.x;
                    let y = self.state.dir.y;
                    self.state.dir.x = x * c - y * s;
                    self.state.dir.y = x * s + y * c;
                }
                '&' => {
                    // Rotate YZ
                    let theta = self.angle_step;
                    let c = theta.cos();
                    let s = theta.sin();
                    let y = self.state.dir.y;
                    let z = self.state.dir.z;
                    self.state.dir.y = y * c - z * s;
                    self.state.dir.z = y * s + z * c;
                }
                '^' => {
                    let theta = -self.angle_step;
                    let c = theta.cos();
                    let s = theta.sin();
                    let y = self.state.dir.y;
                    let z = self.state.dir.z;
                    self.state.dir.y = y * c - z * s;
                    self.state.dir.z = y * s + z * c;
                }
                '>' => {
                    // Rotate XW
                    self.state.dir = self.state.dir.rotate_xw(self.angle_step);
                }
                '<' => {
                    self.state.dir = self.state.dir.rotate_xw(-self.angle_step);
                }
                '/' => {
                    // Rotate YW (just to add more 4D options)
                    self.state.dir = self.state.dir.rotate_yw(self.angle_step);
                }
                '\\' => {
                    self.state.dir = self.state.dir.rotate_yw(-self.angle_step);
                }
                '[' => {
                    self.stack.push(self.state);
                }
                ']' => {
                    if let Some(state) = self.stack.pop() {
                        self.state = state;
                    }
                }
                _ => {}
            }

            // Re-normalize direction to prevent scaling drift
            let len = (self.state.dir.x * self.state.dir.x +
                       self.state.dir.y * self.state.dir.y +
                       self.state.dir.z * self.state.dir.z +
                       self.state.dir.w * self.state.dir.w).sqrt();
            if len > 0.0001 {
               self.state.dir = self.state.dir * (1.0 / len);
            }
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turtle_movement() {
        let start_pos = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let start_dir = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let mut turtle = Turtle::new(start_pos, start_dir, 1.0, PI / 2.0);

        // F: Move 1 unit in Y
        let lines = turtle.interpret("F");
        assert_eq!(lines.len(), 1);
        assert!((lines[0].end.y - 1.0).abs() < 0.001);

        // +: Rotate 90 deg in XY (x = -y, y = x) -> (0, 1) -> (-1, 0)
        // Wait, rotation math:
        // x' = x*c - y*s = 0 - 1*1 = -1
        // y' = x*s + y*c = 0 + 1*0 = 0
        // Correct.
        turtle.interpret("+");
        assert!((turtle.state.dir.x + 1.0).abs() < 0.001);
        assert!(turtle.state.dir.y.abs() < 0.001);
    }
}
