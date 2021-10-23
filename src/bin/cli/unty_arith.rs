use tapl::unty_arith::eval::Evaluator;
use tapl::unty_arith::parser::Parser;
use tapl::unty_arith::term::Term;

use crate::Topic as AbstractTopic;

pub struct Topic;

impl AbstractTopic for Topic {
    type Evaluator = Evaluator;
    type Parser = Parser;
    type Term = Term;

    fn create_evaluator() -> Self::Evaluator {
        Evaluator
    }

    fn create_parser() -> Self::Parser {
        Parser
    }
}
