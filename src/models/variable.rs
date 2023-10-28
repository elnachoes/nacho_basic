use std::collections::HashMap;
use ordered_float::OrderedFloat;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Variable {
    Int(i32),
    Float(OrderedFloat<f32>),
    Bool(bool),
    String(String),
    Array(Array),
    Struct(Struct)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct {
    pub identifier : String,
    pub map : HashMap<String, Variable>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array {
    pub list : Vec<Variable>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Void,
    Int,
    Float,
    Bool,
    String,
    Array(ArrayType),
    Struct(String)
}
impl Type {
    pub fn to_array_type(&self) -> Option<ArrayType> {
        match self {
            Self::Int => Some(ArrayType::Int),
            Self::Float => Some(ArrayType::Float),
            Self::Bool => Some(ArrayType::Bool),
            Self::String => Some(ArrayType::String),
            Self::Struct(string) => Some(ArrayType::Struct(string.clone())),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArrayType {
    Int,
    Float,
    Bool,
    String,
    Struct(String)
}
impl ArrayType {
    pub fn to_type(&self) -> Type {
        match self {
            Self::Int => Type::Int,
            Self::Float => Type::Float,
            Self::Bool => Type::Bool,
            Self::String => Type::String,
            Self::Struct(string) => Type::Struct(string.clone())
        }
    }
}
impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nb_type_string = match self {
            Self::Int => "Int",
            Self::Float => "Float",
            Self::Bool => "Bool",
            Self::String => "String",
            Self::Struct(string) => string.as_str()
        };

        write!(f, "Type : {}", nb_type_string)
    }
}