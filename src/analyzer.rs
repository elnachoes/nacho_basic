use crate::models::{variable::Struct, Token, Type};
use std::collections::HashMap;

pub fn analyze_struct_types(tokens: &mut Vec<Token>) -> Result<(), String> {
    let mut type_map: HashMap<String, Token> = HashMap::new();
    unimplemented!("todo : analyze_struct_types");
    
    Ok(())
}

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
                            return Err("error : void cannot be used as a type".to_string());
                        },
                        _ => {
                            current_nb_type = nb_type.clone();
                            read_state = IdReadState::Field
                        },
                    }
                } else {
                    return Err(format!("error : unexpected type {:?}", nb_type));
                }
            },
            Token::Identifier(identifier) => {
                match read_state {
                    IdReadState::StructIdentifier => {
                        nb_struct.identifier = identifier.clone();
                    }
                    IdReadState::Field => {
                        nb_struct.map.insert(identifier.clone(), current_nb_type.allocate_variable().unwrap());
                        read_state = IdReadState::Type
                    },
                    IdReadState::Type => {
                        current_nb_type = Type::Struct(Struct { identifier: identifier.clone(), map: HashMap::new() });
                        read_state = IdReadState::Field
                    }
                }
            },
            Token::OpenBlockLimiter => read_state = IdReadState::Type,
            Token::CloseBlockLimiter => break,
            _ => {
                // todo: unxpected token error here
                // should base it off of what the current state
                return Err(format!("error : unexpected token identifier"))
            }
        }
    }

    Ok(nb_struct)
}

// fn analyze_function_declarations(flatened_token_list: &Vec<(usize, &Token)>) {}

/// ----building the ast part----
/// ANALYZE FUNCTIONS IN SCOPE ->
///     ANALYZE FUNCTIONS IN SCOPE
///     ANALYZE VARIABLES IN SCOPE
///     ANALYZE CONTROL OF SCOPE
///     ANALYZE RETURNS
pub fn analyze(tokens: &mut Vec<Token>) {
    unimplemented!("todo : analyze")
}
