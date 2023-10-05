mod lexer;

fn main() {
    let token_list = lexer::lexer("asdf.nb");
    token_list.iter().for_each(|token_list| println!("{:?}", token_list));
}
