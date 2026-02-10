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
    pub width: f32, // branch width
}

pub struct Turtle {
    pub state: TurtleState,
    pub stack: Vec<TurtleState>,
    pub step_size: f32,
    pub turn_angle: f32,
    pub width_decay: f32,
}

impl Turtle {
    pub fn new(position: Vec2, angle: f32, step_size: f32, turn_angle: f32, width_decay: f32) -> Self {
        Self {
            state: TurtleState {
                position,
                angle,
                width: 1.0 // Initial width
            },
            stack: Vec::new(),
            step_size,
            turn_angle,
            width_decay,
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
                self.state.width *= self.width_decay;
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
