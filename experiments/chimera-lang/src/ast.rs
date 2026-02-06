use crate::opcode::OpCode;
use crate::Rule;
use pest::iterators::Pair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dna {
    pub helix: Helix,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Helix {
    pub strands: Vec<Strand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Strand {
    pub genes: Vec<Gene>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Gene {
    pub op: OpCode,
    pub args: Vec<Nucleotide>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum JunctionType {
    Any,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nucleotide {
    Number(i64),
    String(String),
    #[allow(dead_code)]
    Identifier(String),
    Junction(JunctionType, Vec<Nucleotide>),
}

impl Dna {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::dna => {
                let mut inner = pair.into_inner();
                let helix_pair = inner.next().ok_or("Expected helix in DNA")?;
                let helix = Helix::try_from_pair(helix_pair)?;
                Ok(Dna { helix })
            }
            _ => Err(format!("Expected DNA rule, got {:?}", pair.as_rule())),
        }
    }
}

impl Helix {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::helix => {
                let strands: Result<Vec<Strand>, String> = pair.into_inner().map(Strand::try_from_pair).collect();
                Ok(Helix { strands: strands? })
            }
            _ => Err(format!("Expected Helix rule, got {:?}", pair.as_rule())),
        }
    }
}

impl Strand {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::strand => {
                let genes: Result<Vec<Gene>, String> = pair.into_inner().map(Gene::try_from_pair).collect();
                Ok(Strand { genes: genes? })
            }
            _ => Err(format!("Expected Strand rule, got {:?}", pair.as_rule())),
        }
    }
}

impl Gene {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::gene => {
                let mut inner = pair.into_inner();
                let name = inner.next().ok_or("Expected gene name")?.as_str();
                let op = name.parse().map_err(|_| format!("Failed to parse opcode: {}", name))?;
                let args_pair = inner.next().ok_or("Expected gene args")?;
                let args: Result<Vec<Nucleotide>, String> = args_pair.into_inner().map(Nucleotide::try_from_pair).collect();
                Ok(Gene { op, args: args? })
            }
            _ => Err(format!("Expected Gene rule, got {:?}", pair.as_rule())),
        }
    }
}

impl Nucleotide {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::number => {
                let s = pair.as_str();
                s.parse().map(Nucleotide::Number).map_err(|e| format!("Invalid number '{}': {}", s, e))
            }
            Rule::string => {
                let s = pair.as_str();
                // Remove quotes
                if s.len() < 2 {
                    return Err(format!("Invalid string literal: {}", s));
                }
                Ok(Nucleotide::String(s[1..s.len() - 1].to_string()))
            }
            Rule::identifier => Ok(Nucleotide::Identifier(pair.as_str().to_string())),
            Rule::junction => {
                let mut inner = pair.into_inner();
                let type_pair = inner.next().ok_or("Expected junction type")?;
                let j_type = match type_pair.as_str() {
                    "any" => JunctionType::Any,
                    "all" => JunctionType::All,
                    _ => return Err(format!("Unknown junction type: {}", type_pair.as_str())),
                };
                let args_pair = inner.next().ok_or("Expected junction args")?;
                let args: Result<Vec<Nucleotide>, String> = args_pair.into_inner().map(Nucleotide::try_from_pair).collect();
                Ok(Nucleotide::Junction(j_type, args?))
            }
            _ => Err(format!("Expected Nucleotide rule, got {:?}", pair.as_rule())),
        }
    }
}
