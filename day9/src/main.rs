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

    day9a(&memory);
    day9b(&memory);
    Ok(())
}

fn day9a(state: &Vec<i64>) {
    let mut intcode = Intcode::new(state);
    let output = intcode.run(&[1]);

    println!("day9a answer: {:?}", output);
}

fn day9b(state: &Vec<i64>) {
    let mut intcode = Intcode::new(state);
    let output = intcode.run(&[2]);

    println!("day9b answer: {:?}", output);
}