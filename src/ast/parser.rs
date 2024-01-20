#![allow(dead_code)]

use std::{fmt::Debug, mem};

use crate::{
    ast::{expressions::UnaryExpr, helpers::match_token},
    tokenizer::Token,
};

use super::{
    expressions::{BinaryExpr, Expression},
    helpers::{match_concrete_token, peek},
};

/// Stores the state of the tokens stream and exposes methods for perform the AST building
#[derive(Debug)]
pub struct Parser<I: Iterator<Item = Token> + Clone + Debug> {
    pub tokens: I,
}

impl<I: Iterator<Item = Token> + Clone + Debug> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self { tokens: tokens }
    }
}

// Expression methods
impl<I: Iterator<Item = Token> + Clone + Debug> Parser<I> {
    /// Builds the root's program expression.
    ///   
    /// Production rule: `Program -> Expression`
    pub fn program(&mut self) -> Expression {
        self.expression()
    }

    /// Builds an expression.
    ///
    /// Production rule: `Expression -> Term (("+" | "-") Term)*`
    fn expression(&mut self) -> Expression {
        let left = self.term();

        if let Some(operator) = match_concrete_token(
            &[
                Token::Operator("+".to_string()),
                Token::Operator("-".to_string()),
            ],
            &mut self.tokens,
        ) {
            let right = self.expression();

            let expr = BinaryExpr::new(left, operator, right);

            return Expression::Binary(expr);
        }

        left
    }

    /// Builds a term.
    ///
    /// Production rule: `Term -> Unary (("*" | "/") Unary)*`
    fn term(&mut self) -> Expression {
        let left = self.unary();

        if let Some(operator) = match_concrete_token(
            &[
                Token::Operator("*".to_string()),
                Token::Operator("/".to_string()),
            ],
            &mut self.tokens,
        ) {
            let right = self.expression();
            let expr = BinaryExpr::new(left, operator, right);

            return Expression::Binary(expr);
        }

        left
    }

    /// Builds an unary.
    ///
    /// Production rule: `"-" Literal`
    fn unary(&mut self) -> Expression {
        match peek(&mut self.tokens) {
            Some(token) => match token {
                Token::Operator(ref operator) => {
                    if *operator == "-".to_string() {
                        self.tokens.next();
                        let literal = self.literal();

                        return Expression::Unary(UnaryExpr::new(token.clone(), literal));
                    }

                    panic!("unexpected token from unary")
                }
                Token::Integer(_) => self.literal(),
            },
            None => panic!("end of input"),
        }
    }

    /// Builds a literal.
    ///
    /// Literal is a `terminal` symbol, so does not belongs to any production rule
    fn literal(&mut self) -> Expression {
        if let Some(number) =
            match_token(&[mem::discriminant(&Token::Integer(0))], &mut self.tokens)
        {
            return Expression::Literal(number);
        }

        panic!("unexpected token")
    }
}
