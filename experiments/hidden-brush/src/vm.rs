use crate::bytecode::Instruction;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct LineSegment {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub color: (u8, u8, u8),
}

pub struct Turtle {
    pub x: f64,
    pub y: f64,
    pub angle: f64, // Degrees
    pub pen_down: bool,
    pub color: (u8, u8, u8),
    pub step_size: f64,
    pub path: Vec<LineSegment>,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            pen_down: true,
            color: (255, 255, 255),
            step_size: 10.0, // Default step
            path: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.angle = 0.0;
        self.pen_down = true;
        self.color = (255, 255, 255);
        self.step_size = 10.0;
        self.path.clear();
    }

    pub fn run(&mut self, instructions: &[Instruction]) -> Result<()> {
        self.execute_slice(instructions)
    }

    fn execute_slice(&mut self, instructions: &[Instruction]) -> Result<()> {
        let mut pc = 0;
        while pc < instructions.len() {
            let instr = &instructions[pc];
            pc += 1;

            match instr {
                Instruction::End => {
                    // Stop execution? Or just do nothing?
                    // Typically END stops everything.
                    // But in a slice, maybe it just returns?
                    return Ok(());
                }
                Instruction::Fwd => {
                    let rad = self.angle.to_radians();
                    let dx = rad.cos() * self.step_size;
                    let dy = rad.sin() * self.step_size;
                    let new_x = self.x + dx;
                    let new_y = self.y + dy;

                    if self.pen_down {
                        self.path.push(LineSegment {
                            start: (self.x, self.y),
                            end: (new_x, new_y),
                            color: self.color,
                        });
                    }
                    self.x = new_x;
                    self.y = new_y;
                }
                Instruction::Rot(deg) => {
                    self.angle += *deg as f64;
                }
                Instruction::Pen(down) => {
                    self.pen_down = *down;
                }
                Instruction::Color(r, g, b) => {
                    self.color = (*r, *g, *b);
                }
                Instruction::SetStep(val) => {
                    self.step_size = *val as f64;
                }
                Instruction::AddStep(val) => {
                    self.step_size += *val as f64;
                }
                Instruction::Rep(count, len) => {
                    let len = *len as usize;
                    if pc + len > instructions.len() {
                        // Not enough instructions for the block
                        // Just ignore or error?
                        // Let's stop to be safe
                        break;
                    }
                    let block = &instructions[pc..pc + len];
                    for _ in 0..*count {
                        self.execute_slice(block)?;
                    }
                    pc += len;
                }
            }
        }
        Ok(())
    }
}
