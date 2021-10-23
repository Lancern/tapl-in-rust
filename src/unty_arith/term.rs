use std::fmt::{Display, Formatter};

use crate::term::Term as AbstractTerm;

#[derive(Clone, Debug)]
pub enum Term {
    True,
    False,
    Zero,
    IfElse {
        condition: Box<Term>,
        true_br: Box<Term>,
        false_br: Box<Term>,
    },
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

impl Term {
    pub fn is_value(&self) -> bool {
        self.is_bool_value() || self.is_numeric_value()
    }

    pub fn is_bool_value(&self) -> bool {
        match self {
            Self::True | Self::False => true,
            _ => false,
        }
    }

    pub fn is_numeric_value(&self) -> bool {
        match self {
            Self::Zero => true,
            Self::Succ(t) => t.is_numeric_value(),
            _ => false,
        }
    }
}

impl AbstractTerm for Term {}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Zero => f.write_str("0"),
            Self::IfElse {
                condition,
                true_br,
                false_br,
            } => f.write_fmt(format_args!(
                "if {} then {} else {}",
                condition, true_br, false_br
            )),
            Self::Succ(t) => f.write_fmt(format_args!("succ {}", t)),
            Self::Pred(t) => f.write_fmt(format_args!("pred {}", t)),
            Self::IsZero(t) => f.write_fmt(format_args!("iszero {}", t)),
        }
    }
}
