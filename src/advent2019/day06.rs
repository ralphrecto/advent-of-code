extern crate petgraph;

use fileutil;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::iter::Map;
use std::str::Split;
use self::petgraph::{Graph, Undirected};
use self::petgraph::graph::NodeIndex;
use self::petgraph::algo::dijkstra;

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

fn parse_orbits(lines: &Vec<String>, undirected: bool) -> HashMap<&str, HashSet<&str>> {
    // object -> direct orbitees
    let mut orbit_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let parsed_orbits = lines.iter()
        .map(|l| {
            let mut split: Split<&str> = l.split(")");
            return (split.next().unwrap(), split.next().unwrap());
        });

    for (orbitee, orbiter) in parsed_orbits {
        orbit_map
            .entry(orbiter)
            .or_insert(HashSet::new())
            .insert(orbitee);

        if undirected {
            orbit_map
                .entry(orbitee)
                .or_insert(HashSet::new())
                .insert(orbiter);
        }        
    }

    return orbit_map;
}

pub fn part1(lines: &Vec<String>) -> u32 {
    let orbitee_map: HashMap<&str, HashSet<&str>> = parse_orbits(lines, false);

    let mut total_orbits: u32 = 0;
    for object in orbitee_map.keys() {
        total_orbits += transitive_orbits(object, &orbitee_map);
    }

    return total_orbits;
}

fn goto_target(start: &str, target: &str, orbitee_map: &HashMap<&str, HashSet<&str>>) -> u32 {
    let mut unique_nodes: HashSet<&str> = HashSet::new();
    unique_nodes.insert(start);
    unique_nodes.insert(target);

    for key in orbitee_map.keys() {
        unique_nodes.insert(key);
    }

    for orbitees in orbitee_map.values() {
        for orbitee in orbitees {
            unique_nodes.insert(orbitee);
        }
    }

    let mut object_index: HashMap<&str, NodeIndex<u32>> = HashMap::new();
    let mut graph: Graph<&str, i32, Undirected> = Graph::new_undirected();
    for node in &unique_nodes {
        object_index.insert(node, graph.add_node(node));
    }

    for node in &unique_nodes {
        let node_idx = object_index.get(node).unwrap();

        match orbitee_map.get(node) {
            Some(orbitees) => {
                for orbitee in orbitees {
                    let orbitee_idx = object_index.get(orbitee).unwrap();
                    graph.add_edge(*node_idx, *orbitee_idx, 1);
                }
            },
            None => ()
        }
    }

    let start_idx = object_index.get(start).unwrap();
    let target_idx = object_index.get(target).unwrap();

    let path_costs: HashMap<NodeIndex<u32>, u32> = dijkstra(&graph, *start_idx, Some(*target_idx), |e| 1);

    return *path_costs.get(target_idx).unwrap();
}

pub fn part2(lines: &Vec<String>) -> u32 {
    let orbitee_map: HashMap<&str, HashSet<&str>> = parse_orbits(lines, true);
    let current = orbitee_map.get("YOU").unwrap().iter().next().unwrap();
    let target = orbitee_map.get("SAN").unwrap().iter().next().unwrap();

    return goto_target(current, target, &orbitee_map);
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/06.txt") {
        Ok(lines) => {
            println!("{}", part2(&lines));
        }
        Err(e) => println!("{}", e)
    }
}