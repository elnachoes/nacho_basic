use crate::models::{token, variable::Struct, Token, Type};
use std::{collections::HashMap, io::Read};

pub fn analyze_struct_types(tokens: &mut Vec<Token>) -> Result<(), String> {
    let mut type_map: HashMap<String, Token> = HashMap::new();

    // for (index, (line, token)) in flattened_token_list.iter().enumerate() {
    //     if let Token::Struct = **token {
    //         let next_index = index + 1;
    //         if next_index < flattened_token_list.len() {
    //             let type_name =
    //                 if let Token::Identifier(ref type_name) = flattened_token_list[next_index].1 {
    //                     type_name.clone()
    //                 } else {
    //                     return Err(format!(
    //                         "line :{} char : {} : expected identifier after keyword struct",
    //                         *line + 1,
    //                         index
    //                     ));
    //                 };

    //             type_map.insert(type_name.clone(), Token::Type(VariableType::Struct(type_name.clone())));
    //         }
    //     }
    // }

    // tokens
    //     .iter_mut()
    //     .for_each(|token_list|
    //         token_list
    //             .iter_mut()
    //             .for_each(|token| {
    //                 if let Token::Identifier(identifier) = token {
    //                     if type_map.contains_key(identifier) {
    //                         *token = Token::Type(VariableType::Struct(identifier.clone()))
    //                     }
    //                 }
    //             })
    //     );
    Ok(())
}

pub fn read_struct_declaration(tokens: &Vec<Token>, index: usize) -> Result<Struct, &str> {
    #[derive(PartialEq, Eq)]
    enum IdReadState {
        Field,
        Type,
    }

    let mut nb_struct = Struct {
        identifier: String::default(),
        map: HashMap::new(),
    };

    let mut read_state = IdReadState::Type;
    let mut current_nb_type : &Type = &Type::Void;

    for token in tokens.iter().skip(index) {
        match token {
            Token::Type(nb_type) => {
                if let IdReadState::Type = read_state {
                    match nb_type {
                        Type::Void => {
                            return Err("error : void cannot be used as a type");
                        },
                        _ => {
                            current_nb_type = nb_type;
                            read_state = IdReadState::Field
                        },
                    }
                } else {
                    // return Err(format!("error : unexpected type {}", nb_type).as_str());
                }
            },
            Token::Identifier(identifier) => {
                if let IdReadState::Field = read_state {
                    nb_struct.map.insert(identifier.clone(), current_nb_type.allocate_variable().unwrap());
                    read_state = IdReadState::Type
                } else {
                    // error unexpected token
                }
            },
            Token::OpenBlockLimiter | Token::Type(_) => read_state = IdReadState::Field,
            Token::CloseBlockLimiter => break,
            _ => {
                // todo: unxpected token error here
                // should base it off of what the current state
            }
        }
    }

    Ok(nb_struct)
}

fn analyze_function_declarations(flatened_token_list: &Vec<(usize, &Token)>) {}

/// ----building the ast part----
/// ANALYZE FUNCTIONS IN SCOPE ->
///     ANALYZE FUNCTIONS IN SCOPE
///     ANALYZE VARIABLES IN SCOPE
///     ANALYZE CONTROL OF SCOPE
///     ANALYZE RETURNS
pub fn analyze(tokens: &mut Vec<Token>) {

    // // identifiers that should be types replace with type tokens
    // let type_map = analyze_struct_types(&flattened_token_list).expect("type parsing error");
    // tokens
    //     .iter_mut()
    //     .for_each(|token_list|
    //         token_list
    //             .iter_mut()
    //             .for_each(|token| {
    //                 if let Token::Identifier(identifier) = token {
    //                     if type_map.contains_key(identifier) {
    //                         *token = Token::Type(Type::Struct(identifier.clone()))
    //                     }
    //                 }
    //             })
    //     );
}
