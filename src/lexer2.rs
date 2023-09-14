use std::collections::HashMap;

trait TokenLiteralMap<T> {
    fn get_token_literal_map() -> HashMap<T, String>;
}

enum Token {
    // variable identity
    Identity(String),

    // literal declarations
    StringLiteral(String),
    IntLiteral(i32),
    FloatLiteral(f32),
    
    // operators
    Operator(Operator),

    // Declarations
    Type(Type),

    KeyWord(Keyword)
}

#[derive(Hash, PartialEq, Eq)]
enum Keyword {
    If,
    Elif,
    Else,
    While,
    For,
    Function,
}

impl TokenLiteralMap<Keyword> for Keyword {
    fn get_token_literal_map() -> HashMap<Keyword, String> {
        HashMap::from([
            (Keyword::If, String::from("if")),
            (Keyword::Elif, String::from("elif")),
            (Keyword::Else, String::from("else")),
            (Keyword::While, String::from("while")),
            (Keyword::For, String::from("for")),
            (Keyword::Function, String::from("fn")),
        ])
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Type {
    Int,
    Float,
    String,
}

impl TokenLiteralMap<Type> for Type {
    fn get_token_literal_map() -> HashMap<Type, String> {
        HashMap::from([
            (Type::Int, String::from("int")),
            (Type::Float, String::from("float")),
            (Type::String, String::from("string")),
        ])
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide, 
    Mod,
    Assignment
}

impl TokenLiteralMap<Operator> for Operator {
    fn get_token_literal_map() -> HashMap<Operator, String> {
        HashMap::from([
            (Operator::Add, String::from("+")),
            (Operator::Subtract, String::from("-")),
            (Operator::Multiply, String::from("*")),
            (Operator::Divide, String::from("/")),
            (Operator::Mod, String::from("%")),
            (Operator::Assignment, String::from("=")),
        ])
    }
}
