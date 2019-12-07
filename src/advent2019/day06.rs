use fileutil;
use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::str::Split;

fn transitive_orbits(object: &str, orbit_map: &HashMap<&str, HashSet<&str>>) -> u32 {
    match orbit_map.get(object) {
        Some(orbitees) => {
            let mut orbits: u32 = orbitees.len() as u32;
            for orbitee in orbitees {
                orbits += transitive_orbits(orbitee, orbit_map);
            }

            return orbits;
        },
        None => 0
    }
}

pub fn part1(lines: &Vec<String>) -> u32 {
    // object -> direct orbitees
    let mut orbitee_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let parsed_orbits = lines.iter()
        .map(|l| {
            let mut split: Split<&str> = l.split(")");
            return (split.next().unwrap(), split.next().unwrap());
        });

    for (orbitee, orbiter) in parsed_orbits {
        orbitee_map
            .entry(orbiter)
            .or_insert(HashSet::new())
            .insert(orbitee);
    }

    let mut total_orbits: u32 = 0;
    for object in orbitee_map.keys() {
        total_orbits += transitive_orbits(object, &orbitee_map);
    }

    return total_orbits;
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/06.txt") {
        Ok(lines) => {
            println!("{}", part1(&lines));
        }
        Err(e) => println!("{}", e)
    }
}