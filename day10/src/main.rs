use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut map: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().map(|c| c == '#').collect());
    }

    let mut asteroids = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, is_asteroid) in line.iter().enumerate() {
            if *is_asteroid {
                asteroids.push(Pos { x: x as i32, y: y as i32 });
            }
        }
    }

    let base = find_base(&map, &asteroids);
    day10b(&map, &asteroids, &base);
    Ok(())
}

fn get_movement_vector(a: &Pos, b: &Pos) -> (i32, i32) {
    (b.x - a.x, b.y - a.y)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a == 0 {
        b.abs()
    } else if b == 0 {
        a.abs()
    } else {
        while a != 0 {
            let t = a;
            a = b % a;
            b = t;
        }
        b.abs()
    }
}

fn normalize_vector(vec: &mut (i32, i32)) {
    let c = gcd(vec.0, vec.1);
    vec.0 /= c;
    vec.1 /= c;
}

fn get_vision_rank(map: &Vec<Vec<bool>>, from: &Pos, to: &Pos, mut vec: (i32, i32)) -> u32 {
   let mut at = from.clone();
   let mut rank = 1;

   normalize_vector(&mut vec);
   loop {
       at.x += vec.0;
       at.y += vec.1;
       if at == *to {
           return rank;
       } else if map[at.y as usize][at.x as usize] {
           rank += 1;
       }
   }
}

fn is_visible(map: &Vec<Vec<bool>>, from: &Pos, to: &Pos, vec: (i32, i32)) -> bool {
    get_vision_rank(map, from, to, vec) == 1
}

fn find_base(map: &Vec<Vec<bool>>, asteroids: &Vec<Pos>) -> Pos {
    let mut max_visible = 0;
    let mut best_base = None;

    for base in asteroids {
        let mut visible = 0;

        for asteroid in asteroids {
            // compute the movement vector
            let vec = get_movement_vector(base, asteroid);
            if base == asteroid {
                // asteroid == base
                continue;
            }

            // go from base in direction of vector, until we reach the
            // asteroid. If we end up on an asteroid that isn't "asteroid",
            // it means it is masked
            if is_visible(map, base, asteroid, vec) {
                visible += 1;
            }
        }

        if visible > max_visible {
            max_visible = visible;
            best_base = Some(base.clone());
        }
    }

    // for every asteroid, compute vector
    println!("day10a maximum number of visible asteroids: {:?}", max_visible);
    best_base.unwrap()
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct PolarVecToAsteroid {
    rank: u32,
    theta: i32,
    asteroid_pos: usize,
}

fn day10b(map: &Vec<Vec<bool>>, asteroids: &Vec<Pos>, base: &Pos)
{
    let mut vecs = Vec::new();

    for (pos, asteroid) in asteroids.iter().enumerate() {
        if asteroid == base {
            continue;
        }
        let vec = get_movement_vector(base, asteroid);

        let mut theta = ((-vec.1 as f64).atan2(vec.0 as f64) * 18000. / std::f64::consts::PI) as i32;

        // theta is direction east, counter clockwise, ]-18000, 18000].
        // Normalize it to [-9000, 45000], direction north, clock wise.
        theta = -theta;
        if theta < -9000 {
            theta += 36000;
        }

        // compute rank of vision from base
        let rank = get_vision_rank(map, base, asteroid, vec);

        vecs.push(PolarVecToAsteroid {
            theta,
            rank,
            asteroid_pos: pos
        });
    }

    // sort vecs according to (theta, r2)
    vecs.sort();

    let selected = &asteroids[vecs[199].asteroid_pos];
    println!("day10b 200th asteroid is: {}", selected.x * 100 + selected.y);
}