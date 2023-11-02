use super::{Control, Operator, Type};
use ordered_float::OrderedFloat;
use std::fmt::{self, format};

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
    StructDeclaration,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_string = match self {
            
            Self::Error => "Error".to_string(),

            Self::Ignored => "Ignored".to_string(),

            Self::Identifier(identifier) => format!("Identifier({identifier})"),

            Self::Variable(identifier, nb_type) => format!("Variable({identifier}:{nb_type})"),

            Self::Function(identifier, nb_type, _parameters) => format!("Function({identifier}:{nb_type}:(parameters todo))"),
            
            Self::Type(nb_type) => format!("Type({nb_type})"),

            Self::IntLiteral(int) => format!("IntLiteral({int})"),
            Self::FloatLiteral(float) => format!("FloatLiteral({float})"),
            Self::StringLiteral(string) => format!("StringLiteral({string})"),
            Self::BoolLiteral(boolean) => format!("BoolLiteral({boolean})"),

            Self::OpenBlockLimiter => "OpenBlockLimiter".to_string(),
            Self::CloseBlockLimiter => "CloseBlockLimiter".to_string(),
            Self::OpenParameterLimiter => "OpenParameterLimiter".to_string(),
            Self::CloseParameterLimiter => "CloseParameterLimiter".to_string(),
            Self::OpenArrayLimiter => "OpenArrayLimiter".to_string(),
            Self::CloseArrayLimiter => "CloseArrayLimiter".to_string(),

            Self::Operator(operator) => format!("Operator({operator})"),


            Self::Control(control) => format!("Control({control})"),

            Self::ArgumentSeparator => "ArgumentSeparator(,)".to_string(),
            Self::StructDeclaration => "StructDeclaration".to_string(),
        };
        write!(f, "{}", token_string.as_str())
    }
}