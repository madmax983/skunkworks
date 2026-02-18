use crate::vm::{Value, GRID_SIZE};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueAgent {
    pub x: usize,
    pub y: usize,
    pub state: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueState {
    pub active: bool,
    pub runes: HashSet<(usize, usize)>,
    pub rules: Vec<String>,
    pub signal_grid: Vec<Vec<Option<Value>>>,
    pub delayed_signals: Vec<Vec<Option<Value>>>,
    pub agents: Vec<PrologueAgent>,
    pub registers: HashMap<(usize, usize), Value>,
    pub teleport_channels: HashMap<i64, Value>,
    pub history: HashMap<(usize, usize), VecDeque<Value>>,
}

impl PrologueState {
    pub fn new() -> Self {
        Self {
            active: false,
            runes: HashSet::new(),
            rules: Vec::new(),
            signal_grid: vec![vec![None; GRID_SIZE]; GRID_SIZE],
            delayed_signals: vec![vec![None; GRID_SIZE]; GRID_SIZE],
            agents: Vec::new(),
            registers: HashMap::new(),
            teleport_channels: HashMap::new(),
            history: HashMap::new(),
        }
    }

    pub fn scan_grid_rules(&mut self, grid: &Vec<Vec<Value>>) {
        self.runes.clear();
        self.rules.clear();
        self.agents.clear();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Value::Str(s) = &grid[y][x] {
                    // Identify Runes
                    if matches!(
                        s.as_str(),
                        "?" | "!"
                            | "~"
                            | "&"
                            | "|"
                            | "+"
                            | "*"
                            | "#"
                            | "@"
                            | "$"
                            | "%"
                            | "^"
                            | "M"
                            | "O"
                            | "G"
                            | "E"
                            | "D"
                            | "A"
                            | "S"
                            | "P"
                            | "Q"
                            | "="
                            | ">"
                            | "<"
                            | "I"
                            | "Y"
                            | "L"
                            | "J"
                            | "C"
                            | "("
                            | "N"
                            | "W"
                            | "K"
                            | "R"
                            | "X"
                            | "Z"
                            | "H"
                            | "["
                            | "]"
                            | "U"
                            | "V"
                            | "F"
                            | "T"
                            | "\\"
                            | "/"
                            | "-"
                            | "q"
                            | "m"
                            | "{"
                            | "}"
                            | "s"
                            | "g"
                            | "r"
                            | "t"
                            | "f"
                            | "d"
                            | "e"
                            | "b"
                            | "l"
                            | "n"
                    ) {
                        self.runes.insert((y, x));

                        if s == "@" || s == "K" || s == "H" {
                            self.agents.push(PrologueAgent {
                                x,
                                y,
                                state: Value::Int(0),
                            });
                        }
                    }
                }
            }
        }
    }
}

impl Default for PrologueState {
    fn default() -> Self {
        Self::new()
    }
}
