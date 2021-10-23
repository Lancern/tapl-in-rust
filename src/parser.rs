use std::error::Error;

use crate::term::Term;

/// A parser parses an input string into a syntax tree.
pub trait Parser<T: Term> {
    /// The type of parsing errors produced by the parser.
    type Err: Error;

    /// Parse the given input string into a syntax tree.
    fn parse(&mut self, input: &str) -> Result<T, Self::Err>;
}
