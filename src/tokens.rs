use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Error,
    Ignored,
    Identifier(String),
    Variable(String, Type),
    Function(String, Type),
    Type(Type),
    IntLiteral(i32),
    FloatLiteral(OrderedFloat<f32>),
    StringLiteral(String),
    BoolLiteral(bool),
    OpenBlockLimiter,
    CloseBlockLimiter,
    OpenParameterLimiter,
    CloseParameterLimiter,
    OpenArrayLimiter,
    CloseArrayLimiter,
    Operator(Operator),
    Control(Control),
    ArgumentSeparator,
    Struct,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Assignment,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulation,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Void,
    Int,
    Float,
    String,
    Bool,
    Array(Box<Option<Type>>),
    Struct(String),
}

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
