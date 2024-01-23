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
    /// Production rule: `Program -> (Term)*`
    pub fn program(&mut self) -> Expression {
        self.term()
    }

    /// Builds a term.
    ///
    /// Production rule: `Term -> Factor (("+" | "-") Factor)*`
    fn term(&mut self) -> Expression {
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
    /// Production rule: `"-" Literal | Literal`
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
            None => panic!("uncomplete of input"),
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

#[cfg(test)]
mod ast_parser_tests {

    use crate::{
        ast::expressions::{BinaryExpr, Expression, UnaryExpr},
        tokenizer::tokens::{Operator, Token},
    };

    use super::Parser;

    #[test]
    fn test_literal_success() {
        // Arrange
        let literal_token = Token::Number(10.0);
        let tokens_source = [literal_token.clone()].into_iter();

        let mut parser = Parser::new(tokens_source);
        let expected_expr = Expression::Literal(literal_token);

        // Act
        let literal_expr = parser.literal();

        // Assert
        assert_eq!(
            literal_expr, expected_expr,
            "should build literal expression for given token"
        );

        assert_eq!(
            parser.tokens.count(),
            0,
            "should consume token from tokens source once expression was built"
        )
    }

    #[test]
    #[should_panic]
    fn test_literal_fails() {
        // Arrange
        let non_literal_token = Token::Operator(Operator::Star);
        let tokens_source = [non_literal_token].into_iter();
        let mut parser = Parser::new(tokens_source);

        // Act & Assert
        // Notice we are trying to create a literal from a non literal token (Operator), so function panics.
        parser.literal();
    }

    #[test]
    fn test_unary_with_operator_success() {
        // Arrange
        let operator_token = Token::Operator(Operator::Minus);
        let literal_token = Token::Number(25.5);

        let tokens_source = [operator_token.clone(), literal_token.clone()].into_iter();
        let mut parser = Parser::new(tokens_source);

        let expected_expr = Expression::Unary(UnaryExpr::new(
            operator_token,
            Expression::Literal(literal_token),
        ));

        // Act
        let unary_expr = parser.unary();

        // Assert
        assert_eq!(
            expected_expr, unary_expr,
            "should build unary expression for a valid stream of tokens"
        );
    }

    #[test]
    fn test_unary_for_literal_success() {
        // Arrange
        let literal_token = Token::Number(29.9);
        let tokens_source = [literal_token.clone()].into_iter();

        let mut parser = Parser::new(tokens_source);
        let expected_expr = Expression::Literal(literal_token);

        // Act
        let literal_from_unary = parser.unary();

        // Assert
        assert_eq!(
            expected_expr, literal_from_unary,
            "should build a literal expression for given tokens stream if operator does not exist"
        )
    }

    #[test]
    #[should_panic]
    fn test_unary_fails_by_invalid_operator() {
        // Arrange
        let non_unary_operator = Token::Operator(Operator::Star);
        let literal_token = Token::Number(99.9);
        let tokens_source = [non_unary_operator.clone(), literal_token.clone()].into_iter();

        let mut parser = Parser::new(tokens_source);

        // Act & Assert
        // Notice it should panic because star operator is not a valid operator to perform unary expression building
        parser.unary();
    }

    #[test]
    fn test_factor_success() {
        // Arrange
        let left_literal = Token::Number(10.0);
        let right_literal = Token::Number(20.0);
        let factor_operators = [
            Token::Operator(Operator::Star),
            Token::Operator(Operator::Slash),
        ];

        for operator in factor_operators {
            let tokens_source = [
                left_literal.clone(),
                operator.clone(),
                right_literal.clone(),
            ]
            .into_iter();

            let mut parser = Parser::new(tokens_source);
            let expected_expr = Expression::Binary(BinaryExpr::new(
                Expression::Literal(left_literal.clone()),
                operator,
                Expression::Literal(right_literal.clone()),
            ));

            // Act
            let factor_expr = parser.factor();

            // Assert
            assert_eq!(
                factor_expr, expected_expr,
                "should build a binary expression from factor production rule"
            )
        }
    }

    #[test]
    fn test_expression_success() {
        // Arrange
        let left_literal = Token::Number(10.0);
        let right_literal = Token::Number(20.0);
        let factor_operators = [
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
        ];

        for operator in factor_operators {
            let tokens_source = [
                left_literal.clone(),
                operator.clone(),
                right_literal.clone(),
            ]
            .into_iter();

            let mut parser = Parser::new(tokens_source);
            let expected_expr = Expression::Binary(BinaryExpr::new(
                Expression::Literal(left_literal.clone()),
                operator,
                Expression::Literal(right_literal.clone()),
            ));

            // Act
            let factor_expr = parser.term();

            // Assert
            assert_eq!(
                factor_expr, expected_expr,
                "should build a binary expression from factor production rule"
            )
        }
    }
}
