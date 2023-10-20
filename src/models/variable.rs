use std::collections::HashMap;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Variable {
    Int(i32),
    Float(OrderedFloat<f32>),
    String(String),
    Bool(bool),
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
    pub list : Vec<Type>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Void,
    Int,
    Float,
    String,
    Bool,
    Array(Array),
    Struct(Struct)
}
impl Type {
    pub fn allocate_variable(&self) -> Option<Variable> {
        match self {
            Self::Void => None,
            Self::Int => Some(Variable::Int(0)),
            Self::Float => Some(Variable::Float(OrderedFloat(0.0))),
            Self::Bool => Some(Variable::Bool(false)),
            Self::String => Some(Variable::String("".to_string())),
            Self::Array(array) => Some(Variable::Array( array.clone())),
            Self::Struct(strct) => Some(Variable::Struct(strct.clone())),
        }        
    }
}