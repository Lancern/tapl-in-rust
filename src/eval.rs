use std::borrow::{Borrow, BorrowMut};
use std::error::Error;

use crate::term::Term;

/// An evaluator evaluates a term into the corresponding normal form.
pub trait Evaluator<T: Term> {
    /// The error type that can be produced during evaluation.
    type Err: Error;

    /// Evaluate the given term one step towards its normal form.
    fn eval_one_step(&mut self, t: T) -> Result<EvaluationStep<T>, Self::Err>;

    /// Evaluate the given term one or more steps until its normal form.
    fn eval(&mut self, mut t: T) -> Result<T, Self::Err> {
        let nf = loop {
            match self.eval_one_step(t)? {
                EvaluationStep::Step(st) => t = st,
                EvaluationStep::Halt(nf) => break nf,
            };
        };
        Ok(nf)
    }
}

/// The result of evaluating a term in one step.
pub enum EvaluationStep<T> {
    /// The input term is reduced toward its normal form.
    Step(T),

    /// The input term is in its normal form and cannot be reduced further.
    Halt(T),
}

impl<T> EvaluationStep<T> {
    /// Determine whether this `EvaluationStep` is a `Step`.
    pub fn is_step(&self) -> bool {
        match self {
            Self::Step(_) => true,
            _ => false,
        }
    }

    /// Determine whether this `EvaluationStep` is a `Halt`.
    pub fn is_halt(&self) -> bool {
        match self {
            Self::Halt(_) => true,
            _ => false,
        }
    }

    /// Get an immutable reference to the term in the evaluation result.
    pub fn as_term(&self) -> &T {
        match self {
            Self::Step(t) => t,
            Self::Halt(t) => t,
        }
    }

    /// Get a mutable reference to the term in the evaluation result.
    pub fn as_term_mut(&mut self) -> &mut T {
        match self {
            Self::Step(t) => t,
            Self::Halt(t) => t,
        }
    }

    /// Consume the `EvaluationResult` value and get the term contained in it.
    pub fn into_term(self) -> T {
        match self {
            Self::Step(t) => t,
            Self::Halt(t) => t,
        }
    }
}

impl<T> AsMut<T> for EvaluationStep<T> {
    fn as_mut(&mut self) -> &mut T {
        self.as_term_mut()
    }
}

impl<T> AsRef<T> for EvaluationStep<T> {
    fn as_ref(&self) -> &T {
        self.as_term()
    }
}

impl<T> Borrow<T> for EvaluationStep<T> {
    fn borrow(&self) -> &T {
        self.as_term()
    }
}

impl<T> BorrowMut<T> for EvaluationStep<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_term_mut()
    }
}
