use rust_decimal::Decimal;

use crate::calc::error::CalcResult;

mod token;
mod tokennizer;
mod ast;
mod error;
mod parser;

pub fn calculate(expr: &str) -> CalcResult<Decimal> {
    let mut parser = parser::Parser::new(expr)?;
    let ast = parser.parse()?;
    Ok(ast.eval())
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;
    #[test]
    fn test_calculate() {
        assert_eq!(calculate("1+2"), Ok(dec!(3)));
    }
    #[test]
    fn test_calculate_with_error_2() {
        assert_eq!(calculate("1+2*3"), Ok(dec!(7)));
    }
    
}