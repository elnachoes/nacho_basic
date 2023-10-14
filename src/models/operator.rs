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
