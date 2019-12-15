use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;
use scan_fmt::scan_fmt;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let planets: Vec<_> = input.lines().map(|line| {
        line.parse::<Planet>().unwrap()
    }).collect();

    day12a(planets.clone());
    day12b(&planets);
    Ok(())
}

fn day12a(mut planets: Vec<Planet>) {
    const TLAST: u32 = 1000;

    for _ in 0..TLAST {
        advance_time(&mut planets);
    }

    let total_energy = planets.iter().fold(0, |acc, p| acc + p.get_energy());
    println!("day12a total energy: {}", total_energy);
}

fn day12b(planets: &Vec<Planet>) {
    let xloop_len = compute_cycle_len(planets, &Coord::X);
    let yloop_len = compute_cycle_len(planets, &Coord::Y);
    let zloop_len = compute_cycle_len(planets, &Coord::Z);

    let loop_len = lcm(xloop_len, lcm(yloop_len, zloop_len));

    println!("day12b cycle len: {}", loop_len);
}

enum Coord {
    X,
    Y,
    Z
}

fn compute_cycle_len(initial_planets: &Vec<Planet>, coord: &Coord) -> u64 {
    let mut planets_slow = initial_planets.clone();
    let mut planets_fast = initial_planets.clone();

    loop {
        // advance planets2 twice as fast as planets
        // planets2 will loop and end up on the same
        // state as planets eventually
        advance_time(&mut planets_slow);
        advance_time(&mut planets_fast);
        advance_time(&mut planets_fast);
        if on_same_state(&planets_slow, &planets_fast, coord) {
            break;
        }
    }

    // compute the size of the loop
    planets_fast = initial_planets.clone();
    let mut loop_size = 0;
    loop {
        advance_time(&mut planets_fast);
        loop_size += 1;
        if on_same_state(&planets_slow, &planets_fast, coord) {
            break;
        }
    }

    loop_size
}

fn on_same_state(planets: &Vec<Planet>, planets2: &Vec<Planet>, coord: &Coord) -> bool {
    !planets.iter().zip(planets2).any(|(a, b)| {
        match coord {
            Coord::X => a.position.x != b.position.x || a.velocity.x != b.velocity.x,
            Coord::Y => a.position.y != b.position.y || a.velocity.y != b.velocity.y,
            Coord::Z => a.position.z != b.position.z || a.velocity.z != b.velocity.z,
        }
    })
}

fn advance_time(planets: &mut Vec<Planet>) {
    update_velocities(planets);
    for planet in planets.iter_mut() {
        planet.position.add(&planet.velocity);
    }
}

fn update_velocities(planets: &mut Vec<Planet>) {
    for i in 0..planets.len() {
        for j in (i+1)..planets.len() {
            let g = compute_gravity(&planets[i].position, &planets[j].position);
            planets[i].velocity.add(&g);
            planets[j].velocity.sub(&g);
        }
    }
}

fn compute_gravity(a: &Vec3, b: &Vec3) -> Vec3 {
    let x = compute_gravity_on_axis(a.x, b.x);
    let y = compute_gravity_on_axis(a.y, b.y);
    let z = compute_gravity_on_axis(a.z, b.z);
    Vec3 { x, y, z }
}

fn compute_gravity_on_axis(a: i32, b: i32) -> i32 {
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={:2}, y={:2}, z={:2}", self.x, self.y, self.z)
    }
}

impl Vec3 {
    fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn sub(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    fn get_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, PartialEq)]
struct Planet {
    position: Vec3,
    velocity: Vec3,
}

impl Planet {
    fn get_energy(&self) -> i32 {
        self.position.get_energy() * self.velocity.get_energy()
    }
}

impl FromStr for Planet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y, z) = scan_fmt!(s, "<x={d}, y={d}, z={d}>", i32, i32, i32)?;

        Ok(Planet {
            position: Vec3 { x, y, z },
            velocity: Vec3 { x: 0, y: 0, z: 0 },
        })
    }
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos=<{}>, vel=<{}>", self.position, self.velocity)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        while a != 0 {
            let t = a;
            a = b % a;
            b = t;
        }
        b
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}