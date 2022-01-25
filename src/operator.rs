use std::fmt::{Display, Formatter};
use crate::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Operator {
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

    pub(crate) fn can_apply(&self) -> bool {
        match self {
            Operator::LeftParen | Operator::RightParen | Operator::Assign => false,
            _ => true,
        }
    }

    pub(crate) fn apply(&self, left: f64, right: f64) -> Result<f64, Error> {
        Ok(match self {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => {
                if right == 0.0 {
                    return Err(Error::DivByZero);
                }
                left / right
            },
            Operator::Mod => left % right,
            Operator::Pow => {
                if right < 0.0 {
                    return Err(Error::NegativeExponent);
                }
                left.powf(right)
            }
            _ => panic!("Operator::apply() called on non-operator"),
        })
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