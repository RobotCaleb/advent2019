use simpleuserinput::simple_user_input::get_input;

enum ParameterMode {
    Position,
    Immediate,
}

struct InputProcessor {
    read: bool,
    value: i32,
}

impl InputProcessor {
    pub fn new() -> InputProcessor {
        InputProcessor {
            read: true,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.read = false;
        self.value = value;
    }
    pub fn get_value(&mut self) -> i32 {
        if !self.read {
            self.read = true;
            return self.value;
        } else {
            self.value = get_input("Input: ").parse::<i32>().unwrap();
            self.value
        }
    }
    pub fn get_read(&self) -> bool {
        self.read
    }
}

struct OutputProcessor {
    read: bool,
    value: i32,
}

impl OutputProcessor {
    pub fn new() -> OutputProcessor {
        OutputProcessor {
            read: true,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.read = false;
        self.value = value;
    }
    pub fn get_value(&mut self) -> i32 {
        self.read = true;
        self.value
    }
    pub fn get_read(&self) -> bool {
        self.read
    }
}

pub struct IntCode {
    pc: usize,
    halted: bool,
    state: Vec<i32>,
    input: InputProcessor,
    output: OutputProcessor,
}

impl IntCode {
    pub fn new() -> IntCode {
        IntCode {
            pc: 0,
            halted: false,
            state: vec![],
            input: InputProcessor::new(),
            output: OutputProcessor::new(),
        }
    }

    pub fn load(&mut self, input: &Vec<i32>) {
        self.state = input.clone();
    }

    pub fn set_input(&mut self, input: i32) {
        self.input.set_value(input);
    }

    pub fn get_input_was_read(&self) -> bool {
        self.input.get_read()
    }

    pub fn get_output(&mut self) -> i32 {
        self.output.get_value()
    }

    pub fn get_output_was_read(&self) -> bool {
        self.output.get_read()
    }

    pub fn get_halted(&self) -> bool {
        self.halted
    }

    pub fn get_state(&self) -> Vec<i32> {
        self.state.clone()
    }

    /// Loops until halted
    pub fn run(&mut self) {
        while !self.get_halted() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            println!("Cannot step, halted");
            return;
        }
        let opcode = self.state[self.pc];
        let opi = opcode % 100;
        let a = ((opcode / 10000) % 10) as u32;
        let b = ((opcode / 1000) % 10) as u32;
        let c = ((opcode / 100) % 10) as u32;
        let param_modes = vec![self.get_mode(c), self.get_mode(b), self.get_mode(a)];
        match opi {
            1 => {
                println!("handling 1");
                // add from first two parameters, store at 3rd
                self.add(
                    self.get_at(self.state[self.pc + 1], &param_modes[0]),
                    self.get_at(self.state[self.pc + 2], &param_modes[1]),
                    self.state[self.pc + 3] as usize,
                );
                self.pc += 4;
            }
            2 => {
                println!("handling 2");
                // multiply from first two parameters, store at 3rd
                self.mul(
                    self.get_at(self.state[self.pc + 1], &param_modes[0]),
                    self.get_at(self.state[self.pc + 2], &param_modes[1]),
                    self.state[self.pc + 3] as usize,
                );
                self.pc += 4;
            }
            3 => {
                println!("handling 3");
                // get input, store at 2nd
                let index = self.state[self.pc + 1] as usize;
                let input = self.input.get_value();
                self.state[index] = input;
                self.pc += 2;
            }
            4 => {
                println!("handling 4");
                // output value at 2nd
                // todo: signal that output is available
                let index = self.state[self.pc + 1];
                let val = self.get_at(index, &param_modes[0]);
                println!("read from {}, value is {}", index, val);
                println!("Output: {}", val);
                self.output.set_value(val);
                self.pc += 2;
            }
            5 => {
                println!("handling 5");
                // jump if true
                let check = self.get_at(self.state[self.pc + 1], &param_modes[0]);
                let val = self.get_at(self.state[self.pc + 2], &param_modes[1]);
                if check != 0 {
                    self.pc = val as usize;
                }
            }
            6 => {
                println!("handling 6");
                // jump if true
                let check = self.get_at(self.state[self.pc + 1], &param_modes[0]);
                let val = self.get_at(self.state[self.pc + 2], &param_modes[1]);
                if check == 0 {
                    self.pc = val as usize;
                }
            }
            7 => {
                println!("handling 7");
                // less than
                let a = self.get_at(self.state[self.pc + 1], &param_modes[0]);
                let b = self.get_at(self.state[self.pc + 2], &param_modes[1]);
                let pos = self.get_at(self.state[self.pc + 3], &param_modes[2]) as usize;
                if a < b {
                    self.state[pos] = 1
                } else {
                    self.state[pos] = 0
                }
                self.pc += 4;
            }
            8 => {
                println!("handling 8");
                // equals
                let a = self.get_at(self.state[self.pc + 1], &param_modes[0]);
                let b = self.get_at(self.state[self.pc + 2], &param_modes[1]);
                let pos = self.get_at(self.state[self.pc + 3], &param_modes[2]) as usize;
                if a == b {
                    println!("Writing 1 to position {}", pos);
                    self.state[pos] = 1
                } else {
                    println!("Writing 0 to position {}", pos);
                    self.state[pos] = 0
                }
                self.pc += 4;
            }
            99 => self.halted = true,
            _ => println!("Error at index {} found {}", self.pc, opcode),
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
