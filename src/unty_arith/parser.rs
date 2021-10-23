use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::parser::Parser as AbstractParser;
use crate::unty_arith::term::Term;

pub struct Parser;

impl Parser {
    fn parse_from_tokens<I: IntoIterator<Item = Token>>(
        &mut self,
        tokens: I,
    ) -> Result<(Term, I::IntoIter), ParseError> {
        let mut tokens = tokens.into_iter();
        match tokens.next() {
            Some(Token::LitFalse) => Ok((Term::False, tokens)),
            Some(Token::LitTrue) => Ok((Term::True, tokens)),
            Some(Token::LitZero) => Ok((Term::Zero, tokens)),
            Some(Token::KwIf) => {
                let (condition, mut tokens) = self.parse_from_tokens(tokens)?;
                self.expect_token(&mut tokens, Token::KwThen)?;
                let (true_br, mut tokens) = self.parse_from_tokens(tokens)?;
                self.expect_token(&mut tokens, Token::KwElse)?;
                let (false_br, tokens) = self.parse_from_tokens(tokens)?;
                let term = Term::IfElse {
                    condition: Box::new(condition),
                    true_br: Box::new(true_br),
                    false_br: Box::new(false_br),
                };
                Ok((term, tokens))
            }
            Some(Token::KwSucc) => {
                let (nv, tokens) = self.parse_from_tokens(tokens)?;
                Ok((Term::Succ(Box::new(nv)), tokens))
            }
            Some(Token::KwPred) => {
                let (nv, tokens) = self.parse_from_tokens(tokens)?;
                Ok((Term::Pred(Box::new(nv)), tokens))
            }
            Some(Token::KwIsZero) => {
                let (nv, tokens) = self.parse_from_tokens(tokens)?;
                Ok((Term::IsZero(Box::new(nv)), tokens))
            }
            Some(_) => Err(ParseError::new("unexpected token")),
            None => Err(ParseError::new("unexpected end of token stream")),
        }
    }

    fn expect_token<I: Iterator<Item = Token>>(
        &mut self,
        mut tokens: I,
        expected: Token,
    ) -> Result<(), ParseError> {
        match tokens.next() {
            Some(tk) => {
                if tk == expected {
                    Ok(())
                } else {
                    Err(ParseError::new("unexpected token"))
                }
            }
            None => Err(ParseError::new("unexpected end of token stream")),
        }
    }

    fn expect_eot<I: Iterator<Item = Token>>(&mut self, mut tokens: I) -> Result<(), ParseError> {
        if tokens.next().is_none() {
            Ok(())
        } else {
            Err(ParseError::new("end of token stream expected"))
        }
    }
}

impl AbstractParser<Term> for Parser {
    type Err = ParseError;

    fn parse(&mut self, input: &str) -> Result<Term, Self::Err> {
        let mut tokens = Vec::new();
        for token_str in input.split_whitespace() {
            tokens.push(token_str.parse::<Token>()?);
        }

        let (term, iter) = self.parse_from_tokens(tokens)?;
        self.expect_eot(iter)?;

        Ok(term)
    }
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new<M: Into<String>>(message: M) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("parse error: {}", self.message))
    }
}

impl Error for ParseError {}

#[derive(Eq, PartialEq)]
enum Token {
    LitTrue,
    LitFalse,
    LitZero,
    KwIf,
    KwThen,
    KwElse,
    KwSucc,
    KwPred,
    KwIsZero,
}

impl FromStr for Token {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Token::LitTrue),
            "false" => Ok(Token::LitFalse),
            "0" => Ok(Token::LitZero),
            "if" => Ok(Token::KwIf),
            "then" => Ok(Token::KwThen),
            "else" => Ok(Token::KwElse),
            "succ" => Ok(Token::KwSucc),
            "pred" => Ok(Token::KwPred),
            "iszero" => Ok(Token::KwIsZero),
            _ => Err(ParseError::new(format!("unknown token: \"{}\"", s))),
        }
    }
}
