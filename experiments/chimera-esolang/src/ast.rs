#[allow(clippy::module_inception)]
pub(crate) mod ast {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    /// An esolang program AST.
    pub struct Program {
        /// The defined DNA strands.
        pub strands: HashMap<String, Strand>,
        /// The defined organelles.
        pub organelles: HashMap<String, Organelle>,
        /// The defined grids.
        pub grids: HashMap<String, Grid>,
        /// The defined logic rules.
        pub rules: HashMap<String, Rule>,
    }

    #[derive(Debug, Clone)]
    /// A sequence of instructions.
    pub struct Strand {
        /// The name of the item.
        pub name: String,
        /// The ordered instructions.
        pub instructions: Vec<Instruction>,
    }

    #[derive(Debug, Clone)]
    /// An autonomous agent definition.
    pub struct Organelle {
        /// The name of the item.
        pub name: String,
        /// The ordered instructions.
        pub instructions: Vec<Instruction>,
    }

    #[derive(Debug, Clone)]
    /// A spatial layout definition.
    pub struct Grid {
        /// The name of the item.
        pub name: String,
        /// The rows of the grid.
        pub rows: Vec<Vec<char>>,
    }

    #[derive(Debug, Clone)]
    pub struct Rule {
        pub head: String,
        /// The variables involved.
        pub vars: Vec<String>,
        pub goals: Vec<Goal>,
    }

    #[derive(Debug, Clone)]
    /// A Prolog-style goal.
    pub struct Goal {
        /// The goal predicate.
        pub predicate: String,
        /// The variables involved.
        pub vars: Vec<String>,
    }

    #[derive(Debug, Clone)]
    /// A single executable instruction.
    pub enum Instruction {
        /// A numeric literal.
        Number(i64),
        /// A string literal.
        String(String),
        /// A named identifier.
        Identifier(String),
        /// An operator symbol.
        Operator(String),
        /// Spawn an organelle.
        Spawn(String),
        /// Call a strand.
        Call(String),
        /// Execute a query.
        Query(String),
    }
}
