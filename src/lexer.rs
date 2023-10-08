use crate::*;
use std::{io::Read, vec};

// This will lexically analyze a nachobasic file into basic tokens.
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
        let mut index = 0;
        let mut line_token_list: Vec<Token> = vec![];

        while index < line.len() {
            let c = line.chars().nth(index).unwrap();
            let try_read_char_token = try_read_char_token(&mut index, c);
            let new_token = match try_read_char_token {
                Token::Error => try_read_multichar_token(&mut index, line),
                _ => try_read_char_token,
            };

            // println!("{:?}", new_token);
            if let Token::Error = new_token {
                return tokens;
            }

            if new_token != Token::Ignored {
                line_token_list.push(new_token);
            }
        }
        tokens.push(line_token_list);
    }
    tokens
}

/// This will read a single or multicharacter string slice and return it.
/// If this is not starting on a alphabetic character then this will panic.
fn read_alpha_numeric_token_string<'a>(index: &mut usize, line: &'a str) -> &'a str {
    let starting_index = *index;
    let mut ending_index = *index;
    for c in line[starting_index..line.len()].chars() {
        if c.is_alphanumeric() || c == '_' {
            ending_index += 1
        } else {
            break;
        }
    }
    *index = ending_index;
    &line[starting_index..ending_index]
}

/// This will try reading a multicharacter token and return it.
/// This will try to read a keyword or an identifier first, then it will try a int/float literal then finally it will try a string literal.
/// If none of the above are found an error token will be returned
fn try_read_multichar_token(index: &mut usize, line: &str) -> Token {
    let c = line.chars().nth(*index).unwrap();
    if c.is_alphabetic() {
        let token_string = read_alpha_numeric_token_string(index, line);
        let try_read_keyword = try_read_keyword_token(token_string);
        match try_read_keyword {
            Token::Error => Token::Identifier(token_string.to_string()),
            _ => try_read_keyword,
        }
    } else if c.is_numeric() || c == '.' {
        try_read_number_literal_token(index, line)
    } else if c == '"' {
        try_read_string_literal_token(index, line)
    } else {
        Token::Error
    }
}

// This will try reading an individual char token.
// This will check for operator tokens, an argument separator token, block tokens, and ignored tokens.
// If none of the above are found this will return an error token.
fn try_read_char_token(index: &mut usize, c: char) -> Token {
    let token = match c {
        // Ignored Tokens
        ' ' | '\n' | '\r' | '\t' => Token::Ignored,

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
    if token != Token::Error {
        *index += 1;
    }
    token
}

// This will try reading a keyword token.
// This will try reading basic type tokens (struct and array types will be parsed in a later step), boolean literal tokens, control keywords, and the struct keyword.
// If none of the above are found return an error token.
fn try_read_keyword_token(string: &str) -> Token {
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

        _ => Token::Error,
    }
}

// This will read a string literal token.
// This may panic if another double quote is not found to enclose the string literal.
fn try_read_string_literal_token(index: &mut usize, line: &str) -> Token {
    *index += 1;
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

// This will read an int or float literal token depending on if there is a decimal present.
// If neither a float or a string could be parsed this will return an error token.
fn try_read_number_literal_token(index: &mut usize, line: &str) -> Token {
    let starting_index = *index;
    let mut ending_index = *index;

    let number_literal_string = if (starting_index..line.len() - 1).len() != 0 {
        for c in line[starting_index..line.len()].chars() {
            if c.is_numeric() || c == '.' {
                ending_index += 1
            } else {
                break;
            }
        }
        line[starting_index..ending_index].to_string()
    } else {
        line.chars().nth(ending_index).unwrap().to_string()
    };

    *index = ending_index + 1;

    if number_literal_string.contains(".") {
        if let Ok(float) = number_literal_string.parse() {
            Token::FloatLiteral(OrderedFloat(float))
        } else {
            Token::Error
        }
    } else {
        if let Ok(int) = number_literal_string.parse() {
            Token::IntLiteral(int)
        } else {
            Token::Error
        }
    }
}
