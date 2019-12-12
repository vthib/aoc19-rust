#[derive(Clone)]
pub struct Intcode {
    pub memory: Vec<i64>,
    eip: usize,
    is_done: bool,
    rel_base: i64,
}

impl Intcode {
    pub fn new(state: &Vec<i64>) -> Self {
        Self {
            memory: state.clone(),
            eip: 0,
            is_done: false,
            rel_base: 0,
        }
    }

    fn get_param_val_and_mode(&mut self, instruction: &mut i64) -> (i64, i64) {
        let v = self.memory[self.eip];
        self.eip += 1;

        let mode = *instruction % 10;
        *instruction /= 10;

        (v, mode)
    }

    fn ensure_memory_available(&mut self, pos: usize) {
        if pos >= self.memory.len() {
            self.memory.resize(pos + 1, 0);
        }
    }

    fn get_memory_at(&mut self, pos: usize) -> i64 {
        self.ensure_memory_available(pos);
        self.memory[pos]
    }

    fn get_memory_at_mut(&mut self, pos: usize) -> &mut i64 {
        self.ensure_memory_available(pos);
        &mut self.memory[pos]
    }
    
    fn get_param_value(&mut self, instruction: &mut i64) -> i64 {
        let (v, mode) = self.get_param_val_and_mode(instruction);

        match mode {
            // position mode
            0 => self.get_memory_at(v as usize),
            // immediate mode
            1 => v,
            2 => self.get_memory_at((v + self.rel_base) as usize),
            _ => panic!("invalid mode {}", mode),
        }
    }

    fn get_outptr<'a>(&'a mut self, instruction: &mut i64) -> &'a mut i64 {
        let (v, mode) = self.get_param_val_and_mode(instruction);

        match mode {
            // position mode
            0 => self.get_memory_at_mut(v as usize),
            2 => self.get_memory_at_mut((v + self.rel_base) as usize),
            _ => panic!("invalid output mode {}", mode),
        }
    }

    pub fn run(&mut self, inputs: &[i64]) -> Vec<i64> {
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
                9 => {
                    let v = self.get_param_value(&mut instruction);
                    self.rel_base += v;
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
