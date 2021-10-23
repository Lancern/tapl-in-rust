use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::eval::{EvaluationStep, Evaluator as AbstractEvaluator};
use crate::unty_arith::term::Term;

pub struct Evaluator;

impl Evaluator {
    fn eval_if_else(
        &mut self,
        condition: Box<Term>,
        true_br: Box<Term>,
        false_br: Box<Term>,
    ) -> Result<Term, EvaluationError> {
        // Try to apply rule `E-IfTrue` or rule `E-IfFalse`.
        match &*condition {
            Term::True => return Ok(*true_br),
            Term::False => return Ok(*false_br),
            _ => (),
        };

        // Try to apply rule `E-If`.
        match self.eval_one_step(*condition) {
            Ok(EvaluationStep::Step(t)) => {
                let step = Term::IfElse {
                    condition: Box::new(t),
                    true_br,
                    false_br,
                };
                Ok(step)
            }
            Ok(EvaluationStep::Halt(t)) => Err(EvaluationError::new(format!(
                "{} is a non-boolean normal form but it appears as the condition of a if-else term",
                t
            ))),
            Err(e) => Err(EvaluationError::new(format!(
                "{}\n\tduring evaluation of a if-else term",
                e
            ))),
        }
    }

    fn eval_succ(&mut self, t: Box<Term>) -> Result<Term, EvaluationError> {
        // Try to apply rule `E-Succ`.
        match self.eval_one_step(*t) {
            Ok(EvaluationStep::Step(t)) => Ok(Term::Succ(Box::new(t))),
            Ok(EvaluationStep::Halt(t)) => Err(EvaluationError::new(format!(
                "{} is non-numerical normal form but it appears as the operand of a succ term",
                t
            ))),
            Err(e) => Err(EvaluationError::new(format!(
                "{}\n\tduring evaluation of a succ term",
                e
            ))),
        }
    }

    fn eval_pred(&mut self, t: Box<Term>) -> Result<Term, EvaluationError> {
        // Try apply rule `E-PredZero` or rule `E-PredSucc`.
        let t = match *t {
            Term::Zero => return Ok(Term::Zero),
            Term::Succ(nv) => {
                if nv.is_numeric_value() {
                    return Ok(*nv);
                }
                Term::Succ(nv)
            }
            t => t,
        };

        // Try apply rule `E-Pred`.
        match self.eval_one_step(t) {
            Ok(EvaluationStep::Step(t)) => Ok(Term::Pred(Box::new(t))),
            Ok(EvaluationStep::Halt(t)) => Err(EvaluationError::new(format!(
                "{} is non-numeric normal form but it appears as the opperand of a pred term",
                t
            ))),
            Err(e) => Err(EvaluationError::new(format!(
                "{}\n\tduring the evalution of a pred term",
                e
            ))),
        }
    }

    fn eval_is_zero(&mut self, t: Box<Term>) -> Result<Term, EvaluationError> {
        // Try apply rule `E-IszeroZero` or rule `E-IszeroSucc`
        let t = match *t {
            Term::Zero => return Ok(Term::True),
            Term::Succ(nv) => {
                if nv.is_numeric_value() {
                    return Ok(Term::False);
                }
                Term::Succ(nv)
            }
            t => t,
        };

        // Try apply rule `E-IsZero`
        match self.eval_one_step(t) {
            Ok(EvaluationStep::Step(t)) => Ok(Term::IsZero(Box::new(t))),
            Ok(EvaluationStep::Halt(t)) => Err(EvaluationError::new(format!(
                "{} is non-numeric normal form but it appears as the operand of an iszero term",
                t
            ))),
            Err(e) => Err(EvaluationError::new(format!(
                "{}\n\tduring the evaluation of an iszero term",
                e
            ))),
        }
    }
}

impl AbstractEvaluator<Term> for Evaluator {
    type Err = EvaluationError;

    fn eval_one_step(&mut self, t: Term) -> Result<EvaluationStep<Term>, Self::Err> {
        if t.is_value() {
            return Ok(EvaluationStep::Halt(t));
        }

        match t {
            Term::IfElse {
                condition,
                true_br,
                false_br,
            } => self
                .eval_if_else(condition, true_br, false_br)
                .map(EvaluationStep::Step),
            Term::Succ(t) => self.eval_succ(t).map(EvaluationStep::Step),
            Term::Pred(t) => self.eval_pred(t).map(EvaluationStep::Step),
            Term::IsZero(t) => self.eval_is_zero(t).map(EvaluationStep::Step),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct EvaluationError {
    message: String,
}

impl EvaluationError {
    fn new<M: Into<String>>(message: M) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("eval error: {}", self.message))
    }
}

impl Error for EvaluationError {}
