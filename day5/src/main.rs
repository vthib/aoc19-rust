use intcode::Intcode;
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
    let mut intcode = Intcode::new(state);
    let output = intcode.run(&[1]);
    for i in 0..(output.len() - 1) {
        assert_eq!(output[i], 0);
    }
    println!("day5a output: {}", output[output.len() - 1]);
}

fn day5b(state: &Vec<i32>) {
    let mut intcode = Intcode::new(state);
    let output = intcode.run(&[5]);
    println!("day5b output: {:?}", output);
}