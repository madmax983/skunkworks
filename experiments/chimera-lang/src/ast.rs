use crate::opcode::OpCode;
use crate::Rule;
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct Dna {
    pub helix: Helix,
}

#[derive(Debug, Clone)]
pub struct Helix {
    pub strands: Vec<Strand>,
}

#[derive(Debug, Clone)]
pub struct Strand {
    pub genes: Vec<Gene>,
}

#[derive(Debug, Clone)]
pub struct Gene {
    pub op: OpCode,
    pub args: Vec<Nucleotide>,
}

#[derive(Debug, Clone)]
pub enum Nucleotide {
    Number(i64),
    String(String),
    #[allow(dead_code)]
    Identifier(String),
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
                let strands = pair.into_inner().map(Strand::from_pair).collect();
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
            _ => panic!("Expected Nucleotide rule, got {:?}", pair.as_rule()),
        }
    }
}
