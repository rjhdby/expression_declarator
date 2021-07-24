use crate::token::Token;
use crate::operation::Operation;

pub enum AstNode<T: Clone> {
    Primitive { val: T, token: Token<T> },
    Unary { op: Box<Operation<T>>, p1: Box<AstNode<T>>, token: Token<T> },
    Binary { op: Box<Operation<T>>, p1: Box<AstNode<T>>, p2: Box<AstNode<T>>, token: Token<T> },
}

impl<T: 'static + Clone> Clone for AstNode<T> {
    fn clone(&self) -> Self {
        return match self {
            AstNode::Primitive { val, token } => AstNode::Primitive { val: val.clone(), token: token.clone() },
            AstNode::Unary { op, p1, token } => AstNode::Unary { op: op.clone(), p1: p1.clone(), token: token.clone() },
            AstNode::Binary { op, p1, p2, token } => AstNode::Binary { op: op.clone(), p1: p1.clone(), p2: p2.clone(), token: token.clone() },
        };
    }
}

impl<T: 'static + Clone> AstNode<T> {
    pub fn calculate(&self) -> Result<T, Token<T>> {
        let result = match self {
            AstNode::Primitive { val, .. } => val.clone(),
            AstNode::Unary { op, p1, .. } => (&op.executor)(vec![p1.calculate()?]),
            AstNode::Binary { op, p1, p2, .. } => (&op.executor)(vec![p1.calculate()?, p2.calculate()?]),
        };

        return Result::Ok(result);
    }
}