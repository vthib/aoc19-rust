use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let recipes: Vec<_> = input
        .lines()
        .map(|line| line.parse::<Recipe>().unwrap())
        .collect();

    let recipes_map: HashMap<_, _> = recipes
        .iter()
        .map(|r| {
            (
                r.product.element.to_owned(),
                ValCompound {
                    ingredients: &r.ingredients,
                    product_count: r.product.count,
                },
            )
        })
        .collect();

    day14a(&recipes_map);
    day14b(&recipes_map);
    Ok(())
}

fn day14a(recipes_map: &HashMap<String, ValCompound>) {
    let mut rest: HashMap<&str, u64> = HashMap::new();

    produce_fuel(recipes_map, 1, false, &mut rest);
    println!(
        "day14a number of ore required: {}",
        rest.get("ORE").unwrap()
    );
}

fn day14b(recipes_map: &HashMap<String, ValCompound>) {
    let mut min = 1;
    let mut max = 2;

    // find an upper bound on the number of fuel that
    // can be produced
    while can_produce_fuel(recipes_map, max) {
        min = max;
        max = min * 2;
    }

    // do a binary search
    loop {
        if max - min < 2 {
            break;
        }
        let m = min + (max - min) / 2;
        if can_produce_fuel(recipes_map, m) {
            min = m;
        } else {
            max = m;
        }
    }
    println!("day14b number of fuel produced: {}", min);
}

fn can_produce_fuel(recipes_map: &HashMap<String, ValCompound>, fuel_qty: u64) -> bool {
    const GOAL: u64 = 1000000000000;
    let mut rest: HashMap<&str, u64> = HashMap::new();

    rest.insert("ORE", GOAL);
    produce_fuel(recipes_map, fuel_qty, true, &mut rest)
}

fn produce_fuel<'r, 'm: 'r>(
    recipes_map: &'m HashMap<String, ValCompound>,
    fuel_qty: u64,
    limited_ore: bool,
    rest: &mut HashMap<&'r str, u64>,
) -> bool {
    let mut want: Vec<(&str, u64)> = Vec::new();

    want.push(("FUEL", fuel_qty));
    while let Some((name, mut qty)) = want.pop() {
        match recipes_map.get(name) {
            Some(vc) => {
                if let Some(rest_qty) = rest.get_mut(name) {
                    if qty <= *rest_qty {
                        *rest_qty -= qty;
                        qty = 0;
                    } else {
                        qty -= *rest_qty;
                        *rest_qty = 0;
                    }
                }
                if qty != 0 {
                    let factor = (qty + vc.product_count - 1) / vc.product_count;
                    for c in vc.ingredients {
                        want.push((&c.element, c.count * factor));
                    }
                    let r = vc.product_count * factor - qty;
                    if r != 0 {
                        *rest.entry(name).or_insert(0) += r;
                    }
                }
            }
            None => {
                if limited_ore {
                    let ore_qty = rest.get_mut(name).unwrap();
                    if *ore_qty < qty {
                        return false;
                    } else {
                        *ore_qty -= qty;
                    }
                } else {
                    *rest.entry(name).or_insert(0) += qty;
                }
            }
        }
    }
    true
}

struct ValCompound<'a> {
    ingredients: &'a Vec<Compound>,
    product_count: u64,
}

struct Compound {
    element: String,
    count: u64,
}

struct Recipe {
    ingredients: Vec<Compound>,
    product: Compound,
}

impl FromStr for Recipe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }
        let mut compounds: Vec<_> = RE
            .captures_iter(s)
            .map(|cap| Compound {
                element: cap[2].to_owned(),
                count: cap[1].parse::<u64>().unwrap(),
            })
            .collect();
        let product = compounds.pop().unwrap();

        Ok(Recipe {
            ingredients: compounds,
            product,
        })
    }
}
