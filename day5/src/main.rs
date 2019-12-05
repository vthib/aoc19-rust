use intcode;
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

fn day5a(state: &Vec<i32>) {
    let mut memory = state.clone();
    println!("day5a run...");
    intcode::run(&mut memory, &[1]);
}

fn day5b(state: &Vec<i32>) {
    let mut memory = state.clone();
    println!("day5b run...");
    intcode::run(&mut memory, &[5]);
}