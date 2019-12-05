use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut memory = Vec::new();

    for line in input.split(',') {
        memory.push(line.trim().parse::<i32>()?)
    }

    day5a(&memory);
    day5b(&memory);
    Ok(())
}

fn get_param_value(memory: &Vec<i32>, pos: usize, instruction: &mut i32) -> i32 {
    let mode = *instruction % 10;
    *instruction /= 10;

    match mode {
        // position mode
        0 => memory[memory[pos] as usize],
        // immediate mode
        1 => memory[pos],
        _ => panic!("invalid mode {}", mode),
    }
}

fn get_outptr<'a>(memory: &'a mut Vec<i32>, pos: usize, instruction: &mut i32) -> &'a mut i32 {
    let v = memory[pos];
    let mode = *instruction % 10;
    *instruction /= 10;

    match mode {
        // position mode
        0 => &mut memory[v as usize],
        _ => panic!("invalid output mode {}", mode),
    }
}

fn run_intcode(state: &Vec<i32>, inputs: &[i32]) {
    let mut memory = state.clone();
    let mut eip = 0;
    let mut input_pos = 0;

    loop {
        let mut instruction = memory[eip];
        let opcode = instruction % 100;
        instruction /= 100;

        match opcode % 100 {
            1 | 2 => {
                let in1 = get_param_value(&memory, eip + 1, &mut instruction);
                let in2 = get_param_value(&memory, eip + 2, &mut instruction);
                let out = get_outptr(&mut memory, eip + 3, &mut instruction);
                *out = if opcode == 1 { in1 + in2 } else { in1 * in2 };
                eip += 4;
            }
            3 => {
                assert!(input_pos < inputs.len());
                let out = get_outptr(&mut memory, eip + 1, &mut instruction);
                *out = inputs[input_pos];
                input_pos += 1;
                eip += 2;
            }
            4 => {
                let val = get_param_value(&memory, eip + 1, &mut instruction);
                println!("output: {}", val);
                eip += 2;
            }
            5 | 6 => {
                let val = get_param_value(&memory, eip + 1, &mut instruction);
                if (opcode == 5 && val != 0) || (opcode == 6 && val == 0) {
                    eip = get_param_value(&memory, eip + 2, &mut instruction) as usize;
                } else {
                    eip += 3;
                }
            }
            7 | 8 => {
                let val1 = get_param_value(&memory, eip + 1, &mut instruction);
                let val2 = get_param_value(&memory, eip + 2, &mut instruction);
                let out = get_outptr(&mut memory, eip + 3, &mut instruction);
                if (opcode == 7 && val1 < val2) || (opcode == 8 && val1 == val2) {
                    *out = 1;
                } else {
                    *out = 0;
                }
                eip += 4;
            }
            99 => break,
            _ => panic!("unknown opcode {}", opcode),
        }
    }
}

fn day5a(state: &Vec<i32>) {
    let mut memory = state.clone();
    println!("day5a run...");
    run_intcode(&mut memory, &[1]);
}

fn day5b(state: &Vec<i32>) {
    let mut memory = state.clone();
    println!("day5b run...");
    run_intcode(&mut memory, &[5]);
}