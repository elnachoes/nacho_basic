struct Program {
    program_memory : Vec<Vec<Token>>,
    stack_memory : Vec<Vec<Variable>>,
    heap_memory : Vec<Box<Variable>>,
    program_counter : i64
}