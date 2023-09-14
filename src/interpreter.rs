struct Interpreter<'a> {
    pub stack_memory : Vec<&'a dyn ValueType>,
    pub heep_memory : Vec<u8>,
    pub program_memory : Vec<String>,
    pub program_counter : u32,
}

impl Interpreter<'_> {
    fn load_program(file_path : &String) -> Self {
        Interpreter { 
            stack_memory: Vec::new(),
            heep_memory: Vec::new(), 
            program_counter: 0,

            // read each line of the program to be compiled and ran line by line
            program_memory: fs::read_to_string(file_path)
                .expect("error : file not found")
                .as_str()
                .replace("\r", "")
                .split("\n")
                .map(|x| String::from(x))
                .collect()
        }
    }

    pub fn run_program(&mut self) {
        // main process loop here.
        // read line -> execute line -> inc pc
        loop {

        }
    }
}