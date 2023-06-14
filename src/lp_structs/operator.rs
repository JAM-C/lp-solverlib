use std::fmt::Display;

// Represents the operator in a constraint
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Operator {
    LTE,
    Eq,
    GTE,
}

impl Operator {
    pub fn flip(&self) -> Self {
        match self {
            Operator::LTE => Operator::GTE,
            Operator::Eq => Operator::Eq,
            Operator::GTE => Operator::LTE,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::LTE => write!(f, "<="),
            Operator::Eq => write!(f, "="),
            Operator::GTE => write!(f, ">="),
        }
    }
}