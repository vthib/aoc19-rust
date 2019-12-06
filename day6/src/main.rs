use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // from star to its satellites
    let mut deps = HashMap::new();
    // from object to its star
    let mut revdeps = HashMap::new();
    for line in input.lines() {
        let names: Vec<&str> = line.split(')').collect();
        assert!(names.len() == 2);

        deps.insert(names[1], names[0]);

        let entry = revdeps.entry(names[0]).or_insert(Vec::new());
        entry.push(names[1]);
    }

    let mut sources = Vec::new();
    for obj in revdeps.keys() {
        if !deps.contains_key(obj) {
            sources.push(*obj);
        }
    }

    let levels = compute_graph_levels(&revdeps, &sources);

    day6a(&levels);
    day6b(&deps, &levels);
    Ok(())
}

fn compute_graph_levels<'a>(
    deps: &'a HashMap<&str, Vec<&str>>,
    sources: &'a Vec<&str>,
) -> HashMap<&'a str, u32> {
    let mut todo = sources.clone();
    let mut levels = HashMap::new();
    let mut level = 0;

    // Do a BFS and add distances to source
    while todo.len() > 0 {
        let mut next_todo = Vec::new();

        for obj in todo {
            levels.insert(obj, level);
            if let Some(children) = deps.get(obj) {
                for c in children {
                    next_todo.push(*c);
                }
            }
        }
        todo = next_todo;
        level += 1;
    }

    levels
}

fn day6a(levels: &HashMap<&str, u32>) {
    let total = levels.values().fold(0, |acc, v| acc + v);

    println!("day6a number of total orbits: {}", total);
}

fn day6b(deps: &HashMap<&str, &str>, levels: &HashMap<&str, u32>) {
    let mut visited = HashSet::new();

    // mark all the parents of "YOU"
    let mut obj = "YOU";
    while let Some(parent) = deps.get(obj) {
        visited.insert(parent);
        obj = parent;
    }

    // then do the same from "SAM". As soon as a visited node is
    // found, we have our path
    let mut obj = "SAN";
    while let Some(parent) = deps.get(obj) {
        if visited.contains(parent) {
            let dist_you = levels.get("YOU").unwrap() - levels.get(parent).unwrap() - 1;
            let dist_san = levels.get("SAN").unwrap() - levels.get(parent).unwrap() - 1;
            println!("day6b distance: {}", dist_you + dist_san);
            return;
        }
        obj = parent;
    }

    assert!(false);
}
