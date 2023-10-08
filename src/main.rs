// mod lexer;

use nacho_basic::analyze_struct_type_identifiers;
use nacho_basic::lexer;

fn main() {
    let token_list = lexer("test/struct.nb");

    // token_list
    //     .iter()
    //     .for_each(|token_list| println!("{:?}", token_list));

    let types = analyze_struct_type_identifiers(&token_list);
    println!("{:?}", types)

    // types.iter().for_each(|t| println!("{}", t))

    // let x = "asdf";
    // let y = "asdf";
    // if x == y {print!("asdf")}

    // let x =
}
