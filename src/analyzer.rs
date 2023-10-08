use itertools::PeekingNext;

use crate::*;

pub fn analyze_struct_type_identifiers(tokens: &Vec<Vec<Token>>) -> Result<Vec<String>, String> {
    let flatened_token_list = get_flattened_token_list(tokens);

    let mut struct_types_list: Vec<String> = vec![];

    for (index, (line, token)) in flatened_token_list.iter().enumerate() {
        if let Token::Struct = **token {
            let next_index = index + 1;
            if next_index < flatened_token_list.len() {
                let type_name = if let Token::Identifier(ref type_name) =
                    flatened_token_list[next_index].1
                {
                    type_name.clone()
                } else {
                    return Err(format!(
                        "error : line :{} char : {} : expected identifier after keyword struct",
                        *line + 1, index
                    ));
                };
                struct_types_list.push(type_name);
            }
        }
    }

    Ok(struct_types_list)
}

fn get_flattened_token_list(tokens: &Vec<Vec<Token>>) -> Vec<(usize, &Token)> {
    tokens
        .iter()
        .enumerate()
        .fold(vec![], |mut accumulated_vec, (line_number, vec)| {
            vec.iter()
                .for_each(|token_ref| accumulated_vec.push((line_number, token_ref)));
            accumulated_vec
        })
}

pub fn analyze(tokens: &mut Vec<Vec<Token>>) {
    let types = analyze_struct_type_identifiers(&tokens);

    // tokens
    //     .iter_mut()
    //     .for_each(|line_tokens| {
    //         line_tokens
    //             .iter_mut()
    //             .for_each(|token| {

    //             })
    //     }) 

}
