use std::fmt::{Display, Formatter};
use crate::{Error, Number};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operator {
    LeftParen,  // (
    RightParen, // )
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // / or ÷
    Mod,        // %
    Assign,     // =
    Pow,        // ^
}

impl Operator {
    /// returns a number from 0 to 2 depending on its precedence, with 3 being the highest
    /// if the operator does not have a precedence, returns None
    pub fn precedence(&self) -> Option<u8> {
        match self {
            Operator::Add | Operator::Sub => Some(0),
            Operator::Mul | Operator::Div | Operator::Mod => Some(1),
            Operator::Pow => Some(2),
            _ => None,
        }
    }

    pub(crate) fn apply(&self, left: Number, right: Number) -> Result<Number, Error> {
        Ok(Number::new(match self {
            Operator::Add => left.as_f64() + right.as_f64(),
            Operator::Sub => left.as_f64() - right.as_f64(),
            Operator::Mul => left.as_f64() * right.as_f64(),
            Operator::Div => left.as_f64() / right.as_f64(),
            Operator::Mod => left.as_f64() % right.as_f64(),
            Operator::Pow => {
                if right.as_f64() < 0.0 {
                    return Err(Error::new_gen("Cannot raise to a negative power!"));
                }
                left.as_f64().powf(right.as_f64())
            }
            _ => panic!("Operator::apply() called on non-operator"),
        }))
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::LeftParen => write!(f, "("),
            Operator::RightParen => write!(f, ")"),
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Mod => write!(f, "%"),
            Operator::Assign => write!(f, "="),
            Operator::Pow => write!(f, "^"),
        }
    }
}