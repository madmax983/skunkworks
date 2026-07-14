use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::iterate_circle;

impl crate::vm::ChimeraVM {
    pub(crate) fn apply_g_read(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() < 2 {
            self.output
                .push("Error: Stack underflow for g_read".to_string());
            return None;
        }

        let x_val = self.stack.pop().unwrap();
        let y_val = self.stack.pop().unwrap();

        let (Value::Int(y), Value::Int(x)) = (y_val, x_val) else {
            self.output
                .push("Error: Type mismatch for g_read".to_string());
            return None;
        };

        if !self.is_valid_coord(y, x) {
            self.output
                .push("Error: Grid index out of bounds".to_string());
            return None;
        }

        self.stack.push(self.grid[y as usize][x as usize].clone());
        None
    }

    pub(crate) fn apply_g_write(&mut self) -> Option<(usize, usize)> {
        #[cfg(feature = "nova")]
        if self.phase == crate::vm::nova::Phase::Ethereal {
            self.output
                .push("Error: Ethereal phase prevents GWrite".to_string());
            return None;
        }

        if self.stack.len() < 3 {
            self.output
                .push("Error: Stack underflow for g_write".to_string());
            return None;
        }

        let x_val = self.stack.pop().unwrap();
        let y_val = self.stack.pop().unwrap();
        let val = self.stack.pop().unwrap();

        let (Value::Int(y), Value::Int(x)) = (y_val, x_val) else {
            self.output
                .push("Error: Type mismatch for g_write".to_string());
            return None;
        };

        if !self.is_valid_coord(y, x) {
            self.output
                .push("Error: Grid index out of bounds".to_string());
            return None;
        }

        self.grid[y as usize][x as usize] = val;
        None
    }

    pub(crate) fn apply_radiate(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() < 4 {
            self.output
                .push("Error: Stack underflow for radiate".to_string());
            return None;
        }

        let x_val = self.stack.pop().unwrap();
        let y_val = self.stack.pop().unwrap();
        let r_val = self.stack.pop().unwrap();
        let val = self.stack.pop().unwrap();

        let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) else {
            self.output
                .push("Error: Type mismatch for radiate".to_string());
            return None;
        };

        let mut count = 0;
        iterate_circle(
            #[cfg(feature = "nova")]
            self.topology,
            x,
            y,
            r,
            |cx, cy| {
                self.grid[cy][cx] = val.clone();
                count += 1;
            },
        );

        self.energy = self.energy.saturating_sub((count / 2) as i64);
        self.output.push(format!(
            "RADIATE: Affected {} cells at {},{} r={}",
            count, x, y, r
        ));
        None
    }

    pub(crate) fn apply_siphon(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() < 3 {
            self.output
                .push("Error: Stack underflow for siphon".to_string());
            return None;
        }

        let x_val = self.stack.pop().unwrap();
        let y_val = self.stack.pop().unwrap();
        let r_val = self.stack.pop().unwrap();

        let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) else {
            self.output
                .push("Error: Type mismatch for siphon".to_string());
            return None;
        };

        let mut count = 0;
        let mut sum: i64 = 0;

        iterate_circle(
            #[cfg(feature = "nova")]
            self.topology,
            x,
            y,
            r,
            |cx, cy| {
                if let Value::Int(n) = self.grid[cy][cx] {
                    sum = sum.saturating_add(n);
                }
                self.grid[cy][cx] = Value::Int(0);
                count += 1;
            },
        );

        self.stack.push(Value::Int(sum));
        self.energy = self.energy.saturating_sub(5);
        self.output
            .push(format!("SIPHON: Absorbed {} from {} cells", sum, count));
        None
    }

    pub(crate) fn apply_virus(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() < 2 {
            self.output
                .push("Error: Stack underflow for virus".to_string());
            return None;
        }

        let x_val = self.stack.pop().unwrap();
        let y_val = self.stack.pop().unwrap();

        let (Value::Int(y), Value::Int(x)) = (y_val, x_val) else {
            self.output
                .push("Error: Type mismatch for virus".to_string());
            return None;
        };

        if !self.is_valid_coord(y, x) {
            self.output
                .push("Error: Grid index out of bounds".to_string());
            return None;
        }

        let val = self.grid[y as usize][x as usize].clone();
        match val {
            Value::Int(n) => self.stack.push(Value::Int(n)),
            Value::Str(s) => {
                let old_loc = self.context_loc;
                self.context_loc = (y as usize, x as usize);
                let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                let result = self.execute_gene(op, &[]);
                self.context_loc = old_loc;
                return result;
            }
            Value::Junction(_, _) => {
                self.output
                    .push("Error: Virus cannot execute junction".to_string());
            }
            Value::Superposition(_) => {
                self.output
                    .push("Error: Virus cannot execute superposition".to_string());
            }
            Value::Symbol(_) => {
                self.output
                    .push("Error: Virus cannot execute symbol".to_string());
            }
            Value::Color(_, _, _) => {
                self.output
                    .push("Error: Virus cannot execute color".to_string());
            }
        }
        None
    }

    pub(crate) fn exec_grid_op(&mut self, op: OpCode) -> Option<(usize, usize)> {
        match op {
            OpCode::GRead => self.apply_g_read(),
            OpCode::GWrite => self.apply_g_write(),
            OpCode::Radiate => self.apply_radiate(),
            OpCode::Siphon => self.apply_siphon(),
            OpCode::Virus => self.apply_virus(),
            _ => None,
        }
    }
}
