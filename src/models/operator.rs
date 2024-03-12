use super::{Char, Token};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Assignment,
    
    Addition,
    AdditionAssignment,
    
    Subtraction,
    SubtractionAssignment,
    
    Multiplication,
    MultiplicationAssignment,
    
    Division,
    DivisionAssignment,

    Modulation,
    
    GreaterThan,
    GreaterThanOrEqualTo,
    
    LessThan,
    LessThanOrEqualTo,
    
    EqualTo,
    
    And,
    Or,
    Not,
}
impl TryFrom<&[Char]> for Operator {
    type Error = ();
    fn try_from(value: &[Char]) -> Result<Self, Self::Error> {
        let operator = match value {
            [Char::EqSign] => Self::Assignment,

            [Char::LessThanSign] => Self::LessThan,
            [Char::LessThanSign, Char::EqSign] => Self::LessThanOrEqualTo,
            
            [Char::GreaterThanSign] => Self::GreaterThan,
            [Char::GreaterThanSign, Char::EqSign] => Self::GreaterThanOrEqualTo,
            
            [Char::AddSign] => Self::Addition,
            [Char::AddSign, Char::EqSign] => Self::AdditionAssignment,
            
            [Char::SubSign] => Self::Subtraction,
            [Char::SubSign, Char::EqSign] => Self::SubtractionAssignment,
            
            [Char::MulSign] => Self::Multiplication,
            [Char::MulSign, Char::EqSign] => Self::MultiplicationAssignment,
            
            [Char::DivSign] => Self::Division,
            [Char::DivSign, Char::EqSign] => Self::DivisionAssignment,

            [Char::ModSign] => Self::Modulation,

            [Char::AndSign] => Self::And,
            [Char::OrSign] => Self::Or,
            [Char::InvertSign] => Self::Not,

            _ => return Err(())
        };

        Ok(operator)
    }
}
impl TryFrom<Token> for Operator {
    type Error = ();
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Token::Operator(operator) = value {
            Ok(operator)
        } else {
            Err(())
        }
    }
}