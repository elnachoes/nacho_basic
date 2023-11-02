use std::fmt;

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
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operator_str = match self {
            Self::Assignment => "Assignment(=)",
            Self::Addition => "Addition(+)",
            Self::Subtraction => "Subtraction(-)",
            Self::Multiplication => "Multiplication(*)",
            Self::Division => "Division(/)",
            Self::Modulation => "Modulation(%)",
            Self::GreaterThan => "GreaterThan(>)",
            Self::LessThan => "LessThan(<)",
            Self::And => "And(&)",
            Self::Or => "Or(|)",
            Self::Not => "Not(!)",
            _ => ""
        };
        write!(f, "{}", operator_str)
    }
}