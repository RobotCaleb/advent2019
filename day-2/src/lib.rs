pub struct IntCode {
    pub state: Vec<i32>,
}

impl IntCode {
    pub fn new() -> IntCode {
        IntCode { state: vec![] }
    }

    pub fn load(&mut self, input: Vec<i32>) {
        self.state = input;
    }

    pub fn run(&mut self) {
        let mut pc: usize = 0;
        loop {
            let opcode = self.state[pc];
            match opcode {
                1 => self.add(
                    self.state[pc + 1] as usize,
                    self.state[pc + 2] as usize,
                    self.state[pc + 3] as usize,
                ),
                2 => self.mul(
                    self.state[pc + 1] as usize,
                    self.state[pc + 2] as usize,
                    self.state[pc + 3] as usize,
                ),
                99 => return,
                _ => println!("Error at index {} found {}", pc, opcode),
            }
            pc += 4;
        }
    }

    fn add(&mut self, a: usize, b: usize, index: usize) {
        self.state[index] = self.state[a] + self.state[b];
    }

    fn mul(&mut self, a: usize, b: usize, index: usize) {
        self.state[index] = self.state[a] * self.state[b];
    }
}
