use ordered_float::OrderedFloat;

use super::{keyword, Char, Keyword, Operator, Type};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Ignored,

    // identity
    Identity(String),

    // literal tokens 
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),

    Keyword(Keyword),

    Character(Char),

    Operator(Operator),

    Type(Type),
}
impl Token {
    pub fn is_ignored(&self) -> bool {
        if let Self::Ignored = self {
            true
        } else {
            false
        }
    }
}
impl TryFrom<char> for Token {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let char_token = Char::try_from(value)?;
        if let Char::Ignored = char_token {
            Ok(Token::Ignored)
        } else {
            Ok(Token::Character(char_token))
        }
    }
}
impl TryFrom<&str> for Token {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Token::Keyword(Keyword::try_from(value)?))
    }
}
impl From<Operator> for Token {
    fn from(value: Operator) -> Self {
        Self::Operator(value)
    }
}
impl From<Keyword> for Token {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}
impl From<Type> for Token {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}