use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut memory = Vec::new();

    for line in input.split(',') {
        memory.push(line.trim().parse::<usize>()?)
    }

    day2a(&memory);
    day2b(&memory);
    Ok(())
}

fn run_program(memory: &mut Vec<usize>) {
    let mut eip = 0;

    loop {
        let opcode = memory[eip];

        match opcode {
            1 => {
                let eipin1 = memory[eip + 1];
                let eipin2 = memory[eip + 2];
                let eipout = memory[eip + 3];
                memory[eipout] = memory[eipin1] + memory[eipin2];
            }
            2 => {
                let eipin1 = memory[eip + 1];
                let eipin2 = memory[eip + 2];
                let eipout = memory[eip + 3];
                memory[eipout] = memory[eipin1] * memory[eipin2];
            }
            99 => break,
            _ => panic!("unknown opcode {}", opcode),
        }
        eip += 4;
    }
}

fn day2a(state: &Vec<usize>) {
    let mut memory = state.clone();
    memory[1] = 12;
    memory[2] = 2;
    run_program(&mut memory);

    println!("day2a: value at pos 0: {}", memory[0]);
}

fn day2b(state: &Vec<usize>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = state.clone();
            memory[1] = noun;
            memory[2] = verb;
            run_program(&mut memory);
            if memory[0] == 19690720 {
                println!(
                    "day2b: noun: {}, verb: {}, answer: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }

    panic!("no noun,verb pair found");
}