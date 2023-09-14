use std::{collections::HashMap, io::{Read, Write}, default};
use serde::{Deserialize, Serialize};
use serde_json::{Result, json};
use {std::fs};

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub enum Token {
    Error,
    None,
    IntLiteral(i32),
    IntDeclaration,
    Assignment,
    Identity(String),
    // SpaceSeperator
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenLiteralMap {
    map : HashMap<String, Token> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccedingTokenList {
    pub token : Token,
    pub succeding_token_list : Vec<Token>
}

#[derive(Debug)]
pub struct Lexer {
    token_literal_map : TokenLiteralMap,
    succeding_token_lists : Vec<SuccedingTokenList>,
    read_index : usize,
    statement : String,
    current_token :Token,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            token_literal_map : Self::load_token_map().expect("could not load tokenmap"),
            succeding_token_lists : Self::load_succeding_token_lists().expect("could not load succeding tokens list"),
            statement : Default::default(),
            read_index : 0,
            current_token : Token::None,
        }
    }

    // this will load the token map for the lexer to parse tokens with
    // for now this is hard coded to TokenMap.json so I can easily add/delete/modify token identifiers like 'int', 'float', 'string', etc
    fn load_token_map() -> Result<TokenLiteralMap> {
        let mut file_to_read =  fs::File::options().read(true).open("TokenMap.json").expect("tokenmap not found");
        let mut json_buffer : String = Default::default();
        file_to_read.read_to_string(&mut json_buffer).expect("couldn't read file");
        Ok(serde_json::from_str(&json_buffer.as_str()).expect("couldn't deserialize"))
    }

    fn load_succeding_token_lists() -> Result<Vec<SuccedingTokenList>> {
        let mut file_to_read = fs::File::options().read(true).open("SuccedingTokensLists.json").expect("succeeding tokens list not found");
        let mut json_buffer : String = Default::default();
        file_to_read.read_to_string(&mut json_buffer).expect("couldn't read file");
        Ok(serde_json::from_str(&json_buffer.as_str()).expect("couldn't deserialize"))
    }

    // fn read_first_token(&mut self) -> Token {
    //     let starting_index = self.read_index;
        
    //     // keep reading alphanumeric charaters until one is not found
    //     for i in self.statement[starting_index..].chars() {
    //         if i.is_alphanumeric() { self.read_index += 1; } 
    //         else { break; }
    //     }

    //     let token = self.statement[starting_index..self.read_index].to_string();

    //     // try doing it more functionally
    //     // if the token matches a token key in the token map like 'int' -> Token::, clone it and return it.
    //     match self.token_literal_map.map.iter().filter(|(key, _val)| token.eq(*key)).next() {
    //         Some((_key, val)) => val.clone(),
    //         _ => Token::Identity(token)
    //     }
    // }


    fn read_token(&mut self) -> Token {
        let starting_index = self.read_index;

        // first check if the token exists in the token literal list. if so return that token. 
        // this will cover language keywords like 'int' '=' 
        for (token_string, token) in self.token_literal_map.map.iter() {

            // println!("{:?}", (starting_index + token_string.len() < self.statement.len()));
            // println!("{:?}",(&self.statement[starting_index..token_string.len()]));
            println!("{:?}",(&token_string));

            if starting_index + token_string.len() < self.statement.len() &&
                &self.statement[starting_index..token_string.len()] == token_string.as_str() {
                
                // move the read index the difference past the token string
                self.read_index = starting_index + (starting_index - token_string.len());
                return token.clone();                
            }
        }
        
        // check for an int literal
        // if self.statement.as_bytes()[starting_index].is_ascii_digit() {
            
        //     let end_of_int_literal_index = self.statement
        //         .as_bytes()[starting_index..self.statement.len()]
        //         .iter()
        //         .enumerate()
        //         .find(|(index, char)| {!char.is_ascii_digit()});

        //     if let Some((usize, &u8)) = end_of_int_literal_index {
                
        //         return
        //     } else {

        //     }
        // }


        // todo : you would check a string literal like "string"


        // check for an id
        
        Token::None
    }


    pub fn lex_statement(&mut self, statement : String) {
        // step 1 strip out whitespace
        self.read_index = 0;
        self.statement = statement.replace(" ", "").to_string();
        

        let x = self.read_token();
        println!("{:?}", x)

        // step 2 read in a declaration or 
        // let x = self.read_first_token();


        // step 2 check if we are reading a single character token
        // setp 3 if we were reading a single character token
    }
}



pub fn test_load_succeding_tokens_lists() {
    let succeding_tokens_lists = json!([
        SuccedingTokenList {
            token : Token::Assignment,
            succeding_token_list : vec![
                Token::Identity(Default::default()),
                Token::IntLiteral(Default::default())
            ]
        },

        SuccedingTokenList {
            token : Token::Identity(Default::default()),
            succeding_token_list : vec![
                Token::Assignment
            ]
        },

        SuccedingTokenList {
            token : Token::IntLiteral(Default::default()),
            succeding_token_list : vec![]
        },

        SuccedingTokenList {
            token : Token::IntDeclaration,
            succeding_token_list : vec![
                Token::Identity(Default::default()),
            ]
        },
    ]);

    std::fs::File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open("SuccedingTokensLists.json")
        .expect("could not open file")
        .write_all(succeding_tokens_lists.to_string().as_bytes())
        .expect("could not write to file");
}
