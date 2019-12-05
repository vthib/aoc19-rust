use std::io;
use std::io::Read;
use intcode;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut memory = Vec::new();

    for line in input.split(',') {
        memory.push(line.trim().parse::<i32>()?)
    }

    day2a(&memory);
    day2b(&memory);
    Ok(())
}

fn day2a(state: &Vec<i32>) {
    let mut memory = state.clone();
    memory[1] = 12;
    memory[2] = 2;
    let res = intcode::run(&memory, &[]);

    println!("day2a: value at pos 0: {}", res);
}

fn day2b(state: &Vec<i32>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = state.clone();
            memory[1] = noun;
            memory[2] = verb;
            let res = intcode::run(&memory, &[]);
            if res == 19690720 {
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
