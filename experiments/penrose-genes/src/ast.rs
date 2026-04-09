use crate::Rule;
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct Dna {
    pub strands: Vec<Strand>,
}

#[derive(Debug, Clone)]
pub struct Strand {
    pub genes: Vec<Gene>,
}

#[derive(Debug, Clone)]
pub struct Gene {
    pub name: String,
    pub args: Vec<Nucleotide>,
}

#[derive(Debug, Clone)]
pub enum Nucleotide {
    Number(i64),
    #[allow(dead_code)]
    String(String),
    #[allow(dead_code)]
    Identifier(String),
}

impl Dna {
    #[allow(dead_code)]
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::dna => {
                let strands = pair
                    .into_inner()
                    .filter(|p| p.as_rule() == Rule::strand)
                    .map(Strand::from_pair)
                    .collect();
                Dna { strands }
            }
            _ => panic!("Expected DNA rule"),
        }
    }
}

impl Strand {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::gene => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let args_pair = inner.next().unwrap();
                let args = args_pair.into_inner().map(Nucleotide::from_pair).collect();
                Gene { name, args }
            }
            _ => panic!("Expected Gene rule"),
        }
    }
}

impl Nucleotide {
    #[allow(dead_code)]
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
