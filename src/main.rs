// mod lexer;

use nacho_basic::lexer;

fn main() {
    let token_list = lexer::lexer("test/struct.nb");
    token_list.iter().for_each(|token_list| println!("{:?}", token_list));

    // let x = "asdf";
    // let y = "asdf";
    // if x == y {print!("asdf")}

    // let x = 




}
