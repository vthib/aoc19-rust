use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // split input into layers;
    let input: Vec<_> = input.trim().chars().collect();
    let mut layers = Vec::new();
    let nb_layers = input.len() / (WIDTH * HEIGHT);
    for i in 0..nb_layers {
        layers.push(&input[(i * WIDTH * HEIGHT)..((i + 1) * WIDTH * HEIGHT)]);
    }

    day8a(&layers);
    day8b(&layers);
    Ok(())
}

fn count_digits(layer: &[char], digit: char) -> usize {
    layer
        .iter()
        .fold(0, |acc, c| if *c == digit { acc + 1 } else { acc })
}

fn find_layer_fewest_zeroes<'a>(layers: &'a Vec<&'a [char]>) -> &'a [char] {
    let mut min_idx = 0;
    let mut min = WIDTH * HEIGHT;

    for (idx, layer) in layers.iter().enumerate() {
        let nb_zeroes = count_digits(layer, '0');
        if nb_zeroes < min {
            min = nb_zeroes;
            min_idx = idx;
        }
    }

    &layers[min_idx]
}

fn day8a(layers: &Vec<&[char]>) {
    let layer = find_layer_fewest_zeroes(layers);

    let nb_ones = count_digits(layer, '1');
    let nb_twos = count_digits(layer, '2');

    println!("day8a answer: {}", nb_ones * nb_twos);
}

fn day8b(layers: &Vec<&[char]>) {
    let mut image = vec!['2'; WIDTH * HEIGHT];

    for layer in layers {
        for (idx, c) in layer.iter().enumerate() {
            if image[idx] == '2' {
                image[idx] = *c;
            }
        }
    }

    println!("day8b image...");
    for (idx, c) in image.iter().enumerate() {
        if idx % WIDTH == 0 {
            println!("");
        }
        if *c == '0' {
            print!(" ");
        } else {
            print!("X");
        }
    }
    println!("");
}