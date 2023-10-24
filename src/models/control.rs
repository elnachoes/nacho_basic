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
