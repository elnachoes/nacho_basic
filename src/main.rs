mod lexer;

fn main() {
    let token_list = lexer::lexer("test/array_init.nb");
    token_list.iter().for_each(|token_list| println!("{:?}", token_list));

    // let x = "asdf";
    // let y = "asdf";
    // if x == y {print!("asdf")}
}
