use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Control {
    If,
    Elif,
    Else,
    While,
    For,
    Break,
    Continue,
    Return,
}
impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let control_str = match self {
            Self::If => "If",
            Self::Elif => "Elif",
            Self::Else => "Else",
            Self::While => "While",
            Self::For => "For",
            Self::Break => "Break",
            Self::Continue => "Continue",
            Self::Return => "Return",
        };
        write!(f, "{}", control_str)
    }
}