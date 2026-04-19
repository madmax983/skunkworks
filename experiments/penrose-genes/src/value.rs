#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Str(String),
}

impl Default for Value {
    fn default() -> Self {
        Value::Int(0)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}
