use crate::vm::iterate_circle;
use crate::opcode::OpCode;
use crate::value::Value;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_grid_op(&mut self, op: OpCode) -> Option<(usize, usize)> {
        match op {
            OpCode::GRead => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if self.is_valid_coord(y, x) {
                            self.stack.push(self.grid[y as usize][x as usize].clone());
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_read".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_read".to_string());
                }
            }
            OpCode::GWrite => {
                #[cfg(feature = "nova")]
                if self.phase == crate::vm::nova::Phase::Ethereal {
                    self.output
                        .push("Error: Ethereal phase prevents GWrite".to_string());
                    return None;
                }

                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if self.is_valid_coord(y, x) {
                            self.grid[y as usize][x as usize] = val;
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_write".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_write".to_string());
                }
            }
            OpCode::Radiate => {
                if self.stack.len() >= 4 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
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
                    } else {
                        self.output
                            .push("Error: Type mismatch for radiate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for radiate".to_string());
                }
            }
            OpCode::Siphon => {
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
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
                    } else {
                        self.output
                            .push("Error: Type mismatch for siphon".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for siphon".to_string());
                }
            }
            OpCode::Virus => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();

                    let coords = if let (Value::Int(y), Value::Int(x)) = (&y_val, &x_val) {
                        if self.is_valid_coord(*y, *x) {
                            Some((y, x))
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                            None
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for virus".to_string());
                        None
                    };

                    if let Some((y, x)) = coords {
                        let val = self.grid[*y as usize][*x as usize].clone();
                        match val {
                            Value::Int(n) => self.stack.push(Value::Int(n)),
                            Value::Str(s) => {
                                let old_loc = self.context_loc;
                                self.context_loc = (*y as usize, *x as usize);
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
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for virus".to_string());
                }
            }
            _ => {}
        }
        None
    }

}
