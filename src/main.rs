use std::collections::HashMap;

use nacho_basic::{*, models::{Token, Type, Struct, token}};

fn print_section_line(section : &str) {
    println!("--------------------------------------{}--------------------------------------", section);
}

fn main() {
    // let mut tokens = vec![
    //     Token::Type(Type::Int),
    //     Token::OpenArrayLimiter,
    //     Token::CloseArrayLimiter,
    // ];
    // println!("{:?}", tokens);
    // match read_array_type_test(tokens, 0) {
    //     Ok(tokens) => println!("{:?}", tokens),
    //     Err(err) => panic!("{}", err)
    // }


    let tokens = lexer("test/struct.nb");
    let nb_struct = analyzer::read_struct_declaration_test(&tokens, 1).expect("couldn't read struct");
    // println!("{nb_struct:?}");
    // println!("{:?}", nb_struct.map)


    // let mut test = HashMap::<&str, &str>::new();
    // test.insert("peniskey", "penisvalue");
    // println!("{:?}", test)
}
