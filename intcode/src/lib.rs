pub struct Intcode {
    pub memory: Vec<i32>,
    eip: usize,
    is_done: bool,
}

impl Intcode {
    pub fn new(state: &Vec<i32>) -> Self {
        Self {
            memory: state.clone(),
            eip: 0,
            is_done: false,
        }
    }

    fn get_param_val_and_mode(&mut self, instruction: &mut i32) -> (i32, i32) {
        let v = self.memory[self.eip];
        self.eip += 1;

        let mode = *instruction % 10;
        *instruction /= 10;

        (v, mode)
    }

    fn get_param_value(&mut self, instruction: &mut i32) -> i32 {
        let (v, mode) = self.get_param_val_and_mode(instruction);

        match mode {
            // position mode
            0 => self.memory[v as usize],
            // immediate mode
            1 => v,
            _ => panic!("invalid mode {}", mode),
        }
    }

    fn get_outptr<'a>(&'a mut self, instruction: &mut i32) -> &'a mut i32 {
        let (v, mode) = self.get_param_val_and_mode(instruction);

        match mode {
            // position mode
            0 => &mut self.memory[v as usize],
            _ => panic!("invalid output mode {}", mode),
        }
    }

    pub fn run(&mut self, inputs: &[i32]) -> Vec<i32> {
        let mut input_pos = 0;
        let mut output = Vec::new();

        loop {
            let mut instruction = self.memory[self.eip];
            self.eip += 1;

            let opcode = instruction % 100;
            instruction /= 100;

            match opcode % 100 {
                1 => {
                    let in1 = self.get_param_value(&mut instruction);
                    let in2 = self.get_param_value(&mut instruction);
                    let out = self.get_outptr(&mut instruction);
                    *out = in1 + in2;
                }
                2 => {
                    let in1 = self.get_param_value(&mut instruction);
                    let in2 = self.get_param_value(&mut instruction);
                    let out = self.get_outptr(&mut instruction);
                    *out = in1 * in2;
                }
                3 => {
                    if input_pos >= inputs.len() {
                        /* rewind eip so that execution can be resumed */
                        self.eip -= 1;
                        break;
                    }
                    let out = self.get_outptr(&mut instruction);
                    *out = inputs[input_pos];
                    input_pos += 1;
                }
                4 => {
                    let val = self.get_param_value(&mut instruction);
                    output.push(val);
                }
                5 => {
                    let val = self.get_param_value(&mut instruction);
                    if val != 0 {
                        self.eip = self.get_param_value(&mut instruction) as usize;
                    } else {
                        self.eip += 1;
                    }
                }
                6 => {
                    let val = self.get_param_value(&mut instruction);
                    if val == 0 {
                        self.eip = self.get_param_value(&mut instruction) as usize;
                    } else {
                        self.eip += 1;
                    }
                }
                7 => {
                    let in1 = self.get_param_value(&mut instruction);
                    let in2 = self.get_param_value(&mut instruction);
                    let out = self.get_outptr(&mut instruction);
                    *out = if in1 < in2 { 1 } else { 0 };
                }
                8 => {
                    let in1 = self.get_param_value(&mut instruction);
                    let in2 = self.get_param_value(&mut instruction);
                    let out = self.get_outptr(&mut instruction);
                    *out = if in1 == in2 { 1 } else { 0 };
                }
                99 => {
                    self.is_done = true;
                    break;
                }
                _ => panic!("unknown opcode {}", opcode),
            }
        }

        output
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}
