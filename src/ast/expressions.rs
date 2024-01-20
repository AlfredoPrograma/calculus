#![allow(dead_code)]

use std::fmt;

use crate::tokenizer::{Operator, Token};

/// Represents the set of expressions used to build the nodes for the AST.
#[derive(Debug)]
pub enum Expression {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Token),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary(binary) => write!(
                f,
                "({left} {operator} {right})",
                left = binary.left,
                operator = binary.operator,
                right = binary.right
            ),
            Expression::Unary(unary) => write!(
                f,
                "({operator}{expr})",
                operator = unary.operator,
                expr = unary.expr
            ),
            Expression::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl Expression {
    pub fn eval(self) -> f64 {
        match self {
            Expression::Binary(binary) => match binary.operator {
                Token::Operator(operator) => match operator {
                    Operator::Plus => binary.left.eval() + binary.right.eval(),
                    Operator::Minus => binary.left.eval() - binary.right.eval(),
                    Operator::Star => binary.left.eval() * binary.right.eval(),
                    Operator::Slash => binary.left.eval() / binary.right.eval(),
                },
                _ => unreachable!(),
            },
            Expression::Unary(unary) => match unary.operator {
                Token::Operator(operator) => match operator {
                    Operator::Minus => unary.expr.eval() * (-1.0),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            Expression::Literal(number) => match number {
                Token::Number(n) => n,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
}

impl BinaryExpr {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    expr: Box<Expression>,
}

impl UnaryExpr {
    pub fn new(operator: Token, expr: Expression) -> Self {
        Self {
            operator,
            expr: Box::new(expr),
        }
    }
}

pub struct LiteralExpr(Token);
