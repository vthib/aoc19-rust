use intcode::Intcode;
use std::fmt;
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

    let grid = Grid::new(&memory);
    day13a(&grid);
    day13b(memory, grid);
    Ok(())
}

struct Grid {
    tab: Vec<TileType>,
    w: usize,
    h: usize,
    score: i64,
    ball_pos_x: i32,
    paddle_pos_x: i32,
}

impl Grid {
    fn new(memory: &Vec<i64>) -> Self {
        let output = Intcode::new(memory).run(&[]);
        let (w, h) = get_bounds(&output);
        let tab = vec![TileType::Empty; w*h];

        let mut s = Self {
            tab,
            w,
            h,
            score: -1,
            ball_pos_x: 0,
            paddle_pos_x: 0,
        };
        s.fill(&output);
        s
    }

    fn fill(&mut self, output: &Vec<i64>) {
        let mut idx = 0;

        while idx < output.len() {
            let x = output[idx];
            let y = output[idx + 1];
            let t = output[idx + 2];
            idx += 3;

            if x == -1 && y == 0 {
                self.score = t; 
                continue;
            }

            let typ = match t {
                0 => TileType::Empty,
                1 => TileType::Wall,
                2 => TileType::Block,
                3 => {
                    self.paddle_pos_x = x as i32;
                    TileType::Paddle
                },
                4 => { 
                    self.ball_pos_x = x as i32;
                    TileType::Ball
                },
                _ => panic!("invalid tile type {}", t),
            };

            *(self.at_mut(x as usize, y as usize)) = typ;
        }
    }

    fn at(&self, x: usize, y: usize) -> &TileType {
        &self.tab[y*self.w + x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut TileType {
        &mut self.tab[y*self.w + x]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                write!(f, "{}", self.at(x, y))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Empty => ' ',
            Self::Wall  => 'W',
            Self::Block => 'O',
            Self::Paddle => '_',
            Self::Ball  => 'X',
        })
    }
}

fn get_bounds(output: &Vec<i64>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;

    let mut idx = 0;
    while idx < output.len() {
        max_x = std::cmp::max(max_x, output[idx] as usize);
        max_y = std::cmp::max(max_y, output[idx + 1] as usize);
        idx += 3;
    }

    (max_x + 1, max_y + 1)
}

fn day13a(grid: &Grid) {
    let nb_blocks = grid.tab.iter().fold(0, |acc, t| {
        match *t {
            TileType::Block => acc + 1,
            _ => acc,
        }
    });
    println!("day13a: nb blocks: {}", nb_blocks);
}

fn day13b(mut memory: Vec<i64>, mut grid: Grid) {
    memory[0] = 2;
    let mut program = Intcode::new(&memory);

    let mut input = 1;
    loop {
        let output = program.run(&[input]);
        grid.fill(&output);
        println!("{}", grid);

        if program.is_done() {
            break;
        }

        input = if grid.paddle_pos_x < grid.ball_pos_x {
            1
        } else if grid.paddle_pos_x > grid.ball_pos_x {
            -1
        } else {
            0
        };

        //let time = std::time::Duration::from_millis(5);
        //std::thread::sleep(time);
    }

    println!("day13b: score: {}", grid.score);
}