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
    let array_type = if let Some(ref token) = nb_type_token {
        if let Token::Type(ref nb_type) = token {
            nb_type
        } else {
            return Err("error : read_array_type : expected type token".to_string())    
        }
    } else {
        return Err("error : read_array_type : index out of range".to_string())
    };

    // check if a void type was used like void[] stupid_void_array
    if let Type::Void = array_type {
        return Err("error : read_array_type : you cannot have a void array type".to_string())
    }

    // ---- TODO ---- : finish this


    // read the brackets and make sure they are present after the token 
    // 
    for token in tokens.iter().skip(index) {
        match *token {
            Token::OpenArrayLimiter => {},
            Token::CloseArrayLimiter => break,
            _ => return Err("error expected bracket".to_string())
        }
    }

    Ok(Type::Void)
}