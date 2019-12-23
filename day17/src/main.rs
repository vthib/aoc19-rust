use intcode::Intcode;
use std::io;
use std::io::Read;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut memory = Vec::new();
    for line in input.split(',') {
        memory.push(line.trim().parse::<i64>()?)
    }

    day17a(&memory);
    memory[0] = 2;
    day17b(&memory);
    Ok(())
}

fn day17a(memory: &Vec<i64>) {
    let mut pgm = Intcode::new(memory);
    let output: Vec<char> = pgm.run(&[]).iter().map(|i| *i as u8 as char).collect();

    // fill a grid with positions of the scaffold
    let grid = Grid::new(&output);

    // find intersections
    let mut acc = 0;
    for y in 0..grid.h {
        for x in 0..grid.w {
            if grid.is_scaffold(x, y) {
                let up = y > 0 && grid.is_scaffold(x, y - 1);
                let down = y < (grid.h - 1) && grid.is_scaffold(x, y + 1);
                let left = x > 0 && grid.is_scaffold(x - 1, y);
                let right = x < (grid.w - 1) && grid.is_scaffold(x + 1, y);
                let vert = up && down;
                let hori = left && right;
                if (vert && (left || right)) || (hori && (up || down)) {
                    acc += x * y;
                }
            }
        }
    }

    println!("{}", output.iter().collect::<String>());
    println!("day17a: alignement parameter: {}", acc);
}

fn day17b(memory: &Vec<i64>) {
    let input: Vec<i64> = [
        "A,B,A,C,B,C,B,C,A,C",
        "R,12,L,10,R,12",
        "L,8,R,10,R,6",
        "R,12,L,10,R,10,L,8",
        "n",
        "",
    ]
    .join("\n")
    .chars()
    .map(|v| v as i64)
    .collect();

    let mut pgm = Intcode::new(&memory);
    let out = pgm.run(&input);
    println!("day17b: {}", out[out.len() - 1]);
}

struct Grid {
    map: Vec<bool>,
    w: usize,
    h: usize,
}

impl Grid {
    fn new(output: &Vec<char>) -> Self {
        let (w, h) = get_bounds(&output);
        let mut map = vec![false; w * h];

        for y in 0..h {
            for x in 0..w {
                let c = output[y * w + x];
                match c {
                    '#' => map[y * w + x] = true,
                    _ => (),
                }
            }
        }
        Grid { map, w, h }
    }

    fn is_scaffold(&self, x: usize, y: usize) -> bool {
        self.map[y * self.w + x]
    }
}

fn get_bounds(output: &Vec<char>) -> (usize, usize) {
    let w = output.iter().position(|c| *c == '\n').unwrap();

    (w + 1, output.len() / (w + 1))
}
