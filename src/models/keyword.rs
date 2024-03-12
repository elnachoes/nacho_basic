use super::Token;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    Void,
    Int,
    Bool,
    Float,
    String,
    Array,
    True,
    False,
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    For,
    Fn,
    Return,
    Struct,
}
impl TryFrom<&str> for Keyword {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let token = match value {
            // Type Keywords
            "void"      => Self::Void,
            "int"       => Self::Int,
            "float"     => Self::Float,
            "bool"      => Self::Bool,
            "string"    => Self::String,
            "array"     => Self::Array,
            "false"     => Self::False,
            "true"      => Self::True,

            // Control Keywords
            "if"        => Self::If,
            "elif"      => Self::Elif,
            "else"      => Self::Else,
            "while"     => Self::While,
            "for"       => Self::For,
            "break"     => Self::Break,
            "continue"  => Self::Continue,
            "return"    => Self::Return,
            "fn"        => Self::Fn,

            // Struct Keyword
            "struct"    => Self::Struct,

            _ => return Err(()),
        };

        Ok(token)
    }
}
impl TryFrom<&Token> for Keyword {
    type Error = ();
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        if let Token::Keyword(keyword) = value {
            Ok(*keyword)
        } else {
            Err(())
        }
    }
}