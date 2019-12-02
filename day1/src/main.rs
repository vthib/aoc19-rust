use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    day1a(&input)?;
    day1b(&input)?;
    Ok(())
}

fn compute_fuel_weight(weight: i32) -> i32 {
    weight / 3 - 2
}

fn compute_total_weight(mut weight: i32) -> i32 {
    let mut acc = 0;

    loop {
        weight = compute_fuel_weight(weight);

        if weight <= 0 {
            break;
        }
        acc += weight;
    }

    acc
}

fn day1a(input: &str) -> Result<()> {
    let mut total = 0;

    for line in input.lines() {
        let module = line.parse::<i32>()?;
        let fuel = compute_fuel_weight(module);

        total += fuel;
    }
    println!("day1a total weight: {}", total);
    Ok(())
}

fn day1b(input: &str) -> Result<()> {
    let mut total = 0;

    for line in input.lines() {
        let module = line.parse::<i32>()?;
        let fuel = compute_total_weight(module);

        total += fuel;
    }
    println!("day1b total weight: {}", total);
    Ok(())
}