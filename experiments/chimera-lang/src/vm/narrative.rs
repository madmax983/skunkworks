use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::ChimeraVM;

/// A high-level builder for constructing narratives on the Chimera grid.
///
/// This abstracts away the low-level details of grid manipulation,
/// AST construction, and reverse stack pushing.
pub struct NarrativeBuilder<'a> {
    vm: &'a mut ChimeraVM,
    start_y: usize,
    start_x: usize,
    current_y: usize,
    current_x: usize,
    story_length: usize,
}

impl<'a> NarrativeBuilder<'a> {
    /// Creates a new `NarrativeBuilder` wrapping a `ChimeraVM`.
    pub fn new(vm: &'a mut ChimeraVM, start_y: usize, start_x: usize) -> Self {
        Self {
            vm,
            start_y,
            start_x,
            current_y: start_y,
            current_x: start_x,
            story_length: 0,
        }
    }

    /// Writes a story element (e.g., text) to the Petri Dish.
    /// It automatically handles placing the "push", the argument, and the "print" instructions.
    pub fn write_story(&mut self, text: &str) -> &mut Self {
        if self.current_x + 2 >= 16 {
            self.current_y += 1;
            self.current_x = 0;
            if self.current_y >= 16 {
                return self;
            }
        }

        self.vm.grid[self.current_y][self.current_x] = Value::Str("push".to_string());
        self.vm.grid[self.current_y][self.current_x + 1] = Value::Str(text.to_string());
        self.vm.grid[self.current_y][self.current_x + 2] = Value::Str("print".to_string());

        self.story_length += 3;
        self.current_x += 3;

        self
    }

    /// Writes an instruction directly.
    pub fn write_instruction(&mut self, instruction: &str) -> &mut Self {
        if self.current_x >= 16 {
            self.current_y += 1;
            self.current_x = 0;
            if self.current_y >= 16 {
                return self;
            }
        }

        self.vm.grid[self.current_y][self.current_x] = Value::Str(instruction.to_string());
        self.story_length += 1;
        self.current_x += 1;

        self
    }

    /// Writes a value directly.
    pub fn write_value(&mut self, value: i64) -> &mut Self {
        if self.current_x >= 16 {
            self.current_y += 1;
            self.current_x = 0;
            if self.current_y >= 16 {
                return self;
            }
        }

        self.vm.grid[self.current_y][self.current_x] = Value::Int(value);
        self.story_length += 1;
        self.current_x += 1;

        self
    }

    /// Creates a "Reader" strand that incubates the story and adds it to the VM's DNA.
    pub fn incubate(self) {
        if self.story_length == 0 {
            return; // Nothing to incubate
        }

        let reader_strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(self.story_length as i64)],
                }, // len
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(self.start_y as i64)],
                }, // y
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(self.start_x as i64)],
                }, // x
                Gene {
                    op: OpCode::Incubate,
                    args: vec![],
                },
            ],
        };

        self.vm.dna.helix.strands.push(reader_strand);
    }
}