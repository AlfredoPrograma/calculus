#![allow(dead_code)]

use std::mem;

use crate::tokenizer::tokens::Token;

/// Takes a look at the next element of the iterator without consume it.
pub fn peek<I: Iterator<Item = Token> + Clone>(tokens_iter: &I) -> Option<Token> {
    tokens_iter.clone().next()
}

/// Tries to match the given token against some token of the given tokens list **comparing its variant only**.
///
/// If some token matches, consumes the token from the iterator.
pub fn match_token<I: Iterator<Item = Token> + Clone>(
    tokens_to_match: &[mem::Discriminant<Token>],
    tokens_iter: &mut I,
) -> Option<Token> {
    let current_token = peek(tokens_iter);

    if let Some(current) = current_token {
        for token in tokens_to_match {
            if mem::discriminant(&current) == *token {
                return tokens_iter.next();
            }
        }
    }

    None
}

/// Tries to match the given token against some token of the given tokens list **comparing its variant and internal value**.
///
/// If token matches, consumes it from the iterator.
pub fn match_concrete_token<I: Iterator<Item = Token> + Clone>(
    tokens_to_match: &[Token],
    tokens_iter: &mut I,
) -> Option<Token> {
    let current_token = peek(tokens_iter);

    if let Some(current) = current_token {
        for token in tokens_to_match {
            if current == *token {
                return tokens_iter.next();
            }
        }
    }

    None
}

#[cfg(test)]
mod ast_helpers_tests {
    use std::mem;

    use crate::tokenizer::tokens::{Operator, Token};

    use super::{match_token, peek};

    #[test]
    fn test_peek() {
        // Arrange
        let tokens_source: Vec<Token> = vec![Token::Number(10.0)];
        let tokens_iterator = tokens_source.clone().into_iter();

        // Act
        let peeked = peek(&tokens_iterator.clone()).unwrap();

        // Assert
        assert_eq!(
            peeked,
            tokens_source.clone()[0],
            "should take a look at the current element of the iterator"
        );

        assert_eq!(
            tokens_iterator.count(),
            tokens_source.len(),
            "should not consume token from the iterator after peek"
        );
    }

    #[test]
    fn test_match_token_success() {
        // Arrange
        let matching_token = Token::Number(10.0);
        let base_number_token = Token::Number(0.0);
        let mut tokens_source = vec![matching_token.clone()].into_iter();

        // Act
        let matched = match_token(
            // Notice we are trying to match `Token::Number(10.0)` against `Token::Number(0.0)`.
            // It is intentional because `match_token` just compares the enum variant, so we dont care about the internal value of the token.
            &[mem::discriminant(&base_number_token)],
            &mut tokens_source,
        );

        // Assert
        assert!(
            matched.is_some(),
            "should match token and return a fullfilled option"
        );

        assert_eq!(
            mem::discriminant(&matched.unwrap()),
            mem::discriminant(&matching_token),
            "token variants should match without consider their internal values"
        );

        assert_eq!(
            tokens_source.count(),
            0,
            "matching token should consume iterator element"
        )
    }

    #[test]
    fn test_match_token_fails() {
        // Arrange
        let number_token = Token::Number(10.0);
        let operator_token = Token::Operator(Operator::Star);
        let mut tokens_source = vec![number_token].into_iter();

        // Act
        let matched = match_token(
            // Notice in this case we are trying to match `Token::Number(10.0)` against `Token::Operator(Operator::Star)`.
            // Since token's variants are not the same, it shouldn't match
            &[mem::discriminant(&operator_token)],
            &mut tokens_source,
        );

        // Assert
        assert!(
            matched.is_none(),
            "should return none if token variants does not match"
        );

        assert_eq!(
            tokens_source.count(),
            1,
            "non matching tokens shouldn't consume current iterator element"
        )
    }
}
