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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_turtle_movement() {
        // F+F
        // Start (0,0, 0) -> F -> (10, 0)
        // + (90 deg) -> angle = PI/2
        // F -> (10, 10)
        let mut turtle = Turtle::new(0.0, 0.0, 0.0, 10.0, PI / 2.0);
        let lines = turtle.interpret("F+F");

        assert_eq!(lines.len(), 2);

        // Line 1: (0,0) to (10,0)
        assert!((lines[0].x1 - 0.0).abs() < 1e-6);
        assert!((lines[0].y1 - 0.0).abs() < 1e-6);
        assert!((lines[0].x2 - 10.0).abs() < 1e-6);
        assert!((lines[0].y2 - 0.0).abs() < 1e-6);

        // Line 2: (10,0) to (10,10)
        assert!((lines[1].x1 - 10.0).abs() < 1e-6);
        assert!((lines[1].y1 - 0.0).abs() < 1e-6);
        assert!((lines[1].x2 - 10.0).abs() < 1e-6);
        assert!((lines[1].y2 - 10.0).abs() < 1e-6);
    }
}
