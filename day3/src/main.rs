use std::cmp::{max, min};
use std::io;
use std::io::Read;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<Vec<Movement>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<Movement>().unwrap())
                .collect()
        })
        .collect();
    assert_eq!(lines.len(), 2);

    // compute the bounds of the grid
    let mut bounds = Bounds::new();
    bounds.add_circuit(&lines[0]);
    bounds.add_circuit(&lines[1]);

    let mut map = bounds.into_map();
    map.mark_circuit(&lines[0]);

    day3a(&map, &lines[1]);
    day3b(&map, &lines[1]);
    Ok(())
}

// {{{ Movement

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Movement {
    direction: Direction,
    unit: u32,
}

impl FromStr for Movement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let c = s.chars().next().unwrap();
        let direction = match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("unknown movement {}", c),
        };

        Ok(Self {
            direction,
            unit: s.get(1..).unwrap().parse::<u32>()?,
        })
    }
}

// }}}
// {{{ Bounds

struct Bounds {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Bounds {
    pub fn new() -> Self {
        Self {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }

    pub fn add_circuit(&mut self, circuit: &Vec<Movement>) {
        let mut x_pos: i32 = 0;
        let mut y_pos: i32 = 0;

        for mov in circuit {
            match mov.direction {
                Direction::Up => y_pos += mov.unit as i32,
                Direction::Down => y_pos -= mov.unit as i32,
                Direction::Left => x_pos -= mov.unit as i32,
                Direction::Right => x_pos += mov.unit as i32,
            }
            self.x_min = min(self.x_min, x_pos);
            self.x_max = max(self.x_max, x_pos);
            self.y_min = min(self.y_min, y_pos);
            self.y_max = max(self.y_max, y_pos);
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (
            (self.x_max - self.x_min + 1) as usize,
            (self.y_max - self.y_min + 1) as usize,
        )
    }

    pub fn get_central_port_position(&self) -> Pos {
        Pos {
            x: -self.x_min,
            y: -self.y_min,
        }
    }

    pub fn into_map(&self) -> Map {
        let (w, h) = self.get_size();
        let cp = self.get_central_port_position();

        Map {
            grid: vec![0; w * h as usize],
            w,
            cp,
        }
    }
}

#[derive(Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn apply_movement<F>(&mut self, mov: &Movement, mut cb: F)
    where
        F: FnMut(&Self) -> (),
    {
        let offset: (i32, i32) = match mov.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..mov.unit {
            self.x = self.x + offset.0;
            self.y = self.y + offset.1;
            cb(&self)
        }
    }
}

struct Map {
    grid: Vec<u32>,
    w: usize,
    cp: Pos,
}

impl Map {
    fn at_mut(&mut self, pos: &Pos) -> &mut u32 {
        &mut self.grid[(pos.y as usize) * self.w + (pos.x as usize)]
    }

    fn at(&self, pos: &Pos) -> u32 {
        self.grid[(pos.y as usize) * self.w + (pos.x as usize)]
    }

    pub fn mark_circuit(&mut self, circuit: &Vec<Movement>) {
        let mut pos = self.cp.clone();
        let mut steps = 0;

        *self.at_mut(&pos) = 0;
        for mov in circuit {
            pos.apply_movement(mov, |pos| {
                steps += 1;
                *self.at_mut(pos) = steps;
            });
        }
    }

    fn distance_from_central_port(&self, pos: &Pos) -> i32 {
        (pos.x - self.cp.x).abs() + (pos.y - self.cp.y).abs()
    }

    fn find_min_distance(&self, circuit: &Vec<Movement>) -> Option<i32> {
        let mut pos = self.cp.clone();
        let mut min_distance = None;

        for mov in circuit {
            pos.apply_movement(mov, |pos| {
                if self.at(pos) > 0 {
                    let distance = self.distance_from_central_port(pos);
                    min_distance.replace(match min_distance {
                        None => distance,
                        Some(d) => min(d, distance),
                    });
                }
            });
        }
        min_distance
    }

    fn find_min_steps(&self, circuit: &Vec<Movement>) -> Option<u32> {
        let mut pos = self.cp.clone();
        let mut steps = 0;
        let mut min_steps = None;

        for mov in circuit {
            pos.apply_movement(mov, |pos| {
                steps += 1;
                if self.at(pos) > 0 {
                    let total_steps = steps + self.at(pos);
                    min_steps.replace(match min_steps {
                        None => total_steps,
                        Some(d) => min(d, total_steps),
                    });
                }
            });
        }
        min_steps
    }
}

fn day3a(map: &Map, circuit2: &Vec<Movement>) {
    let min_distance = map.find_min_distance(&circuit2);

    println!("day2a: minimal distance: {}", min_distance.unwrap());
}

fn day3b(map: &Map, circuit2: &Vec<Movement>) {
    let min_steps = map.find_min_steps(&circuit2);

    println!("day2a: minimal steps: {}", min_steps.unwrap());
}