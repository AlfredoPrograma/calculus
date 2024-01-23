use core::fmt;
use std::{error::Error, fmt::Debug, mem};

use crate::{
    ast::{expressions::UnaryExpr, helpers::match_token},
    tokenizer::tokens::{Operator, Token},
};

use super::{
    expressions::{BinaryExpr, Expression},
    helpers::{match_concrete_token, peek},
};

#[derive(Debug, Clone)]
pub struct ASTParseError {
    message: &'static str,
}

impl ASTParseError {
    fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl fmt::Display for ASTParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[AST PARSE ERROR]: {}", self.message)
    }
}

impl Error for ASTParseError {}

type ExpressionResult = Result<Expression, ASTParseError>;

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
    pub fn program(&mut self) -> ExpressionResult {
        self.term()
    }

    /// Builds a term.
    ///
    /// Production rule: `Term -> Factor (("+" | "-") Factor)*`
    fn term(&mut self) -> ExpressionResult {
        const TERM_OPERATORS: &[Token] = &[
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
        ];

        let mut binary_expr: Option<Expression> = None;
        let left = self.factor();

        if left.is_err() {
            return Err(left.unwrap_err());
        }

        while let Some(operator) = match_concrete_token(TERM_OPERATORS, &mut self.tokens) {
            let right = self.factor();

            if right.is_err() {
                return Err(right.unwrap_err());
            }

            match binary_expr {
                Some(prev_expr) => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        prev_expr,
                        operator,
                        right.unwrap(),
                    )))
                }
                None => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        left.clone().unwrap(),
                        operator,
                        right.unwrap(),
                    )))
                }
            };
        }

        match binary_expr {
            Some(binary_expr) => Ok(binary_expr),
            None => left,
        }
    }

    /// Builds a factor.
    ///
    /// Production rule: `Factor -> Unary (("*" | "/") Unary)*`
    fn factor(&mut self) -> ExpressionResult {
        const FACTOR_OPERATORS: &[Token] = &[
            Token::Operator(Operator::Star),
            Token::Operator(Operator::Slash),
        ];

        let mut binary_expr: Option<Expression> = None;
        let left = self.unary();

        if left.is_err() {
            return Err(left.unwrap_err());
        }

        while let Some(operator) = match_concrete_token(FACTOR_OPERATORS, &mut self.tokens) {
            let right = self.unary();

            if right.is_err() {
                return Err(right.unwrap_err());
            }

            match binary_expr {
                Some(prev_expr) => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        prev_expr,
                        operator,
                        right.unwrap(),
                    )))
                }
                None => {
                    binary_expr = Some(Expression::Binary(BinaryExpr::new(
                        left.clone().unwrap(),
                        operator.clone(),
                        right.unwrap(),
                    )))
                }
            }
        }

        match binary_expr {
            Some(binary_expr) => Ok(binary_expr),
            None => left,
        }
    }

    /// Builds an unary.
    ///
    /// Production rule: `"-" Literal | Literal`
    fn unary(&mut self) -> ExpressionResult {
        match peek(&mut self.tokens) {
            Some(token) => match token {
                Token::Operator(ref operator) => {
                    if *operator == Operator::Minus {
                        self.tokens.next();
                        let literal = self.literal();

                        if literal.is_err() {
                            return Err(literal.unwrap_err());
                        }

                        return Ok(Expression::Unary(UnaryExpr::new(
                            token.clone(),
                            literal.unwrap(),
                        )));
                    }

                    return Err(ASTParseError::new("syntax error in <unary> expression"));
                }
                Token::Number(_) => self.literal(),
            },
            None => return Err(ASTParseError::new("syntax error by uncomplete expression")),
        }
    }

    /// Builds a literal.
    ///
    /// Literal is a `terminal` symbol, so does not belongs to any production rule
    fn literal(&mut self) -> ExpressionResult {
        if let Some(number) =
            match_token(&[mem::discriminant(&Token::Number(0.0))], &mut self.tokens)
        {
            return Ok(Expression::Literal(number));
        }

        Err(ASTParseError::new("unexpected expression"))
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
            literal_expr.unwrap(),
            expected_expr,
            "should build literal expression for given token"
        );

        assert_eq!(
            parser.tokens.count(),
            0,
            "should consume token from tokens source once expression was built"
        )
    }

    #[test]
    fn test_literal_fails() {
        // Arrange
        let non_literal_token = Token::Operator(Operator::Star);
        let tokens_source = [non_literal_token].into_iter();
        let mut parser = Parser::new(tokens_source);

        // Act
        let result = parser.literal();

        // Assert
        assert!(
            result.is_err(),
            "should return error if cannot parse literal expression"
        )
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
            expected_expr,
            unary_expr.unwrap(),
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
            expected_expr,
            literal_from_unary.unwrap(),
            "should build a literal expression for given tokens stream if operator does not exist"
        )
    }

    #[test]
    fn test_unary_fails_by_invalid_operator() {
        // Arrange
        let non_unary_operator = Token::Operator(Operator::Star);
        let literal_token = Token::Number(99.9);
        let tokens_source = [non_unary_operator.clone(), literal_token.clone()].into_iter();

        let mut parser = Parser::new(tokens_source);

        // Act
        let result = parser.unary();

        // Assert
        assert!(
            result.is_err(),
            "should return error if cannot parse unary expression by invalid operator"
        )
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
                factor_expr.unwrap(),
                expected_expr,
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
                factor_expr.unwrap(),
                expected_expr,
                "should build a binary expression from factor production rule"
            )
        }
    }
}
