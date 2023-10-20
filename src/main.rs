use nacho_basic::{*};

fn print_section_line(section : &str) {
    println!("--------------------------------------{}--------------------------------------", section);
}

fn main() {
    let file = "test/struct.nb";
    
    print_section_line("lexer");

    // get token list and print it
    let token_list = lexer(file);
    token_list
        .iter()
        .for_each(|token_list| println!("{:?}", token_list));

    print_section_line("analyzer");

    let types = analyzer::read_struct_declaration(&token_list, 1);

    println!("{:?}", types);

    print_section_line("end")

    // println!("\n\n\n\n");

    // let token_list = lexer_no_line(file);

    // token_list
    //     .iter()
    //     .for_each(|token_list| println!("{:?}", token_list));

    // println!("")


    // println!("---- AFTER ANALYZING TYPES ----");

    // analyze(&mut token_list);

    // token_list
    //     .iter()
    //     .for_each(|token_list| println!("{:?}", token_list));

    // let types = analyze_struct_type_identifiers(&token_list);
    // println!("{:?}", types)

    // types.iter().for_each(|t| println!("{}", t))

    // let x = "asdf";
    // let y = "asdf";
    // if x == y {print!("asdf")}

    // let x =
}
