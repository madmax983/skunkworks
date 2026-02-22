use crate::math::Vec4D;

#[derive(Clone, Copy, Debug)]
pub struct TurtleState {
    pub pos: Vec4D,
    pub dir: Vec4D,
}

pub struct Turtle {
    pub state: TurtleState,
    pub stack: Vec<TurtleState>,
    pub step_size: f32,
    pub angle_step: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Line4D {
    pub start: Vec4D,
    pub end: Vec4D,
}

impl Turtle {
    pub fn new(pos: Vec4D, dir: Vec4D, step_size: f32, angle_step: f32) -> Self {
        Self {
            state: TurtleState { pos, dir },
            stack: Vec::new(),
            step_size,
            angle_step,
        }
    }

    pub fn interpret(&mut self, instructions: &str) -> Vec<Line4D> {
        let mut lines = Vec::new();

        for c in instructions.chars() {
            match c {
                'F' => {
                    let new_pos = self.state.pos + self.state.dir.scale(self.step_size);
                    lines.push(Line4D {
                        start: self.state.pos,
                        end: new_pos,
                    });
                    self.state.pos = new_pos;
                }
                'f' => {
                    self.state.pos = self.state.pos + self.state.dir.scale(self.step_size);
                }
                '+' => {
                    // Rotate XY (Yaw)
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
                    // Rotate YZ (Pitch)
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
                    // Rotate YW (Roll into 4D)
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
            self.state.dir = self.state.dir.normalize();
        }
        lines
    }
}
