use super::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Char {
    Ignored,
    EndOfStatement,
    ArgumentSeparator,
    OpenBlockLimiter,
    CloseBlockLimiter,
    OpenParameterLimiter,
    CloseParameterLimiter,
    OpenArrayLimiter,
    CloseArrayLimiter,
    InvertSign,
    AndSign,
    OrSign,
    GreaterThanSign,
    LessThanSign,
    AddSign,
    SubSign,
    MulSign,
    DivSign,
    ModSign,
    EqSign,
}
impl TryFrom<char> for Char {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let char = match value {
            // Ignored Char Tokens
            ' ' | '\r' | '\t' => Self::Ignored,

            '\n' => Self::EndOfStatement,

            // Argument Separator Token
            ',' => Self::ArgumentSeparator,

            // Operator Tokens
            '=' => Self::EqSign,
            '+' => Self::AddSign,
            '-' => Self::SubSign,
            '*' => Self::MulSign,
            '/' => Self::DivSign,
            '%' => Self::ModSign,
            '!' => Self::InvertSign,
            '&' => Self::AndSign,
            '|' => Self::OrSign,
            '<' => Self::LessThanSign,
            '>' => Self::GreaterThanSign,

            // Block Tokens
            '{' => Self::OpenBlockLimiter,
            '}' => Self::CloseBlockLimiter,
            '(' => Self::OpenParameterLimiter,
            ')' => Self::CloseParameterLimiter,
            '[' => Self::OpenArrayLimiter,
            ']' => Self::CloseArrayLimiter,

            _ => return Err(()),
        };

        Ok(char)
    }
}
impl TryFrom<&Token> for Char {
    type Error = ();
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        if let Token::Character(char_token) = value {
            Ok(*char_token)
        } else {
            Err(())
        }
    }
}