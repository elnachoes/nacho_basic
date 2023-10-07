struct Interpreter<'a> {
    pub stack_memory : Vec<&'a dyn ValueType>,
    pub heep_memory : Vec<u8>,
    pub program_memory : Vec<String>,
    pub program_counter : u32,
}

impl Interpreter<'_> {
    pub fn run_program(&mut self) {
        // main process loop here.
        // read line -> execute line -> inc pc
        loop {

        }
    }
}