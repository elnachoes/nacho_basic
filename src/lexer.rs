use std::{io::Read, vec};

#[derive(Debug)]
pub enum Token {
    Error,

    Identity(String, IdentityTokenType),

    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    BoolLiteral(bool),

    SpaceSeperator,

    OpenBlockLimiter,
    CloseBlockLimiter,

    OpenParameterLimiter,
    CloseParameterLimiter,

    Operator(OperatorTokenType),

    EndOfStatement,
}

#[derive(Debug)]
pub enum IdentityTokenType {
    Variable,
    Type(Type),
    Function,
}

#[derive(Debug)]
pub enum OperatorTokenType {
    Assignment,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulation,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

enum LexerState {
    Start,
    MultiChar,
    SingleChar,
    Error,
    // AlphaNumericToken,
    // NumericToken,
}
impl LexerState {
    pub fn step_state(&mut self, token : &Token) {
        *self = match token {
            Token::Error => Self::Error,
            Token::Identity(_,_) |
            Token::IntLiteral(_) |
            Token::FloatLiteral(_) |
            Token::StringLiteral(_) | 
            Token::BoolLiteral(_) => Self::SingleChar,
            _ => Self::MultiChar
        }
    }
}

pub fn lexer(file_path: &str) -> Vec<(usize, Vec<Token>)> {
    let mut file_contents = String::default();
    std::fs::File::options()
        .read(true)
        .open(file_path)
        .expect("could not open file")
        .read_to_string(&mut file_contents)
        .expect("could not read file");

    let mut tokens: Vec<(usize, Vec<Token>)> = vec![];

    for (line_number, line) in file_contents.split('\n').enumerate() {
        let mut lex_state = LexerState::Start;
        let mut line_index = 0;
        let mut line_token_list: Vec<Token> = vec![];
        
        while line_index < line.len() {
            let c = line.chars().nth(line_index).unwrap();

            let new_token = match lex_state {
                LexerState::MultiChar | LexerState::Start =>  {
                    read_alpha_numeric_token(&mut line_index, line)
                },
                LexerState::SingleChar => {
                    let try_read_char_token = read_char_token(&mut line_index, c);
                    if let Token::Error = try_read_char_token {
                        read_alpha_numeric_token(&mut line_index, line)
                    } 
                    else { try_read_char_token }
                }
                LexerState::Error => { Token::Error }
            };
            lex_state.step_state(&new_token);
            line_token_list.push(new_token);
            println!("{:?}",line_token_list)
        }

        tokens.push((line_number, line_token_list));        
    }

    tokens
}

pub fn read_alpha_numeric_token(index: &mut usize, line: &str) -> Token {
    let starting_index = *index;
    let mut ending_index = *index;
    for c in line[starting_index..line.len() - 1].chars() {
        if c.is_alphanumeric() {
            ending_index += 1
        } else {
            break;
        }
    }

    *index = ending_index;

    println!("{}..{}", starting_index, ending_index);

    match &line[starting_index..ending_index] {
        "int" => Token::Identity("int".to_string(), IdentityTokenType::Type(Type::Int)),
        "float" => Token::Identity("float".to_string(), IdentityTokenType::Type(Type::Float)),
        "bool" => Token::Identity("bool".to_string(), IdentityTokenType::Type(Type::Bool)),
        "string" => Token::Identity("string".to_string(), IdentityTokenType::Type(Type::String)),
        "fn" => Token::Identity("fn".to_string(), IdentityTokenType::Function),
        "true" => Token::BoolLiteral(true),
        "false" => Token::BoolLiteral(false),
        
        _ => { 
            Token::Identity(
                line[starting_index..ending_index].to_string(),
                IdentityTokenType::Variable
            )
        },
    }
}

pub fn read_char_token(index: &mut usize, c: char) -> Token {
    *index += 1;
    match c {
        ' ' => Token::SpaceSeperator,
        '=' => Token::Operator(OperatorTokenType::Assignment),
        '+' => Token::Operator(OperatorTokenType::Addition),
        '-' => Token::Operator(OperatorTokenType::Subtraction),
        '*' => Token::Operator(OperatorTokenType::Multiplication),
        '/' => Token::Operator(OperatorTokenType::Division),
        '%' => Token::Operator(OperatorTokenType::Modulation),
        '{' => Token::OpenBlockLimiter,
        '}' => Token::CloseBlockLimiter,
        '(' => Token::OpenParameterLimiter,
        ')' => Token::CloseParameterLimiter,
        _ => Token::Error,
    }
}

pub fn read_string_literal_token(index: &mut usize, line: &str) -> Token {
    let starting_index = *index;
    let mut ending_index = *index;
    for c in line[starting_index..line.len() - 1].chars() {
        if c == '"' {
            break;
        }
        ending_index += 1
    }
    let token_string = line[starting_index..ending_index].to_string();
    *index += 2 + token_string.len();
    Token::StringLiteral(token_string)
}

// pub fn read_int_literal

// pub fn read_int_literal_token

// step 1 build the token list

// step 2 validate the tokens and build an ast

// step 3 THEN use it like an actual language and interpret it

