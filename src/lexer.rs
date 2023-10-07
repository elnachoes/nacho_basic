use std::{io::Read, vec};
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Error,
    Ignored,
    Identifier(String),
    Variable(String),
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
    StringLimiter,
    Struct
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Void,
    Int,
    Float,
    String,
    Bool,
    Array(Box<Option<Type>>),
    Struct(String)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Control {
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
                    let token_string = read_alpha_numeric_token_string(&mut line_index, line);
                    let try_read_keyword = read_keyword_token(token_string);
                    match try_read_keyword {
                        Token::Error => Token::Identifier(token_string.to_string()),
                        _ => try_read_keyword
                    }
                } else if c.is_numeric() {
                    read_int_literal_token(&mut line_index, line)
                } else {
                    Token::Error
                }
                Token::StringLimiter => read_string_literal_token(&mut line_index, line),
                _ => try_read_char_token
            };
            
            // println!("{:?}", new_token);
            if new_token != Token::Ignored {
                line_token_list.push(new_token);
            }
        }
        tokens.push(line_token_list);
    }
    tokens
}

// This will read an individual character token.
fn read_char_token(index: &mut usize, c: char) -> Token {
    let token = match c {
        // Ignored Tokens
        ' ' | '\n' | '\r' | '\t' => Token::Ignored,
        
        // String Literal Limiter Token
        '"' => Token::StringLimiter,

        // Argument Separator Token 
        ',' => Token::ArgumentSeparator,

        // Operator Tokens
        '=' => Token::Operator(Operator::Assignment),
        '+' => Token::Operator(Operator::Addition),
        '-' => Token::Operator(Operator::Subtraction),
        '*' => Token::Operator(Operator::Multiplication),
        '/' => Token::Operator(Operator::Division),
        '%' => Token::Operator(Operator::Modulation),
        '!' => Token::Operator(Operator::Not),
        '&' => Token::Operator(Operator::And),
        '|' => Token::Operator(Operator::Or),
        '<' => Token::Operator(Operator::LessThan),
        '>' => Token::Operator(Operator::GreaterThan),

        // Block Tokens
        '{' => Token::OpenBlockLimiter,
        '}' => Token::CloseBlockLimiter,
        '(' => Token::OpenParameterLimiter,
        ')' => Token::CloseParameterLimiter,
        '[' => Token::OpenArrayLimiter,
        ']' => Token::CloseArrayLimiter,

        _ => Token::Error,
    };
    if token != Token::Error { *index += 1; }
    token
}

fn read_alpha_numeric_token_string<'a>(index : &mut usize, line : &'a str) -> &'a str {
    let starting_index = *index;
    let mut ending_index = *index;
    for c in line[starting_index..line.len()].chars() {
        if c.is_alphanumeric() || c == '_' { ending_index += 1 } 
        else { break; }
    }
    *index = ending_index;
    &line[starting_index..ending_index]
}

fn read_keyword_token(string : &str) -> Token {
    match string {
        // Type Keywords
        "void" => Token::Type(Type::Void),
        "int" => Token::Type(Type::Int),
        "float" => Token::Type(Type::Float),
        "bool" => Token::Type(Type::Bool),
        "string" => Token::Type(Type::String),

        // Bool Literal Keywords
        "true" => Token::BoolLiteral(true),
        "false" => Token::BoolLiteral(false),        
        
        // Control Keywords
        "if" => Token::Control(Control::If),
        "elif" => Token::Control(Control::Elif),
        "else" => Token::Control(Control::Else),
        "while" => Token::Control(Control::While),
        "for" => Token::Control(Control::For),
        "break" => Token::Control(Control::Break),
        "continue" => Token::Control(Control::Continue),
        "return" => Token::Control(Control::Return),
        
        // Struct Keyword
        "struct" => Token::Struct,  

        _ => Token::Error
    }
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