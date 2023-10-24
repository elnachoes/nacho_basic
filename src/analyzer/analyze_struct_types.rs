use std::collections::HashMap;
use crate::models::*;


pub fn read_struct_declaration(tokens: &Vec<Token>, index: usize) -> Result<Struct, String> {
    #[derive(PartialEq, Eq, Debug)]
    enum IdReadState {
        StructIdentifier,
        Field,
        Type,
    }

    let mut nb_struct = Struct {
        identifier: String::default(),
        map: HashMap::new(),
    };

    let mut read_state = IdReadState::StructIdentifier;
    let mut current_nb_type : Type = Type::Void;

    for token in tokens.iter().skip(index) {
        match token {
            Token::Type(nb_type) => {
                if let IdReadState::Type = read_state {
                    match nb_type {
                        Type::Void => {
                            return Err("error : read_struct_declaration : void cannot be used as a type".to_string());
                        },
                        _ => {
                            current_nb_type = nb_type.clone();
                            read_state = IdReadState::Field
                        },
                    }
                } else {
                    return Err(format!("error : read_struct_declaration : unexpected type {:?}", nb_type));
                }
            },
            Token::Identifier(identifier) => {
                match read_state {
                    IdReadState::StructIdentifier => {
                        nb_struct.identifier = identifier.clone();
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
                }
            },
            Token::OpenBlockLimiter => read_state = IdReadState::Type,
            Token::CloseBlockLimiter => break,

            Token::OpenArrayLimiter => {
                // todo handle reading array type
            }
            _ => {
                // todo: unxpected token error here
                // should base it off of what the current state
                return Err("error : read_struct_declaration : unexpected token".to_string())
            }
        }
    }

    Ok(nb_struct)
}