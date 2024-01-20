use std::{fmt, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Minus,
    Plus,
    Star,
    Slash,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operator: &str;

        match self {
            Operator::Plus => operator = "+",
            Operator::Minus => operator = "-",
            Operator::Star => operator = "*",
            Operator::Slash => operator = "/",
        }

        write!(f, "{operator}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(number) => write!(f, "{}", number),
            Token::Operator(operator) => write!(f, "{}", operator),
        }
    }
}

fn peek(chars: &Chars) -> Option<char> {
    chars.clone().next()
}

fn parse_operator<'a>(chars: &'a mut Chars) -> Result<Option<Token>, &'a str> {
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

fn parse_number<'a>(chars: &'a mut Chars) -> Result<Option<Token>, &'a str> {
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
