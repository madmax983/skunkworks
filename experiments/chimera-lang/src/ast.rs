use crate::opcode::OpCode;
use crate::Rule;
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Dna {
    pub helix: Helix,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Helix {
    pub strands: Vec<Rc<Strand>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Strand {
    pub genes: Vec<Gene>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gene {
    pub op: OpCode,
    pub args: Vec<Nucleotide>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum JunctionType {
    Any,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Nucleotide {
    Number(i64),
    String(String),
    #[allow(dead_code)]
    Identifier(String),
    Junction(JunctionType, Vec<Nucleotide>),
}

impl Dna {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::dna => {
                let mut inner = pair.into_inner();
                let helix = Helix::from_pair(inner.next().unwrap());
                Dna { helix }
            }
            _ => panic!("Expected DNA rule"),
        }
    }
}

impl Helix {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::helix => {
                let strands = pair
                    .into_inner()
                    .map(Strand::from_pair)
                    .map(Rc::new)
                    .collect();
                Helix { strands }
            }
            _ => panic!("Expected Helix rule"),
        }
    }
}

impl Strand {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::strand => {
                let genes = pair.into_inner().map(Gene::from_pair).collect();
                Strand { genes }
            }
            _ => panic!("Expected Strand rule"),
        }
    }
}

impl Gene {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::gene => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let op = name.parse().expect("Failed to parse opcode");
                let args_pair = inner.next().unwrap();
                let args = args_pair.into_inner().map(Nucleotide::from_pair).collect();
                Gene { op, args }
            }
            _ => panic!("Expected Gene rule"),
        }
    }
}

impl Nucleotide {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::number => Nucleotide::Number(pair.as_str().parse().unwrap()),
            Rule::string => {
                let s = pair.as_str();
                // Remove quotes
                Nucleotide::String(s[1..s.len() - 1].to_string())
            }
            Rule::identifier => Nucleotide::Identifier(pair.as_str().to_string()),
            Rule::junction => {
                let mut inner = pair.into_inner();
                let type_pair = inner.next().unwrap();
                let j_type = match type_pair.as_str() {
                    "any" => JunctionType::Any,
                    "all" => JunctionType::All,
                    _ => panic!("Unknown junction type"),
                };
                let args_pair = inner.next().unwrap();
                let args = args_pair.into_inner().map(Nucleotide::from_pair).collect();
                Nucleotide::Junction(j_type, args)
            }
            _ => panic!("Expected Nucleotide rule, got {:?}", pair.as_rule()),
        }
    }
}
