use std::fmt::Display;

use rust_decimal::Decimal;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Pow, 
    LParen,
    RParen,
    Number(Decimal),
    EOF
}

impl Token {
    pub fn get_precedence(&self) -> Precedence {
        match self {
            Token::Add | Token::Sub => Precedence::AddSub,
            Token::Mul | Token::Div => Precedence::MulDiv,
            Token::Pow => Precedence::Pow,
            _ => Precedence::Default,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Pow => write!(f, "^"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Number(num) => write!(f, "{}", num),
            Token::EOF => write!(f, "EOF"),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    Default,
    AddSub,
    MulDiv,
    Pow,
    Negative,
}