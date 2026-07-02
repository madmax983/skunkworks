#[allow(clippy::module_inception)]
pub(crate) mod ast {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Program {
        pub strands: HashMap<String, Strand>,
        pub organelles: HashMap<String, Organelle>,
        pub grids: HashMap<String, Grid>,
        pub rules: HashMap<String, Rule>,
    }

    #[derive(Debug, Clone)]
    pub struct Strand {
        pub name: String,
        pub instructions: Vec<Instruction>,
    }

    #[derive(Debug, Clone)]
    pub struct Organelle {
        pub name: String,
        pub instructions: Vec<Instruction>,
    }

    #[derive(Debug, Clone)]
    pub struct Grid {
        pub name: String,
        pub rows: Vec<Vec<char>>,
    }

    #[derive(Debug, Clone)]
    pub struct Rule {
        pub head: String,
        pub vars: Vec<String>,
        pub goals: Vec<Goal>,
    }

    #[derive(Debug, Clone)]
    pub struct Goal {
        pub predicate: String,
        pub vars: Vec<String>,
    }

    #[derive(Debug, Clone)]
    pub enum Instruction {
        Number(i64),
        String(String),
        Identifier(String),
        Operator(String),
        Spawn(String),
        Call(String),
        Query(String),
    }
}
