use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Ignored,

    // identity
    Identity(String),

    // literal tokens 
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),

    // keywords
    Void,
    Int,
    Bool,
    Float,
    String,
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

    // characters
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

impl TryFrom<&str> for Token {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let token = match value {
            // Type Keywords
            "void"      => Self::Void,
            "int"       => Self::Int,
            "float"     => Self::Float,
            "bool"      => Self::Bool,
            "string"    => Self::String,
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