struct Interpreter {
    struct_type_registry : HashMap<String, Variable>,
    program_memory : Vec<Vec<Token>>,
    stack_memory : Vec<Vec<Variable>>,
    heap_memory : Vec<Box<Variable>>,
    program_counter : i64
}

impl Interpreter {

    // ---- TODO ---- 
}