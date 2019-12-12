use std::io;
use std::io::Read;
use intcode::Intcode;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut memory = Vec::new();

    for line in input.split(',') {
        memory.push(line.trim().parse::<i64>()?)
    }

    day2a(&memory);
    day2b(&memory);
    Ok(())
}

fn day2a(state: &Vec<i64>) {
    let mut memory = state.clone();
    memory[1] = 12;
    memory[2] = 2;
    let mut intcode = Intcode::new(&memory);
    intcode.run(&[]);

    println!("day2a: value at pos 0: {}", intcode.memory[0]);
}

fn day2b(state: &Vec<i64>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = state.clone();
            memory[1] = noun;
            memory[2] = verb;
            let mut intcode = Intcode::new(&memory);
            intcode.run(&[]);
            if intcode.memory[0] == 19690720 {
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
