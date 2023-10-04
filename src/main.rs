mod lexer;

fn main() {
    let token_list = lexer::lexer("asdf.nb");
    println!("{:?}",token_list);
}
