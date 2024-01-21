use std::str::Chars;

use crate::tokenizer::helpers::{parse_number, parse_operator};

use super::tokens::Token;

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

    fn scan_token(&mut self) {
        let parsers = [parse_number, parse_operator];

        for p in parsers {
            // Check if token parsing was successful
            if let Ok(result) = p(&mut self.chars) {
                // If result returns a token, push it in the `Tokens` register
                if let Some(token) = result {
                    self.tokens.push(token);
                }

                // Since parse was successful, early returns breaking for loop and avoiding below `panic!`
                return;
            }
        }

        panic!("unexpected token")
    }

    pub fn tokenize(&mut self) {
        while !self.is_end() {
            self.scan_token()
        }
    }
}
