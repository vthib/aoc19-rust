use std::io;
use std::io::Read;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    day16a(input.clone());
    day16b(&input);
    Ok(())
}

fn day16a(mut input: Vec<u32>) {
    for _ in 0..100 {
        let mut new_input = (0..input.len()).map(|idx| apply_fft(&input, idx)).collect();
        std::mem::swap(&mut input, &mut new_input);
    }
    println!("day16a: {}", vec_to_u32(&input[0..8]));
}

fn day16b(orig: &Vec<u32>) {
    let full_len = orig.len() * 10000;

    let offset = vec_to_u32(&orig[0..7]);
    let rest_len = full_len - offset;
    let mut input = Vec::new();
    let mut i = 1;
    for _ in 0..rest_len {
        input.push(orig[orig.len() - i]);
        i = if i == orig.len() { 1 } else { i + 1 };
    }

    for _ in 0..100 {
        let mut acc = 0;
        for v in input.iter_mut() {
            acc += *v;
            *v = acc % 10;
        }
    }
    println!("day16b: {:?}", vec_to_u32(input.iter().rev().take(8)));
}

fn apply_fft(input: &Vec<u32>, lvl: usize) -> u32 {
    const FILTER: [i32; 4] = [0, 1, 0, -1];

    input
        .iter()
        .enumerate()
        .fold(0, |acc: i32, (idx, v)| {
            acc + (*v as i32) * FILTER[((idx + 1) / (lvl + 1)) % 4]
        })
        .abs() as u32
        % 10
}

fn vec_to_u32<'a, T>(tab: T) -> usize
where
    T: IntoIterator<Item = &'a u32>,
{
    tab.into_iter().fold(0, |acc, v| acc * 10 + (*v as usize))
}
