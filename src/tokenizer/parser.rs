use core::fmt;
use std::{error::Error, str::Chars};

use crate::tokenizer::helpers::{parse_number, parse_operator};

use super::tokens::Token;

#[derive(Debug)]
pub struct TokenizerError {
    message: &'static str,
}

impl TokenizerError {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[TOKENIZER ERROR]: {}", self.message)
    }
}

impl Error for TokenizerError {}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    pub tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            tokens: Vec::new(),
        }
    }

    fn is_end(&self) -> bool {
        self.chars.clone().count() == 0
    }

    fn scan_token(&mut self) -> Result<(), TokenizerError> {
        let parsers = [parse_number, parse_operator];

        for p in parsers {
            // Check if token parsing was successful
            if let Ok(result) = p(&mut self.chars) {
                // If result returns a token, push it in the `Tokens` register
                if let Some(token) = result {
                    self.tokens.push(token);
                }

                // Since parse was successful, early returns breaking for loop and avoiding below `panic!`
                return Ok(());
            }
        }

        Err(TokenizerError::new("unexpected token"))
    }

    pub fn tokenize(&mut self) -> Result<(), TokenizerError> {
        while !self.is_end() {
            if let Err(err) = self.scan_token() {
                return Err(err);
            }
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tokenizer_parser_tests {
    use crate::tokenizer::tokens::{Operator, Token};

    use super::Tokenizer;

    const SOURCE: &str = "3 + 4.33 / 5";

    #[test]
    fn test_new_tokenizer() {
        // Act
        let tokenizer = Tokenizer::new(SOURCE);

        // Assert
        assert!(
            tokenizer.tokens.is_empty(),
            "tokenizer should be instantiated with an empty array of tokens"
        );

        assert_eq!(
            tokenizer.chars.count(),
            SOURCE.chars().count(),
            "tokenizer should be instantiated with an stream of characters based on the source"
        )
    }

    #[test]
    fn test_is_end() {
        // Arrange
        let tokenizer = Tokenizer::new(SOURCE);
        let empty_tokenizer = Tokenizer::new("");

        // Act & Assert
        assert!(
            !tokenizer.is_end(),
            "should return false if tokenizer characters were not consumed fully yet "
        );

        assert!(
            empty_tokenizer.is_end(),
            "should return true if tokenizer characters were consumed all"
        )
    }

    #[test]
    fn test_tokenize_success() {
        // Arrange
        let mut tokenizer = Tokenizer::new(SOURCE);

        // `expected_tokens` vector is based on the `SOURCE` input.
        // Any change on some of them should be reflected in the other in order to keep sync
        // the input and the expected set of tokens
        let expected_tokens = vec![
            Token::Number(3.0),
            Token::Operator(Operator::Plus),
            Token::Number(4.33),
            Token::Operator(Operator::Slash),
            Token::Number(5.0),
        ];

        // Act
        tokenizer.tokenize().unwrap();

        // Assert
        assert_eq!(
            tokenizer.tokens, expected_tokens,
            "should take source characters stream and convert it into a stream of tokens"
        );

        assert_eq!(
            tokenizer.chars.count(),
            0,
            "once all characters are converte into tokens, iterator should consume items and should be empty"
        )
    }

    #[test]
    fn test_tokenize_fails() {
        // Arrange
        const INVALID_SOURCE: &str = "invalid source";
        let mut tokenizer = Tokenizer::new(INVALID_SOURCE);

        // Act & Assert
        assert!(
            tokenizer.tokenize().is_err(),
            "should return error if given source cannot be parsed"
        )
    }
}
