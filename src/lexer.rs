use std::{io::Read, vec};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use crate::{models::token::*, utilities::dyn_range};

pub fn lexer(file_path : &str) -> Result<Vec<Token>, String> {
    // open the file and read it into a string
    let mut input = String::default();
    std::fs::File::options()
        .read(true)
        .open(file_path)
        .or(Err("could not open file".to_string()))?
        .read_to_string(&mut input)
        .or(Err("could not read file".to_string()))?;

    let mut index = 0;
    let mut tokens: Vec<Token> = vec![];
    while index < input.len() {
        let current_index_character = input.chars().nth(index).unwrap();
        
        // try reading a char token 
        if let Ok(character_token) = Token::try_from(current_index_character) {
            if !character_token.is_ignored() {
                tokens.push(character_token)
            }
            index += 1;
            continue 
        }

        // try reading a keyword, boolean literal or identity token
        if current_index_character.is_alphabetic() {
            let str_to_parse = input
                .char_indices()
                .skip(index)
                .find_or_last(|(_index, c)| !c.is_alphabetic() && *c != '_')
                .map(|(ending_index, _c)| ending_index)
                .or(Some(index))
                .map(|ending_index| &input[dyn_range(index, ending_index)])
                .unwrap();

            // try reading a keyword
            if let Ok(keyword_token) = Token::try_from(str_to_parse) {
                tokens.push(keyword_token);
                index += str_to_parse.len();
                continue
            }
            
            // read an identity
            tokens.push(Token::Identity(str_to_parse.to_string()));
            index += str_to_parse.len();
            continue
        }

        // try reading a string literal
        if current_index_character == '"' {
            let ending_quote_index = input
                .char_indices()
                .skip(index + 1)
                .find(|(_index, c)| *c == '"')
                .ok_or("missing ending string litteral quote".to_string())?
                .0;
            
            tokens.push(Token::StringLiteral(input[index + 1..ending_quote_index].to_string()));
            index = ending_quote_index + 1;
            continue
        }

        // try reading a number type literal
        if current_index_character.is_numeric() || current_index_character == '.' {
            let str_to_parse = input
                .char_indices()
                .skip(index)
                .find_or_last(|(_index, c)| !c.is_numeric() && *c != '.')
                .map(|(ending_index, _c)| ending_index)
                .or(Some(index))
                .map(|ending_index| &input[dyn_range(index, ending_index)])
                .unwrap();

            // if there is a decimal point in the number it will be read as a float literal otherwise read an int literal
            let new_number_token = if str_to_parse.contains('.') {
                let float_literal : f64 = str_to_parse.parse().or(Err("could not parse float literal".to_string()))?;
                Token::FloatLiteral(OrderedFloat::from(float_literal))
            } else {
                let int_literal = str_to_parse.parse().or(Err("could not parse int literal".to_string()))?;
                Token::IntLiteral(int_literal)
            };
                
            index += str_to_parse.len();
            tokens.push(new_number_token);
            continue
        }
    }

    Ok(tokens)
}