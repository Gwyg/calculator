
use std::{iter::Peekable, str::Chars};
use crate::calc::token::Token;


pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
    reached_end: bool,
    unexpected_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &'a str) -> Self {
        Tokenizer {
            expr: expr.chars().peekable(),
            reached_end: false,
            unexpected_char: None,
        }
    }

    pub fn get_unexpected_char(&self) -> Option<char> {
        self.unexpected_char
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_end {
            return None;
        }
        let next_chr = self.expr.next();
        match next_chr {
            Some(chr) if chr.is_numeric() => {
                let mut number = chr.to_string();
                while let Some(next) = self.expr
                .next_if(|c| c.is_numeric()){
                    number.push(next);
                }

                Some(Token::Number(number.parse().unwrap()))
            }
            Some(chr) if chr.is_whitespace() => {
                while let Some(_) = self.expr
                .next_if(|c| c.is_whitespace()) {}
                self.next()
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Div),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            None => {
                self.reached_end = true;
                self.unexpected_char = next_chr;
                Some(Token::EOF)
            }
            _ => {
                self.reached_end = true;
                self.unexpected_char = next_chr;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests { 
    use rust_decimal::dec;
    use super::*;

    #[test]
    fn test_tokenizer() { 
        let tokenizer = Tokenizer::new("1 + 2 * 3");
        assert_eq!(
            tokenizer.collect::<Vec<Token>>(),
            vec![
                Token::Number(dec!(1)),
                Token::Add,
                Token::Number(dec!(2)),
                Token::Mul,
                Token::Number(dec!(3)),
                Token::EOF
            ]
        )
    }
}
