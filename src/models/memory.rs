

use std::collections::HashMap;
use ordered_float::OrderedFloat;
use super::{Token, Keyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Variable {
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Bool(bool),
    Array(Vec<Variable>),
    Struct {
        identifier : String,
        map : HashMap<String, Variable>
    }
}

// TODO : we need to create a separate memory allocator for going from type to constructor
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Void,
    Int,
    Float,
    Bool,
    String,
    Array,
    Struct {
        identifier : String
    }
}
impl TryFrom<&Token> for Type {
    type Error = ();
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        let nb_type = match value {
            Token::Keyword(Keyword::Int) => Type::Int,
            Token::Keyword(Keyword::Float) => Type::Float,
            Token::Keyword(Keyword::String) => Type::String,
            Token::Keyword(Keyword::Bool) => Type::Bool,
            Token::Keyword(Keyword::Array) => Type::Array,
            Token::Identity(type_name) => Type::Struct{ identifier : type_name.to_string() },
            _ =>return Err(())
        };

        Ok(nb_type)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ProgramMemoryAddress {
    pub block_address : usize,
    pub expression_address : usize
}

struct StackFrame {
    pub local_variables : Vec<Variable>,
    pub return_type : Type,
    pub return_block_address : ProgramMemoryAddress
}


