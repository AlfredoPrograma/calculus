use std::str::Chars;

use super::tokens::{Operator, Token};

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
