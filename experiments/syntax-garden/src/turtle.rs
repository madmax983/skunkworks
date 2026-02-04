#[derive(Debug, Clone, Copy)]
pub struct TurtleState {
    pub x: f64,
    pub y: f64,
    pub angle: f64, // Radians
    pub depth: usize,
}

pub struct Turtle {
    pub stack: Vec<TurtleState>,
    pub current: TurtleState,
    pub lines: Vec<(f64, f64, f64, f64, usize)>, // x1, y1, x2, y2, depth
    pub angle_step: f64,
    pub step_len: f64,
    pub bounds: (f64, f64, f64, f64), // min_x, max_x, min_y, max_y
}

impl Turtle {
    pub fn new(x: f64, y: f64, angle_deg: f64, angle_step_deg: f64, step_len: f64) -> Self {
        Self {
            stack: Vec::new(),
            current: TurtleState {
                x,
                y,
                angle: angle_deg.to_radians(),
                depth: 0,
            },
            lines: Vec::new(),
            angle_step: angle_step_deg.to_radians(),
            step_len,
            bounds: (x, x, y, y),
        }
    }

    pub fn forward(&mut self, draw: bool) {
        let new_x = self.current.x + self.step_len * self.current.angle.cos();
        let new_y = self.current.y + self.step_len * self.current.angle.sin();

        if draw {
            self.lines.push((self.current.x, self.current.y, new_x, new_y, self.current.depth));
        }

        self.current.x = new_x;
        self.current.y = new_y;
        self.update_bounds(new_x, new_y);
    }

    pub fn turn(&mut self, direction: f64) {
        // direction: 1.0 for right (+), -1.0 for left (-)
        self.current.angle += direction * self.angle_step;
    }

    pub fn push(&mut self) {
        self.current.depth += 1;
        self.stack.push(self.current);
    }

    pub fn pop(&mut self) {
        if let Some(state) = self.stack.pop() {
            self.current = state;
            // Note: we don't update bounds on pop because we just jumped back
        }
    }

    fn update_bounds(&mut self, x: f64, y: f64) {
        if x < self.bounds.0 { self.bounds.0 = x; }
        if x > self.bounds.1 { self.bounds.1 = x; }
        if y < self.bounds.2 { self.bounds.2 = y; }
        if y > self.bounds.3 { self.bounds.3 = y; }
    }

    pub fn process_str(&mut self, s: &str) {
        for c in s.chars() {
            match c {
                'F' | 'G' => self.forward(true),
                'f' => self.forward(false),
                '+' => self.turn(1.0),
                '-' => self.turn(-1.0),
                '[' => self.push(),
                ']' => self.pop(),
                _ => {} // Ignore other symbols
            }
        }
    }
}
