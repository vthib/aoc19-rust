use intcode::Intcode;
use itertools::Itertools;
use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut memory = Vec::new();
    for line in input.split(',') {
        memory.push(line.trim().parse::<i64>()?)
    }

    day7a(&memory);
    day7b(&memory);
    Ok(())
}

fn run_amps(amps: &mut Vec<Intcode>, inputs: &[i64]) -> Vec<i64> {
    assert_eq!(amps.len(), 5);
    let out = amps[0].run(inputs);
    let out = amps[1].run(&out);
    let out = amps[2].run(&out);
    let out = amps[3].run(&out);
    let out = amps[4].run(&out);
    out
}

fn new_amps(state: &Vec<i64>, phases: &[&i64]) -> Vec<Intcode> {
    let mut amps = vec![Intcode::new(state); 5];
    for i in 0..5 {
        amps[i].run(&[*phases[i]]);
        assert!(!amps[i].is_done());
    }
    amps
}

fn day7a(state: &Vec<i64>) {
    let mut max_output = None;

    for phases in [0, 1, 2, 3, 4].iter().permutations(5) {
        let mut amps = new_amps(state, &phases);
        let out = run_amps(&mut amps, &[0]);

        assert_eq!(out.len(), 1);
        max_output.replace(match max_output {
            Some(v) => std::cmp::max(v, out[0]),
            None => out[0],
        });
    }

    println!("day7a maximum output: {}", max_output.unwrap());
}

fn day7b(state: &Vec<i64>) {
    let mut max_output = None;

    for phases in [5, 6, 7, 8, 9].iter().permutations(5) {
        let mut amps = new_amps(state, &phases);
        let mut input = Vec::new();
        input.push(0);
        while !amps.iter().any(|a| a.is_done()) {
            let out = run_amps(&mut amps, &input);
            input = out;
        }

        assert_eq!(input.len(), 1);
        max_output.replace(match max_output {
            Some(v) => std::cmp::max(v, input[0]),
            None => input[0],
        });
    }

    println!("day7b maximum output: {}", max_output.unwrap());
}
