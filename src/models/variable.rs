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
impl Variable {
    pub fn allocate_int() -> Self { Self::Int(i32::default()) }

    pub fn allocate_float() -> Self { Self::Float(OrderedFloat(f32::default())) }

    pub fn allocate_bool() -> Self { Self::Bool(bool::default()) }

    pub fn allocate_string() -> Self { Self::String(String::default()) }

    pub fn allocate_array(array_type : ArrayType) -> Self { 
        Self::Array(Array { array_type : array_type, array : vec![] })
    }

    pub fn allocate_blank_struct(struct_type : String) -> Self {
        Self::Struct(Struct { struct_type: struct_type, map: HashMap::new() }) 
    }

    // this will clone a struct from a struct type frolm a type registry
    pub fn allocate_struct_from_registry(struct_type : &str, struct_type_registry : HashMap<&str, Variable>) -> Result<Self, String> {
        if struct_type_registry.contains_key(struct_type) {
            Ok(struct_type_registry[struct_type].clone())
        } else {
            Err(format!("error : no type of name \"{}\" contained within the type registry", struct_type))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct {
    pub struct_type : String,
    pub map : HashMap<String, Variable> 
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array {
    pub array_type : ArrayType,
    pub array : Vec<Variable>
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
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_string = match self {
            Self::Void => "Void".to_string(),
            Self::Int => "Int".to_string(),
            Self::Float => "Float".to_string(),
            Self::Bool => "Bool".to_string(),
            Self::String => "String".to_string(),
            Self::Array(nb_array_type) => format!("Array[{nb_array_type}]"),
            Self::Struct(nb_struct_type) => format!("Struct[{nb_struct_type}]")
        };
        write!(f, "{}", type_string)
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
        let nb_type_str = match self {
            Self::Int => "Int",
            Self::Float => "Float",
            Self::Bool => "Bool",
            Self::String => "String",
            Self::Struct(string) => string.as_str()
        };
        write!(f, "Type : {}", nb_type_str)
    }
}