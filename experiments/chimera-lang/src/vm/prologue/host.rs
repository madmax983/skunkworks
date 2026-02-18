use crate::ast::{Dna, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::Value;
use std::collections::{HashMap, VecDeque};

pub trait PrologueHost {
    fn grid_read(&self, y: usize, x: usize) -> Value;
    fn grid_write(&mut self, y: usize, x: usize, val: Value);

    fn ether_get_map(&mut self) -> &mut HashMap<i64, VecDeque<Value>>;

    fn interrupt(&mut self, strand_idx: usize);
    fn output_push(&mut self, msg: String);
    fn dictionary_get(&self, name: &str) -> Option<usize>;

    fn execute_gene(&mut self, op: OpCode, args: &[Nucleotide]);

    fn tick_counter(&self) -> u64;
    fn dna(&self) -> &Dna;
    fn dna_mut(&mut self) -> &mut Dna;
}
