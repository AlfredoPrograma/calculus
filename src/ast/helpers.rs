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
    use crate::tokenizer::tokens::Token;

    use super::peek;

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
}
