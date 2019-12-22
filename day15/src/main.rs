use intcode::Intcode;
use std::fmt;
use std::io;
use std::io::Read;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let state: Vec<_> = input
        .split(',')
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect();

    let mut grid = Grid::new();
    day15a(&state, &mut grid);
    day15b(&mut grid);
    Ok(())
}

const DROID_X: usize = 25;
const DROID_Y: usize = 25;

fn day15a(state: &Vec<i64>, grid: &mut Grid) {
    let mut stack = Vec::new();

    *grid.at_mut(DROID_X, DROID_Y) = TileType::Empty;
    grid.set_distance(DROID_X, DROID_Y, 0);
    stack.push(Continuation {
        pgm: Intcode::new(state),
        x: DROID_X,
        y: DROID_Y,
    });

    while let Some(c) = stack.pop() {
        let distance = grid.get_distance(c.x, c.y) + 1;
        run_continuation(&c.pgm, c.x, c.y - 1, 1, distance, grid, &mut stack);
        run_continuation(&c.pgm, c.x, c.y + 1, 2, distance, grid, &mut stack);
        run_continuation(&c.pgm, c.x - 1, c.y, 3, distance, grid, &mut stack);
        run_continuation(&c.pgm, c.x + 1, c.y, 4, distance, grid, &mut stack);
    }
    println!("{}", grid);
    let oxygen_pos = grid.get_oxygen_position();
    println!(
        "day15a: distance from oxygen system: {}",
        grid.get_distance(oxygen_pos.0, oxygen_pos.1)
    );
}

fn run_continuation(
    pgm: &Intcode,
    x: usize,
    y: usize,
    input: i64,
    distance: u32,
    grid: &mut Grid,
    conts: &mut Vec<Continuation>,
) {
    match grid.at(x, y) {
        TileType::Wall | TileType::OxygenSystem => return,
        TileType::Empty => {
            if grid.get_distance(x, y) <= distance {
                return;
            }
        }
        TileType::Unknown => (),
    }

    let mut pgm = pgm.clone();
    let output = pgm.run(&[input]);

    *grid.at_mut(x, y) = match output[0] {
        0 => TileType::Wall,
        1 => TileType::Empty,
        2 => TileType::OxygenSystem,
        _ => panic!("unknown output {}", output[0]),
    };
    grid.set_distance(x, y, distance);
    if output[0] == 1 {
        conts.push(Continuation { pgm, x, y })
    }
    if output[0] == 2 {
        grid.set_oxygen_position(x, y);
    }
}

struct Continuation {
    pgm: Intcode,
    x: usize,
    y: usize,
}

fn day15b(grid: &mut Grid) {
    for x in 0..grid.w {
        for y in 0..grid.h {
            grid.set_distance(x, y, std::u32::MAX);
        }
    }

    let mut stack = Vec::new();
    let mut distance = 0;
    let oxygen_pos = grid.get_oxygen_position();
    stack.push((oxygen_pos.0, oxygen_pos.1));
    while stack.len() > 0 {
        let mut new_stack = Vec::new();

        while let Some((x, y)) = stack.pop() {
            grid.set_distance(x, y, distance);
            add_empty_tile(x, y - 1, distance + 1, grid, &mut new_stack);
            add_empty_tile(x, y + 1, distance + 1, grid, &mut new_stack);
            add_empty_tile(x - 1, y, distance + 1, grid, &mut new_stack);
            add_empty_tile(x + 1, y, distance + 1, grid, &mut new_stack);
        }
        std::mem::swap(&mut stack, &mut new_stack);
        distance += 1;
    }

    println!("day15b: distance max from oxygen system: {}", distance - 1);
}

fn add_empty_tile(
    x: usize,
    y: usize,
    distance: u32,
    grid: &mut Grid,
    stack: &mut Vec<(usize, usize)>,
) {
    if grid.get_distance(x, y) <= distance {
        return;
    }

    match grid.at(x, y) {
        TileType::Empty => stack.push((x, y)),
        _ => (),
    }
}

#[derive(Clone)]
enum TileType {
    Unknown,
    Empty,
    Wall,
    OxygenSystem,
}

struct Grid {
    map: Vec<TileType>,
    distance: Vec<u32>,
    w: usize,
    h: usize,
    oxygen_x: usize,
    oxygen_y: usize,
}

impl Grid {
    fn new() -> Self {
        let w = 50;
        let h = 50;
        Grid {
            map: vec![TileType::Unknown; w * h],
            distance: vec![std::u32::MAX; w * h],
            w,
            h,
            oxygen_x: 0,
            oxygen_y: 0,
        }
    }

    fn at(&self, x: usize, y: usize) -> &TileType {
        &self.map[y * self.w + x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut TileType {
        &mut self.map[y * self.w + x]
    }

    fn get_distance(&self, x: usize, y: usize) -> u32 {
        self.distance[y * self.w + x]
    }

    fn set_distance(&mut self, x: usize, y: usize, distance: u32) {
        self.distance[y * self.w + x] = distance;
    }

    fn get_oxygen_position(&mut self) -> (usize, usize) {
        (self.oxygen_x, self.oxygen_y)
    }

    fn set_oxygen_position(&mut self, x: usize, y: usize) {
        self.oxygen_x = x;
        self.oxygen_y = y;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                if x == DROID_X && y == DROID_Y {
                    write!(f, "D")?;
                } else {
                    write!(
                        f,
                        "{}",
                        match *self.at(x, y) {
                            TileType::Unknown => ' ',
                            TileType::Empty => '.',
                            TileType::Wall => '#',
                            TileType::OxygenSystem => 'O',
                        }
                    )?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
