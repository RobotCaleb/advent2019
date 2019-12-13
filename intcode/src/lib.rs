use simpleuserinput::simple_user_input::get_input;

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
            let mut step = 0;
            match opcode {
                1 => {
                    // add from first two parameters, store at 3rd
                    self.add(
                        self.state[pc + 1] as usize,
                        self.state[pc + 2] as usize,
                        self.state[pc + 3] as usize,
                    );
                    step = 4;
                }
                2 => {
                    // multiply from first two parameters, store at 3rd
                    self.mul(
                        self.state[pc + 1] as usize,
                        self.state[pc + 2] as usize,
                        self.state[pc + 3] as usize,
                    );
                    step = 4;
                }
                3 => {
                    // get input, store at 2nd
                    let index = self.state[pc + 1] as usize;
                    let input = get_input("Input: ").parse::<i32>().unwrap();
                    self.state[index] = input;
                    step = 2;
                }
                4 => {
                    // output value at 2nd
                    let index = self.state[pc + 1] as usize;
                    let val = self.state[index];
                    println!("{}", val);
                    step = 2;
                }
                99 => return,
                _ => println!("Error at index {} found {}", pc, opcode),
            }
            pc += step;
        }
    }

    fn add(&mut self, a: usize, b: usize, index: usize) {
        self.state[index] = self.state[a] + self.state[b];
    }

    fn mul(&mut self, a: usize, b: usize, index: usize) {
        self.state[index] = self.state[a] * self.state[b];
    }
}
