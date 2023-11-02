use std::collections::HashMap;
use crate::{models::*};

pub fn analyze_struct_types(tokens: &mut Vec<Token>) -> Result<(), String> {
    unimplemented!("todo : analyze_struct_types");
}

pub fn analyze_array_types(tokens : &mut Vec<Token>) {
    unimplemented!()
}


pub fn read_array_type_test(tokens : &Vec<Token>, index : usize) -> Result<Type, String> {
    enum ReadState {
        OpenBracket,
        CloseBracket
    }

    let mut tokens = tokens;

    let nb_type_token = tokens.iter().nth(index);
    
    // get base type out of the token
    let nb_type = if let Some(ref token) = nb_type_token {
        if let Token::Type(ref nb_type) = token {
            nb_type
        } else {
            return Err("error : read_array_type : expected type token".to_string())    
        }
    } else {
        return Err("error : read_array_type : index out of range".to_string())
    };

    // check if a void type was used like void[] stupid_void_array
    if let Type::Void = nb_type {
        return Err("error : read_array_type : you cannot have a void array type".to_string())
    }

    let mut read_state = ReadState::OpenBracket;

    for token in tokens.iter().skip(index + 1) {
        match *token {
            Token::OpenArrayLimiter => read_state = ReadState::CloseBracket,
            Token::CloseArrayLimiter => {
                if let ReadState::CloseBracket = read_state {
                    break
                } else {
                    return Err("error : expected close bracket".to_string())
                }
            },
            _ => return Err("error : expected brackets".to_string())
        }
    }

    if let Some(nb_array_type) = nb_type.to_array_type() {
        // advance the index past the type token
        // ---- DO THIS IN THE PARENT ----
        // tokens.drain(index..=index + 2);
        // tokens.insert(index, Token::Type(Type::Array(nb_array_type)))
        Ok(Type::Array(nb_array_type))
    } else {
        return Err("error : type cannot be stored in an array".to_string())
    }
}

pub fn read_struct_declaration_test(tokens : &Vec<Token>, index : usize) -> Result<Struct, String> {
    #[derive(PartialEq, Eq, Debug)]
    enum IdReadState {
        StructIdentifier,
        Field,
        Type,
    }

    let mut nb_struct = Struct {
        struct_type : String::default(),
        map: HashMap::new(),
    };

    let mut read_state = IdReadState::StructIdentifier;
    let mut current_nb_type : Option<Type> = None;

    let mut token_iterator = tokens.iter().enumerate().skip(index);

    loop {
        let (index, token) = if let Some(token) = token_iterator.next() {
            token
        } else {
            return Err("error expecting tokens for struct declaration".to_string())
        };

        match token {
            Token::Type(nb_type) => {
                if let IdReadState::Type = read_state {
                    match nb_type {
                        Type::Void => return Err("error : read_struct_declaration : void cannot be used as a type".to_string()),
                        _ => {
                            if let Some(nb_type) = current_nb_type {
                                current_nb_type = Some(nb_type);
                                read_state = IdReadState::Field
                            }
                        },
                    }
                } else {
                    return Err(format!("error : read_struct_declaration : unexpected type {:?}", nb_type));
                }
            },
            Token::Identifier(identifier) => {
                match read_state {
                    IdReadState::StructIdentifier => {
                        nb_struct.struct_type = identifier.clone();
                    }
                    //---- TODO ---- : this needs to use the type registry and functions to allocate a struct or a regular type
                    IdReadState::Field => {
                        // this will need to use the type registry to allocated a variable like a struct
                        // nb_struct.map.insert(identifier.clone(), current_nb_type.allocate_variable().unwrap());
                        read_state = IdReadState::Type
                    },
                    IdReadState::Type => {
                        // current_nb_type = Type::Struct(Struct { identifier: identifier.clone(), map: HashMap::new() });
                        read_state = IdReadState::Field
                    }
                    // IdReadState::ArrayBrackets(_) => return Err("error : expected array bracket".to_string())
                }
            },
            Token::OpenBlockLimiter => read_state = IdReadState::Type,
            Token::CloseBlockLimiter => break,
            Token::OpenArrayLimiter => {
                // try reading an array type and if you cant throw an error. 
                // DO NOT TRY ACTUALLY REPLACING THE TYPE JUST VALIDATE THAT YOU CAN DO IT 
                // THE ANALYZE STRUCTS FUNCTION WILL ANALYZE ARRAY TYPES AFTER ANALYZING THE STRUCT
                match read_array_type_test(&tokens, index - 1) {
                    Err(err) => return Err(err),
                    //step the index 3 times
                    Ok(nb_array_type) => {
                        current_nb_type = Some(nb_array_type);
                        for _ in 0..2 { token_iterator.next(); }
                    }
                }
            }
            _ => {
                // todo: unxpected token error here
                // should base it off of what the current state
                return Err(format!("error : read_struct_declaration_test : unexpected token : {token}"))
            }
        }
    }

    Ok(nb_struct)
}