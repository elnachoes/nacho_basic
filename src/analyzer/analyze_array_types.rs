use crate::models::*;

/// this will read an array type like int[] or string[]
/// for now it only reads 1 dimensional arrays. unsure if multidimensional arrays are nessesary.
pub fn read_array_type(tokens : &mut Vec<Token>, index : usize) -> Result<Type, String> {
    enum ReadState {
        OpenBracket,
        CloseBracket
    }

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
        Ok(Type::Array(nb_array_type))
    } else {
        Err("error : type cannot be stored in an array".to_string())
    }
}