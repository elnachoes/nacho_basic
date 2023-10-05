use std::{io::Read, vec};
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Ignored,
    Identity(String, IdentityTokenType),
    IntLiteral(i32),
    FloatLiteral(OrderedFloat<f32>),
    StringLiteral(String),
    BoolLiteral(bool),
    OpenBlockLimiter,
    CloseBlockLimiter,
    OpenParameterLimiter,
    CloseParameterLimiter,
    Operator(OperatorTokenType),
    Control(ControlTokenType),
    ArgumentSeparator,
    StringLimiter,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentityTokenType {
    Variable,
    Type(Type),
    Function,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorTokenType {
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

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ControlTokenType {
    If,
    Elif,
    Else,
    While,
    For,
    Break,
    Continue,
    Return,
}

// This will lexically analyze a nachobasic file.
pub fn lexer(file_path: &str) -> Vec<Vec<Token>> {
    let mut file_contents = String::default();
    std::fs::File::options()
        .read(true)
        .open(file_path)
        .expect("could not open file")
        .read_to_string(&mut file_contents)
        .expect("could not read file");

    let mut tokens: Vec<Vec<Token>> = vec![];

    for line in file_contents.split('\n') {
        let mut line_index = 0;
        let mut line_token_list: Vec<Token> = vec![];
        
        while line_index < line.len() {
            let c = line.chars().nth(line_index).unwrap();
            let try_read_char_token = read_char_token(&mut line_index, c);
            let new_token = match try_read_char_token {
                Token::Error => if c.is_alphabetic() {
                    read_alpha_numeric_token(&mut line_index, line)
                } else if c.is_numeric() {
                    read_int_literal_token(&mut line_index, line)
                } else {
                    Token::Error
                }
                Token::StringLimiter => read_string_literal_token(&mut line_index, line),
                _ => try_read_char_token
            };

            if new_token != Token::Ignored {
                line_token_list.push(new_token);
            }
        }

        tokens.push(line_token_list);
    }

    tokens
}

// This will read an alphanumeric token. A number cannot be the first character.
fn read_alpha_numeric_token(index: &mut usize, line: &str) -> Token {
    let starting_index = *index;
    let mut ending_index = *index;
    for c in line[starting_index..line.len()].chars() {
        if c.is_alphanumeric() { ending_index += 1 } 
        else { break; }
    }

    *index = ending_index;

    match &line[starting_index..ending_index] {
        // Type Keywords
        "int" => Token::Identity("int".to_string(), IdentityTokenType::Type(Type::Int)),
        "float" => Token::Identity("float".to_string(), IdentityTokenType::Type(Type::Float)),
        "bool" => Token::Identity("bool".to_string(), IdentityTokenType::Type(Type::Bool)),
        "string" => Token::Identity("string".to_string(), IdentityTokenType::Type(Type::String)),

        // Function Keywords 
        "fn" => Token::Identity("fn".to_string(), IdentityTokenType::Function),

        // Bool Literal Keywords
        "true" => Token::BoolLiteral(true),
        "false" => Token::BoolLiteral(false),        
        
        // Control Keywords
        "if" => Token::Control(ControlTokenType::If),
        "elif" => Token::Control(ControlTokenType::Elif),
        "else" => Token::Control(ControlTokenType::Else),
        "while" => Token::Control(ControlTokenType::While),
        "for" => Token::Control(ControlTokenType::For),
        "break" => Token::Control(ControlTokenType::Break),
        "continue" => Token::Control(ControlTokenType::Continue),
        "return" => Token::Control(ControlTokenType::Return),
        
        _ => Token::Identity(line[starting_index..ending_index].to_string(), IdentityTokenType::Variable)
    }
}

// This will read an individual character token.
fn read_char_token(index: &mut usize, c: char) -> Token {
    let token = match c {
        // Ignored Tokens
        ' ' | '\n' | '\r' => Token::Ignored,
        
        // String Literal Limiter Token
        '"' => Token::StringLimiter,

        // Argument Separator Token 
        ',' => Token::ArgumentSeparator,

        // Operator Tokens
        '=' => Token::Operator(OperatorTokenType::Assignment),
        '+' => Token::Operator(OperatorTokenType::Addition),
        '-' => Token::Operator(OperatorTokenType::Subtraction),
        '*' => Token::Operator(OperatorTokenType::Multiplication),
        '/' => Token::Operator(OperatorTokenType::Division),
        '%' => Token::Operator(OperatorTokenType::Modulation),
        '!' => Token::Operator(OperatorTokenType::Not),
        '&' => Token::Operator(OperatorTokenType::And),
        '|' => Token::Operator(OperatorTokenType::Or),
        '<' => Token::Operator(OperatorTokenType::LessThan),
        '>' => Token::Operator(OperatorTokenType::GreaterThan),

        // Block Tokens
        '{' => Token::OpenBlockLimiter,
        '}' => Token::CloseBlockLimiter,
        '(' => Token::OpenParameterLimiter,
        ')' => Token::CloseParameterLimiter,

        _ => Token::Error,
    };
    if token != Token::Error { *index += 1; }
    token
}

// This will read a string literal token.
fn read_string_literal_token(index: &mut usize, line: &str) -> Token {
    let starting_index = *index;
    let mut ending_index = *index;
    for c in line[starting_index..line.len() - 1].chars() {
        if c == '"' { break }
        ending_index += 1
    }
    let token_string = line[starting_index..ending_index].to_string();
    *index += 2 + token_string.len();
    Token::StringLiteral(token_string)
}

// This will read an int literal token.
fn read_int_literal_token(index: &mut usize, line: &str) -> Token {
    let starting_index = *index;
    let mut ending_index = *index;

    let int_literal_string = if (starting_index..line.len() - 1).len() != 0 {
        for c in line[starting_index..line.len()].chars() {
            if c.is_numeric() { ending_index += 1 }
            else { break }
        }
        line[starting_index..ending_index].to_string()
    } else {
        line.chars().nth(ending_index).unwrap().to_string()
    };
    *index = ending_index + 1;

    if let Ok(int) = int_literal_string.parse() {
        Token::IntLiteral(int)
    } else {
        Token::Error
    }
}