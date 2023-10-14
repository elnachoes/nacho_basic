use super::{Control, Operator, Type};
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Error,
    Ignored,
    Identifier(String),
    Variable(String, Type),
    Function(String, Type, Option<Vec<Type>>),
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
