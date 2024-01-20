use std::{fmt, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i32),
    Operator(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(int) => write!(f, "{}", int),
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
        Some('+') => Ok(Some(Token::Operator('+'.to_string()))),
        Some('-') => Ok(Some(Token::Operator('-'.to_string()))),
        Some('*') => Ok(Some(Token::Operator('*'.to_string()))),
        Some('/') => Ok(Some(Token::Operator('/'.to_string()))),
        Some('\n' | ' ') => Ok(None),
        _ => Err("cannot parse operator"),
    };

    // If operator matches, consumes current character from iterator
    if token.is_ok() {
        chars.next();
    }

    token
}

fn parse_integer<'a>(chars: &'a mut Chars) -> Result<Option<Token>, &'a str> {
    let mut str_integer = String::new();

    while let Some(c) = peek(chars) {
        // If first character is not numeric means parser doesnt match and return `None` immediately
        if !c.is_numeric() && str_integer.is_empty() {
            return Err("cannot parse integer");
        }

        // If some characters already matched but reaches a non numeric character, it means
        // integer number has been ended
        if !c.is_numeric() {
            break;
        }

        // Keep updating iterator status while numeric characters are beign found
        chars.next();
        str_integer.push(c);
    }

    let token = Token::Integer(
        str_integer
            .parse::<i32>()
            .expect("cannot parse into integer"),
    );

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
        let parsers = [parse_integer, parse_operator];

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
