#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Clone, Copy)]
struct State {
    x: f64,
    y: f64,
    angle: f64, // radians
}

pub struct Turtle {
    state: State,
    stack: Vec<State>,
    step_size: f64,
    angle_increment: f64,
}

impl Turtle {
    pub fn new(x: f64, y: f64, angle: f64, step_size: f64, angle_increment: f64) -> Self {
        Self {
            state: State { x, y, angle },
            stack: Vec::new(),
            step_size,
            angle_increment,
        }
    }

    pub fn interpret(&mut self, instructions: &str) -> Vec<Line> {
        let mut lines = Vec::new();

        for c in instructions.chars() {
            match c {
                'F' | 'G' => {
                    let new_x = self.state.x + self.step_size * self.state.angle.cos();
                    let new_y = self.state.y + self.step_size * self.state.angle.sin();

                    lines.push(Line {
                        x1: self.state.x,
                        y1: self.state.y,
                        x2: new_x,
                        y2: new_y,
                    });

                    self.state.x = new_x;
                    self.state.y = new_y;
                }
                '+' => {
                    self.state.angle += self.angle_increment;
                }
                '-' => {
                    self.state.angle -= self.angle_increment;
                }
                '[' => {
                    self.stack.push(self.state);
                }
                ']' => {
                    if let Some(saved_state) = self.stack.pop() {
                        self.state = saved_state;
                    }
                }
                _ => {} // Ignore other characters
            }
        }
        lines
    }
}
