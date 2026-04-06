use crate::calc::{
    ast::Node,
    error::{CalcError, CalcResult},
    token::{Precedence, Token},
    tokennizer::Tokenizer,
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(expr);
        let current_token = tokenizer
            .next()
            .ok_or_else(|| CalcError::UnexpectedChar(tokenizer.get_unexpected_char().unwrap()))?;
        Ok(Parser {
            tokenizer,
            current_token,
        })
    }

    pub fn parse(&mut self) -> CalcResult<Node> {
        self.parse_expression(Precedence::Default)
    }
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> CalcResult<()> {
        self.current_token = self.tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedChar(self.tokenizer.get_unexpected_char().unwrap())
        })?;
        Ok(())
    }
    fn parse_expression(&mut self, precedence: Precedence) -> CalcResult<Node> {
        let mut expr = self.parse_number_or_expression()?;
        while self.current_token.get_precedence() > precedence {
            expr = self.parse_binary_expression(expr)?;
        }
        Ok(expr)
    }

    fn parse_binary_expression(&mut self, left: Node) -> CalcResult<Node> {
        match self.current_token {
            Token::Add => {
                self.next_token()?;
                let right = self.parse_expression(Precedence::AddSub)?;
                Ok(Node::Add(Box::new(left), Box::new(right)))
            }
            Token::Sub => {
                self.next_token()?;
                let right = self.parse_expression(Precedence::AddSub)?;
                Ok(Node::Sub(Box::new(left), Box::new(right)))
            }
            Token::Div => {
                self.next_token()?;
                let right = self.parse_expression(Precedence::MulDiv)?;
                Ok(Node::Div(Box::new(left), Box::new(right)))
            }
            Token::Mul => {
                self.next_token()?;
                let right = self.parse_expression(Precedence::MulDiv)?;
                Ok(Node::Mul(Box::new(left), Box::new(right)))
            }
            Token::Pow => {
                self.next_token()?;
                let right = self.parse_expression(Precedence::Pow)?;
                Ok(Node::Pow(Box::new(left), Box::new(right)))
            }
            _ => unreachable!("Invalid token"),
        }
    }

    fn parse_number_or_expression(&mut self) -> CalcResult<Node> {
        match self.current_token {
            Token::Number(n) => {
                self.next_token()?;
                Ok(Node::Number(n))
            }
            Token::Sub => {
                self.next_token()?;
                let expr = self.parse_expression(Precedence::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::LParen => {
                self.next_token()?;
                let expr = self.parse_expression(Precedence::Default)?;
                if self.current_token != Token::RParen {
                    if self.current_token == Token::EOF {
                        return Err(CalcError::InvalidOperator(String::from("不完整表达式")));
                    }
                    return Err(CalcError::InvalidOperator(format!(
                        "期望 ')', 但是遇到 '{}'",
                        self.current_token
                    )));
                }
                self.next_token()?;
                Ok(expr)
            }
            _ => {
                if self.current_token == Token::EOF {
                    return Err(CalcError::InvalidOperator(String::from("不完整表达式")));
                }
                return Err(CalcError::InvalidOperator(format!(
                    "期望数字, 但是遇到 '{}'",
                    self.current_token
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;
    use super::*;

    #[test]
    fn test_add_or_mul() {
        let mut parser = Parser::new(" 1  +    2  * 3  ").unwrap();
        assert_eq!(
            parser.parse(),
            Ok(Node::Add(
                Box::new(Node::Number(dec!(1))),
                Box::new(Node::Mul(
                    Box::new(Node::Number(dec!(2))),
                    Box::new(Node::Number(dec!(3)))
                ))
            ))
        )
    }
}
