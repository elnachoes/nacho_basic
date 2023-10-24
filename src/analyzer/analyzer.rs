use crate::models::{variable::{Struct, Array}, Token, Type};
use std::{collections::HashMap, usize};

pub fn analyze_struct_types(tokens: &mut Vec<Token>) -> Result<(), String> {
    let mut type_map: HashMap<String, Token> = HashMap::new();
    unimplemented!("todo : analyze_struct_types");
    
    Ok(())
}

// pub fn read_struct_declaration(tokens: &Vec<Token>, index: usize) -> Result<Struct, String> {
//     #[derive(PartialEq, Eq, Debug)]
//     enum IdReadState {
//         StructIdentifier,
//         Field,
//         Type,
//     }

//     let mut nb_struct = Struct {
//         identifier: String::default(),
//         map: HashMap::new(),
//     };

//     let mut read_state = IdReadState::StructIdentifier;
//     let mut current_nb_type : Type = Type::Void;

//     for token in tokens.iter().skip(index) {
//         match token {
//             Token::Type(nb_type) => {
//                 if let IdReadState::Type = read_state {
//                     match nb_type {
//                         Type::Void => {
//                             return Err("error : read_struct_declaration : void cannot be used as a type".to_string());
//                         },
//                         _ => {
//                             current_nb_type = nb_type.clone();
//                             read_state = IdReadState::Field
//                         },
//                     }
//                 } else {
//                     return Err(format!("error : read_struct_declaration : unexpected type {:?}", nb_type));
//                 }
//             },
//             Token::Identifier(identifier) => {
//                 match read_state {
//                     IdReadState::StructIdentifier => {
//                         nb_struct.identifier = identifier.clone();
//                     }
//                     IdReadState::Field => {
//                         nb_struct.map.insert(identifier.clone(), current_nb_type.allocate_variable().unwrap());
//                         read_state = IdReadState::Type
//                     },
//                     IdReadState::Type => {
//                         current_nb_type = Type::Struct(Struct { identifier: identifier.clone(), map: HashMap::new() });
//                         read_state = IdReadState::Field
//                     }
//                 }
//             },
//             Token::OpenBlockLimiter => read_state = IdReadState::Type,
//             Token::CloseBlockLimiter => break,

//             Token::OpenArrayLimiter => {
//                 // todo handle reading array type
//             }
//             _ => {
//                 // todo: unxpected token error here
//                 // should base it off of what the current state
//                 return Err("error : read_struct_declaration : unexpected token".to_string())
//             }
//         }
//     }

//     Ok(nb_struct)
// }

// /// this will read an array type like int[] or string[]
// /// for now it only reads 1 dimensional arrays. unsure if multidimensional arrays are nessesary.
// pub fn read_array_type(tokens : &mut Vec<Token>, index : usize) -> Result<Type, String> {
//     enum ReadState {
//         OpenBracket,
//         CloseBracket
//     }

//     let nb_type_token = tokens.iter().nth(index);
    
//     // get base type out of the token
//     let array_type = if let Some(ref token) = nb_type_token {
//         if let Token::Type(ref nb_type) = token {
//             nb_type
//         } else {
//             return Err("error : read_array_type : expected type token".to_string())    
//         }
//     } else {
//         return Err("error : read_array_type : index out of range".to_string())
//     };

//     // check if a void type was used like void[] stupid_void_array
//     if let Type::Void = array_type {
//         return Err("error : read_array_type : you cannot have a void array type".to_string())
//     }

//     // read the brackets and make sure they are present after the token 
//     // 
//     for token in tokens.iter().skip(index) {
//         match *token {
//             Token::OpenArrayLimiter => {},
//             Token::CloseArrayLimiter => break,
//             _ => return Err("error expected bracket".to_string())
//         }
//     }



//     Ok(Type::Array(Array))
// }


// fn analyze_function_declarations(flatened_token_list: &Vec<(usize, &Token)>) {}

// /// ----building the ast part----
// /// ANALYZE FUNCTIONS IN SCOPE ->
// ///     ANALYZE FUNCTIONS IN SCOPE
// ///     ANALYZE VARIABLES IN SCOPE
// ///     ANALYZE CONTROL OF SCOPE
// ///     ANALYZE RETURNS
// pub fn analyze(tokens: &mut Vec<Token>) {
//     unimplemented!("todo : analyze")
// }
