use std::fmt;

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
