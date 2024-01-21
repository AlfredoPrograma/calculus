use std::str::Chars;

use super::tokens::{Operator, Token};

/// Takes a look at the current element of the iterator without consume it.
pub fn peek(chars: &Chars) -> Option<char> {
    chars.clone().next()
}

pub fn parse_operator<'a>(chars: &'a mut Chars) -> Result<Option<Token>, &'a str> {
    let c = peek(chars);

    let token = match c {
        Some('+') => Ok(Some(Token::Operator(Operator::Plus))),
        Some('-') => Ok(Some(Token::Operator(Operator::Minus))),
        Some('*') => Ok(Some(Token::Operator(Operator::Star))),
        Some('/') => Ok(Some(Token::Operator(Operator::Slash))),
        Some('\n' | ' ') => Ok(None),
        _ => Err("cannot parse operator"),
    };

    // If operator matches, consumes current character from iterator
    if token.is_ok() {
        chars.next();
    }

    token
}

pub fn parse_number<'a>(chars: &'a mut Chars) -> Result<Option<Token>, &'a str> {
    const CANNOT_PARSE_MSG: &'static str = "cannot parse number";
    let mut str_number = String::new();

    while let Some(c) = peek(chars) {
        // If first character is not numeric means parser doesnt match and return `None` immediately
        if !c.is_numeric() && str_number.is_empty() {
            return Err(CANNOT_PARSE_MSG);
        }

        // If current character is `.` so we must check if number string already has a `.`
        if c == '.' {
            // If it has, so it is an invalid number, because only one `.` character is allowed per number
            if str_number.find('.').is_some() {
                return Err(CANNOT_PARSE_MSG);
            }

            chars.next();
            str_number.push(c);
            continue;
        }

        // If some characters already matched but reaches a non numeric character, it means
        // integer number has been ended
        if !c.is_numeric() {
            break;
        }

        // Keep updating iterator status while numeric characters are beign found
        chars.next();
        str_number.push(c);
    }

    let parsed_number = str_number.parse::<f64>().expect(CANNOT_PARSE_MSG);
    let token = Token::Number(parsed_number);

    Ok(Some(token))
}

#[cfg(test)]
mod tokenizer_helpers_tests {
    use crate::tokenizer::{
        helpers::{parse_operator, peek},
        tokens::{Operator, Token},
    };

    use super::parse_number;

    #[test]
    fn test_peek() {
        // Arrange
        const SOURCE: &str = "4*3-2+1";

        for (i, c) in SOURCE.chars().enumerate() {
            // Act
            let peeked = peek(&SOURCE[i..].chars()).unwrap();

            // Assert
            assert_eq!(peeked, c, "should look at the current element of the iterator and return it without consume it")
        }
    }

    #[test]
    fn test_parse_operator_success() {
        // Arrange
        let mut operator_chars = "+-*/  \n".chars();

        // `expected_operator_tokens` slice is based on the `VALID_SOURCE` input.
        // Any change on some of them should be reflected in the other in order to keep sync
        // the input and the expected set of tokens
        let expected_operator_tokens = &[
            Some(Token::Operator(Operator::Plus)),
            Some(Token::Operator(Operator::Minus)),
            Some(Token::Operator(Operator::Star)),
            Some(Token::Operator(Operator::Slash)),
            None, // whitespace
            None, // end of lines
        ];

        for token in expected_operator_tokens {
            // Act
            let parsed = parse_operator(&mut operator_chars).unwrap();

            // Assert
            assert_eq!(
                *token, parsed,
                "should take current character and parse it as operator token"
            )
        }
    }

    #[test]
    fn test_parse_operator_fail() {
        // Arrange
        let mut non_operator_chars = "1<>(invalid".chars();

        // Act
        let result = parse_operator(&mut non_operator_chars);

        // Assert
        assert!(
            result.is_err(),
            "should return error if given character cannot be parsed as operator token"
        );
    }

    #[test]
    fn test_parse_number_success() {
        // Arrange
        let numbers_chars = vec!["10.25".chars(), "5".chars(), "0".chars()];

        // `expected_numbers_tokens` slice is based on the `numbers_chars` input.
        // Any change on some of them should be reflected in the other in order to keep sync
        // the input and the expected set of tokens
        let expected_numbers_tokens = &[
            Some(Token::Number(10.25)),
            Some(Token::Number(5.0)),
            Some(Token::Number(0.0)),
        ];

        for (i, mut number_chars) in numbers_chars.into_iter().enumerate() {
            // Act
            let parsed = parse_number(&mut number_chars);

            // Assert
            assert_eq!(
                expected_numbers_tokens[i],
                parsed.unwrap(),
                "should take given stream of characters and parse it as number token"
            )
        }
    }

    #[test]
    fn test_parse_number_fail() {
        // Arrange
        let invalid_numbers_chars = vec!["not a number".chars(), "3.20.49.9".chars()];

        for mut number_chars in invalid_numbers_chars {
            // Act
            let result = parse_number(&mut number_chars);

            // Assert
            assert!(
                result.is_err(),
                "should return error if given characters stream cannot be parsed as a valid number"
            )
        }
    }
}
