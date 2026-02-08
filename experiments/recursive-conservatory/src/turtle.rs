use macroquad::prelude::Vec2;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Move(Vec2, Vec2), // start, end
    Turn(f32),
    Push(usize), // depth after push
    Pop(usize),  // depth after pop
    None,
}

#[derive(Clone, Debug)]
pub struct TurtleState {
    pub position: Vec2,
    pub angle: f32, // radians
}

pub struct Turtle {
    pub state: TurtleState,
    pub stack: Vec<TurtleState>,
    pub step_size: f32,
    pub turn_angle: f32,
}

impl Turtle {
    pub fn new(position: Vec2, angle: f32, step_size: f32, turn_angle: f32) -> Self {
        Self {
            state: TurtleState { position, angle },
            stack: Vec::new(),
            step_size,
            turn_angle,
        }
    }

    pub fn interpret(&mut self, command: char) -> Action {
        match command {
            'F' | 'G' => {
                let start = self.state.position;
                let direction = Vec2::new(self.state.angle.cos(), self.state.angle.sin());
                let end = start + direction * self.step_size;
                self.state.position = end;
                Action::Move(start, end)
            }
            '+' => {
                self.state.angle += self.turn_angle;
                Action::Turn(self.state.angle)
            }
            '-' => {
                self.state.angle -= self.turn_angle;
                Action::Turn(self.state.angle)
            }
            '[' => {
                self.stack.push(self.state.clone());
                Action::Push(self.stack.len())
            }
            ']' => {
                if let Some(s) = self.stack.pop() {
                    self.state = s;
                    Action::Pop(self.stack.len())
                } else {
                    Action::None
                }
            }
            _ => Action::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let start = Vec2::new(0.0, 0.0);
        let mut t = Turtle::new(start, 0.0, 10.0, 90.0f32.to_radians());

        // Move F
        if let Action::Move(s, e) = t.interpret('F') {
            assert_eq!(s, start);
            assert_eq!(e, Vec2::new(10.0, 0.0));
        } else {
            panic!("Expected Move");
        }

        // Push [
        if let Action::Push(depth) = t.interpret('[') {
            assert_eq!(depth, 1);
        } else {
            panic!("Expected Push");
        }

        // Turn +
        t.interpret('+');
        assert!(t.state.angle > 0.0);

        // Pop ]
        if let Action::Pop(depth) = t.interpret(']') {
            assert_eq!(depth, 0);
            assert_eq!(t.state.angle, 0.0); // Restored
        } else {
            panic!("Expected Pop");
        }
    }
}
