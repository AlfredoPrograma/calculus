#![allow(dead_code)]

use std::fmt;

use crate::tokenizer::tokens::{Operator, Token};

/// Represents the set of expressions used to build the nodes for the AST.
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod ast_expressions_tests {
    use crate::tokenizer::tokens::{Operator, Token};

    use super::{BinaryExpr, Expression, UnaryExpr};

    const LEFT_NUMBER: f64 = 10.0;
    const RIGHT_NUMBER: f64 = 5.0;

    #[test]
    fn test_binary_expr_eval() {
        // Arrange
        let operators = &[
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Star),
            Token::Operator(Operator::Slash),
        ];

        // `expected_results` are based on the `operators` slice order.
        // If some slice is updated, the other should be updated too in order to keep sync
        // the expected results
        let expected_results = &[
            (LEFT_NUMBER + RIGHT_NUMBER),
            (LEFT_NUMBER - RIGHT_NUMBER),
            (LEFT_NUMBER * RIGHT_NUMBER),
            (LEFT_NUMBER / RIGHT_NUMBER),
        ];

        for (i, op) in operators.into_iter().enumerate() {
            let binary_expr = Expression::Binary(BinaryExpr::new(
                Expression::Literal(Token::Number(LEFT_NUMBER)),
                op.clone(),
                Expression::Literal(Token::Number(RIGHT_NUMBER)),
            ));

            // Act & Assert
            assert_eq!(binary_expr.eval(), expected_results[i], "should evaluate binary expression based on its operator and return the corresponding result")
        }
    }

    #[test]
    fn test_unary_expr_eval() {
        // Notice currently unary expressions just supports `minus` operator in front of the number
        // to negate it. So the test is hardcoded in order to evaluate just this case
        // Once new unary operators were added, this test should be improved to cover all possible cases

        // Arrange
        let operator = Token::Operator(Operator::Minus);
        let unary_expr = Expression::Unary(UnaryExpr::new(
            operator,
            Expression::Literal(Token::Number(LEFT_NUMBER)),
        ));

        // Act & Assert
        assert_eq!(unary_expr.eval(), -LEFT_NUMBER, "should evauluate unary expression based on its operator and return the corresponding result")
    }

    #[test]
    fn test_literal_expr_eval() {
        // Arrange
        let literal_expr = Expression::Literal(Token::Number(LEFT_NUMBER));

        // Act & Assert
        assert_eq!(
            literal_expr.eval(),
            LEFT_NUMBER,
            "should evaluate literal expression and just unwraps its value and return it"
        )
    }
}
