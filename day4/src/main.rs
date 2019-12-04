use std::io;
use std::io::Read;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bounds: Vec<u32> = input
        .split('-')
        .map(|v| v.trim().parse::<u32>().unwrap())
        .collect();
    assert_eq!(bounds.len(), 2);

    day4a(bounds[0], bounds[1]);
    day4b(bounds[0], bounds[1]);
    Ok(())
}

fn has_same_adjacent_digits(mut val: u32, larger_group_allowed: bool) -> bool {
    let mut last_digit = None;
    let mut group_len = 1;

    while val > 0 {
        let digit = val % 10;

        if let Some(prev) = last_digit {
            if prev == digit {
                if larger_group_allowed {
                    return true;
                }
                group_len += 1;
            } else {
                if group_len == 2 {
                    return true;
                }
                group_len = 1;
            }
        }
        last_digit.replace(digit);
        val /= 10;
    }
    return group_len == 2;
}

fn has_increasing_digits(mut val: u32) -> bool {
    let mut last_digit = None;

    while val > 0 {
        let digit = val % 10;

        if let Some(prev) = last_digit {
            if prev < digit {
                return false;
            }
        }
        last_digit.replace(digit);
        val /= 10;
    }
    true
}

fn day4a(minval: u32, maxval: u32) {
    let mut nb_possible_passwords = 0;

    for val in minval..(maxval + 1) {
        if has_same_adjacent_digits(val, true) && has_increasing_digits(val) {
            nb_possible_passwords += 1;
        }
    }

    println!("day4a: nb possible passwords: {}", nb_possible_passwords);
}

fn day4b(minval: u32, maxval: u32) {
    let mut nb_possible_passwords = 0;

    for val in minval..(maxval + 1) {
        if has_same_adjacent_digits(val, false) && has_increasing_digits(val) {
            nb_possible_passwords += 1;
        }
    }

    println!("day4b: nb possible passwords: {}", nb_possible_passwords);
}