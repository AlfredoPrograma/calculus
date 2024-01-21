use std::{fmt::Debug, mem};

use crate::{
    ast::{expressions::UnaryExpr, helpers::match_token},
    tokenizer::tokens::{Operator, Token},
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
    /// Production rule: `Expression -> Factor (("+" | "-") Factor)*`
    fn expression(&mut self) -> Expression {
        const TERM_OPERATORS: &[Token] = &[
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
        ];

        let mut binary_expr: Option<Expression> = None;
        let left = self.factor();

        while let Some(operator) = match_concrete_token(TERM_OPERATORS, &mut self.tokens) {
            let right = self.factor();

            match binary_expr {
                Some(prev_expr) => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        prev_expr, operator, right,
                    )))
                }
                None => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        left.clone(),
                        operator,
                        right,
                    )))
                }
            };
        }

        match binary_expr {
            Some(binary_expr) => binary_expr,
            None => left,
        }
    }

    /// Builds a factor.
    ///
    /// Production rule: `Factor -> Unary (("*" | "/") Unary)*`
    fn factor(&mut self) -> Expression {
        const FACTOR_OPERATORS: &[Token] = &[
            Token::Operator(Operator::Star),
            Token::Operator(Operator::Slash),
        ];

        let mut binary_expr: Option<Expression> = None;
        let left = self.unary();

        while let Some(operator) = match_concrete_token(FACTOR_OPERATORS, &mut self.tokens) {
            let right = self.unary();

            match binary_expr {
                Some(prev_expr) => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        prev_expr, operator, right,
                    )))
                }
                None => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        left.clone(),
                        operator.clone(),
                        right,
                    )))
                }
            }
        }

        match binary_expr {
            Some(binary_expr) => binary_expr,
            None => left,
        }
    }

    /// Builds an unary.
    ///
    /// Production rule: `"-" Literal`
    fn unary(&mut self) -> Expression {
        match peek(&mut self.tokens) {
            Some(token) => match token {
                Token::Operator(ref operator) => {
                    if *operator == Operator::Minus {
                        self.tokens.next();
                        let literal = self.literal();

                        return Expression::Unary(UnaryExpr::new(token.clone(), literal));
                    }

                    panic!("unexpected token from unary")
                }
                Token::Number(_) => self.literal(),
            },
            None => panic!("end of input"),
        }
    }

    /// Builds a literal.
    ///
    /// Literal is a `terminal` symbol, so does not belongs to any production rule
    fn literal(&mut self) -> Expression {
        if let Some(number) =
            match_token(&[mem::discriminant(&Token::Number(0.0))], &mut self.tokens)
        {
            return Expression::Literal(number);
        }

        panic!("unexpected token")
    }
}
