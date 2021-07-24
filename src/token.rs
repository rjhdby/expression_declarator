use crate::operation::Operation;

pub enum Token<T: Clone> {
    WhiteSpace { pos: usize, val: String },
    Open { pos: usize },
    Close { pos: usize },
    Primitive { pos: usize, val: T, original: String },
    Operation { pos: usize, val: Box<Operation<T>> },
    Unknown { pos: usize, val: String },
}

impl<T: 'static + Clone> Clone for Token<T> {
    fn clone(&self) -> Self {
        return match self {
            Token::WhiteSpace { pos, val } => Token::WhiteSpace { pos: *pos, val: val.clone() },
            Token::Open { pos } => Token::Open { pos: *pos },
            Token::Close { pos } => Token::Close { pos: *pos },
            Token::Primitive { pos, val, original } => Token::Primitive { pos: *pos, val: val.clone(), original: original.clone() },
            Token::Operation { pos, val } => Token::Operation { pos: *pos, val: val.clone() },
            Token::Unknown { pos, val } => Token::Unknown { pos: *pos, val: val.clone() },
        };
    }
}

impl<T: 'static + Clone> Token<T> {
    pub fn get_pos(&self) -> usize {
        return match self {
            Token::WhiteSpace { pos, .. } => *pos,
            Token::Open { pos } => *pos,
            Token::Close { pos } => *pos,
            Token::Primitive { pos, .. } => *pos,
            Token::Operation { pos, .. } => *pos,
            Token::Unknown { pos, .. } => *pos,
        };
    }

    pub fn get_value(&self) -> String {
        return match self {
            Token::WhiteSpace { pos: _pos, val } => val.clone(),
            Token::Open { .. } => "(".to_string(),
            Token::Close { .. } => ")".to_string(),
            Token::Primitive { pos: _pos, val: _val, original } => original.clone(),
            Token::Operation { pos: _pos, val } => val.signature.clone(),
            Token::Unknown { pos: _pos, val } => val.clone()
        };
    }
}

impl<T: 'static + Clone> Token<T> {
    pub fn pretty_print(&self) -> String {
        return match self {
            Token::WhiteSpace { pos, val } => format!("'{}' at position {}", val, pos),
            Token::Open { pos } => format!("'(' at position {}", pos),
            Token::Close { pos } => format!("')' at position {}", pos),
            Token::Primitive { pos, val: _val, original } => format!("'{}' at position {}", original, pos),
            Token::Operation { pos, val } => format!("'{}' at position {}", val.signature.to_string(), pos),
            Token::Unknown { pos, val } => format!("'{}' at position {}", val, pos),
        };
    }
}