use simpleuserinput::simple_user_input::get_input;

enum ParameterMode {
    Position,
    Immediate,
}

pub struct IntCode {
    pub state: Vec<i32>,
}

impl IntCode {
    pub fn new() -> IntCode {
        IntCode { state: vec![] }
    }

    pub fn load(&mut self, input: &Vec<i32>) {
        self.state = input.clone();
    }

    pub fn run(&mut self) {
        let mut pc: usize = 0;
        loop {
            let opcode = self.state[pc];
            let opi = opcode % 100;
            let a = ((opcode / 10000) % 10) as u32;
            let b = ((opcode / 1000) % 10) as u32;
            let c = ((opcode / 100) % 10) as u32;
            let param_modes = vec![self.get_mode(c), self.get_mode(b), self.get_mode(a)];
            match opi {
                1 => {
                    // add from first two parameters, store at 3rd
                    self.add(
                        self.get_at(self.state[pc + 1], &param_modes[0]),
                        self.get_at(self.state[pc + 2], &param_modes[1]),
                        self.state[pc + 3] as usize,
                    );
                    pc += 4;
                }
                2 => {
                    // multiply from first two parameters, store at 3rd
                    self.mul(
                        self.get_at(self.state[pc + 1], &param_modes[0]),
                        self.get_at(self.state[pc + 2], &param_modes[1]),
                        self.state[pc + 3] as usize,
                    );
                    pc += 4;
                }
                3 => {
                    // get input, store at 2nd
                    let index = self.state[pc + 1] as usize;
                    let input = get_input("Input: ").parse::<i32>().unwrap();
                    self.state[index] = input;
                    pc += 2;
                }
                4 => {
                    // output value at 2nd
                    let index = self.state[pc + 1];
                    let val = self.get_at(index, &param_modes[0]);
                    println!("Output: {}", val);
                    pc += 2;
                }
                // 5 => {
                //     // jump if true
                //     let index = self.state[pc + 1];
                //     let val = self.get_at(index, &param_modes[0]);
                //     println!("{}", val);
                //     step = 2;
                // }
                // 6 => {
                //     // jump if false
                //     let index = self.state[pc + 1];
                //     let val = self.get_at(index, &param_modes[0]);
                //     println!("{}", val);
                //     step = 2;
                // }
                // 7 => {
                //     // less than
                //     let index = self.state[pc + 1];
                //     let val = self.get_at(index, &param_modes[0]);
                //     println!("{}", val);
                //     step = 2;
                // }
                // 8 => {
                //     // equals
                //     let index = self.state[pc + 1];
                //     let val = self.get_at(index, &param_modes[0]);
                //     println!("{}", val);
                //     step = 2;
                // }
                99 => return,
                _ => println!("Error at index {} found {}", pc, opcode),
            }
        }
    }

    fn get_at(&self, at: i32, mode: &ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self.state[at as usize],
            ParameterMode::Immediate => at as i32,
        }
    }

    fn add(&mut self, a: i32, b: i32, index: usize) {
        self.state[index] = a + b;
    }

    fn mul(&mut self, a: i32, b: i32, index: usize) {
        self.state[index] = a * b;
    }

    fn get_mode(&self, mode: u32) -> ParameterMode {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => {
                println!("Invalid parameter mode: {}", mode);
                std::process::exit(1);
            }
        }
    }
}
