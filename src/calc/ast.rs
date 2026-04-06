use rust_decimal::{Decimal, MathematicalOps};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(Decimal),
}

impl Node {
    pub fn eval(&self) -> Decimal {
        match self {
            Node::Add(left, right) => {
                let left = left.eval();
                let right = right.eval();
                left + right
            }
            Node::Sub(left, right) => {
                let left = left.eval();
                let right = right.eval();
                left - right
            }
            Node::Mul(left, right) => {
                let left = left.eval();
                let right = right.eval();
                left * right
            }
            Node::Div(left, right) => {
                let left = left.eval();
                let right = right.eval();
                left / right
            }
            Node::Pow(left, right) => {
                let left = left.eval();
                let right = right.eval();
                left.powd(right)
            }
            Node::Negative(node) => {
                let node = node.eval();
                -node
            }
            Node::Number(num) => *num,
            
        }
    }
}

#[cfg(test)]
mod tests { 
    use rust_decimal::dec;

    use super::*;
    #[test]
    fn test_eval() {
        let node = Node::Add(
            Box::new(Node::Number(dec!(1))),
            Box::new(Node::Number(dec!(2))),
        );
        assert_eq!(node.eval(), dec!(3));
    }
    #[test]
    fn test_eval_sub() {
        let node = Node::Sub(
            Box::new(Node::Number(dec!(1))),
            Box::new(Node::Number(dec!(2))),
        );
        assert_eq!(node.eval(), dec!(-1));
    }
    #[test]
    fn test_eval_mul() {
        let node = Node::Mul(
            Box::new(Node::Number(dec!(1))),
            Box::new(Node::Number(dec!(2))),
        );
        assert_eq!(node.eval(), dec!(2));
    }
    #[test]
    fn test_eval_div() {
        let node = Node::Div(
            Box::new(Node::Number(dec!(1))),
            Box::new(Node::Number(dec!(2))),
        );
        assert_eq!(node.eval(), dec!(0.5));
    }
    #[test]
    fn test_eval_pow() {
        let node = Node::Pow(
            Box::new(Node::Number(dec!(2))),
            Box::new(Node::Number(dec!(3))),
        );
        assert_eq!(node.eval(), dec!(8));
    }
    #[test]
    fn test_eval_negative() {
        let node = Node::Negative(Box::new(Node::Number(dec!(1))));
        assert_eq!(node.eval(), dec!(-1));
    }

}