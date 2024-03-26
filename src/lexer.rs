use crate::models::token::Token;
use std::{io::Read, vec};
use nom::{
    bytes::complete::is_not, 
    character::complete::{alpha1, char, one_of}, 
    combinator::recognize, 
    error::Error, 
    multi::{many0, many1}, 
    sequence::{delimited, terminated, } 
};

/// this is a helper function to recognize a list of characters inside of an input in a sequence.
fn recognize_sequence_of<'a>(input : &'a str, char_list : &'a str) -> Result<(&'a str, &'a str), String> {
    recognize(many1(terminated(one_of(char_list), many0(char::<&'a str, Error<&'a str>>('_')))))(input).map_err(|error| error.to_string())
}

/// trys to parse a string literal.
fn string_literal_token(input: &str) -> Result<(Token, &str), String> {
    delimited(char::<&str, Error<&str>>('"'), is_not("\""), char('"'))(input)
        .map_err(|_| "could not parse string literal".to_string())
        .map(|(remaining, value)| (Token::StringLiteral(value.to_string()), remaining))
}

/// trys to parse a number token. this will parse a float literal if there is a decimal point present, if not it will return an int literal.
fn number_literal_token(input: &str) -> Result<(Token, &str), String> {
    recognize_sequence_of(input, "0123456789.")
        .and_then(|(remaining, value)| 
            if value.contains(".") {
                Ok((Token::FloatLiteral(value.parse().or(Err("could not parse float literal".to_string()))?), remaining))
            } else {
                Ok((Token::IntLiteral(value.parse().or(Err("could not parse int literal".to_string()))?), remaining))
            }
        )
}

/// trys to parse a keyword token, then an identity token
fn keyword_or_identity_token(input: &str) -> Result<(Token, &str), String> {
    keyword_token(input).or(
        recognize_sequence_of(input, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")
            .map(|(remaining, value)| (Token::Identity(value.to_string()), remaining))
    )
} 

/// trys to parse a keyword token
fn keyword_token(input: &str) -> Result<(Token, &str), ()> {
    alpha1::<&str, Error<&str>>(input)
        .map_err(|_| ())
        .and_then(|(remaining, value)| Ok((Token::try_from(value)?, remaining)))
}

/// trys to parses a single char token
fn char_token(input: &str) -> Result<(Token, &str), ()> {
    Ok((input.chars().next().ok_or(())?.try_into()?, &input[1..]))
}

/// this will return a vec of char/keyword/literal/identity tokens from a nacho basic file at the given path
pub fn lexer(file_path : &str) -> Result<Vec<Token>, String> {
    // open the file and read it into a string
    let mut input = String::default();
    std::fs::File::options()
        .read(true)
        .open(file_path)
        .or(Err("could not open file".to_string()))?
        .read_to_string(&mut input)
        .or(Err("could not read file".to_string()))?;

    let mut remaining = input.as_str();
    let mut output = vec![];
    while remaining.len() > 0 {
        // try reading a single character token first
        if let Ok((token, new_remaining)) = char_token(remaining) {
            if !token.is_ignored() { output.push(token) }
            remaining = new_remaining;
            continue
        }

        // if reading a character token was unsuccessful parse a multicharacter token based on the next char.
        let next_char = remaining.chars().next().unwrap();

        // read a keyword or idenity token if the first character is a letter or underscore
        let (token, new_remaining) = if next_char.is_alphabetic() || next_char == '_' {
            keyword_or_identity_token(remaining)?
        // read a number literal if the first character is a number or decimal dot
        } else if next_char.is_numeric() || next_char == '.' {
            number_literal_token(remaining)?
        // read a string literal if the first character is a quote
        } else if next_char == '"' {
            string_literal_token(remaining)?
        } else {
            return Err(format!("unexpected char : {next_char}"))
        };

        if !token.is_ignored() { output.push(token) }
        remaining = new_remaining
    }

    Ok(output)
}