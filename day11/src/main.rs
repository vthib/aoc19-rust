use intcode::Intcode;
use std::io;
use std::io::Read;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut memory = Vec::new();
    for line in input.split(',') {
        memory.push(line.trim().parse::<i64>()?)
    }

    day11a(&memory);
    day11b(&memory);
    Ok(())
}

fn run_robot(state: &Vec<i64>, init_color: i64) -> HashMap<(i32, i32), i64> {
    let mut robot = Intcode::new(state);
    let mut painted = HashMap::new();
    let mut pos = (0, 0);
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut dir_pos = 1;

    painted.insert(pos.clone(), init_color);
    loop {
        // 1: white, 0: black
        let color = painted.get(&pos).unwrap_or(&0);
        let outputs = robot.run(&[*color]);

        painted.insert(pos.clone(), outputs[0]);
        if outputs[1] == 0 {
            dir_pos = (dir_pos + 1) % 4;
        } else {
            dir_pos = (dir_pos + 3) % 4;
        }
        pos.0 += DIRECTIONS[dir_pos].0;
        pos.1 += DIRECTIONS[dir_pos].1;

        if robot.is_done() {
            break;
        }
    }

    painted
}

fn day11a(state: &Vec<i64>)
{
    let painted = run_robot(state, 0);

    println!("painted number of cells: {}", painted.len());
}

struct Pos {
    x: i32,
    y: i32,
}

struct Bounds {
    min: Pos,
    max: Pos,
}

fn get_bounds(painted: &HashMap<(i32, i32), i64>) -> Bounds {
    let mut bounds = Bounds { min: Pos { x: 0, y: 0 }, max: Pos { x: 0, y: 0 } };

    for c in painted.keys() {
        bounds.min.x = std::cmp::min(bounds.min.x, c.0);
        bounds.max.x = std::cmp::max(bounds.max.x, c.0);
        bounds.min.y = std::cmp::min(bounds.min.y, c.1);
        bounds.max.y = std::cmp::max(bounds.max.y, c.1);
    }
    bounds
}

fn day11b(state: &Vec<i64>)
{
    let painted = run_robot(state, 1);

    let bounds = get_bounds(&painted);
    for x in bounds.min.x..(bounds.max.x+1) {
        for y in bounds.min.y..(bounds.max.y+1) {
            let color = painted.get(&(x, y)).unwrap_or(&0);
            print!("{}", if *color == 1 { 'X' } else { ' ' });
        }
        println!("");
    }
}