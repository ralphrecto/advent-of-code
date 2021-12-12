use std::collections::{HashMap, HashSet};

use fileutil;
use itertools::Itertools;

enum SmallCaveMode {
    SingleVisit,
    DoubleVisit
}

fn has_lower_char_at_bound(path: &Vec<&str>, bound: u32) -> bool {
    let mut charmap: HashMap<&str, u32> = HashMap::new();
    for p in path {
        *charmap.entry(p).or_insert(0) += 1;
    }

    charmap.values().any(|v| *v == bound)
}

fn at_end(path: &Vec<&str>) -> bool {
    path.last().expect("fatal error: expecting non-empty path").eq(&"end")
}

fn find_paths(mode: SmallCaveMode) -> u32 {
    let lines = fileutil::read_lines("data/2021/12-tmp.txt").unwrap();
    let edge_strs: Vec<(&str,&str)> = lines.iter()
        .map(|l| {
            let mut splits = l.split("-");
            (
                splits.next().unwrap().trim(),
                splits.next().unwrap().trim()
            )
        }).collect();

    
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (node_a, node_b) in edge_strs {
        edges.entry(node_a)
            .or_insert_with(|| HashSet::new())
            .insert(node_b);

        edges.entry(node_b)
            .or_insert_with(|| HashSet::new())
            .insert(node_a);
    }

    let neighbor_bound = match mode {
        SmallCaveMode::SingleVisit => 1,
        SmallCaveMode::DoubleVisit => 2
    };
    let mut paths = vec![vec!["start"]];
    loop {
        let mut all_done = true;
        let mut new_paths: Vec<Vec<&str>> = vec![];

        for path in paths.iter_mut() {
            let current_node = *path.last().expect("Fatal error: empty path");
            if current_node.eq("end") {
                continue;
            }

            let path_has_char_at_bound= has_lower_char_at_bound(path, neighbor_bound);
            println!("path {:?}, bound {}", path, path_has_char_at_bound);

            let adjacent = edges.get(current_node).expect("Fatal error: no edges for current node");
            let valid_adjacents: Vec<&str> = adjacent.iter()
                .filter(|neighbor| {
                    let is_lower = neighbor.chars().all(char::is_lowercase);
                    let neighbor_count = path.iter()
                        .filter(|p| p == neighbor)
                        .count() as u32;

                    let upper_ok = !is_lower;
                    let lower_ok = **neighbor != "start" && neighbor_count < neighbor_bound;
                    let bound_ok = neighbor_count + 1 < neighbor_bound || !path_has_char_at_bound;

                    upper_ok || (lower_ok && bound_ok)
                }).map(|s| *s)
                .collect();

            for (index, valid_adjacent) in valid_adjacents.iter().enumerate().rev() {
                if index == 0 {
                    path.push(*valid_adjacent);
                } else {
                    let mut path_clone= path.clone();
                    path_clone.push(*valid_adjacent);
                    new_paths.push(path_clone);
                }
            }

            all_done &= valid_adjacents.is_empty() || at_end(path);
        }

        if all_done && new_paths.is_empty() {
            break;
        }

        paths.extend(new_paths);
    }

    let mut num_valid_paths = 0;
    let mut str_paths: HashSet<String> = HashSet::new();
    for path in paths {
        if at_end(&path) {
            str_paths.insert(path.join(","));
        }
    }

    let mut sorted_paths = str_paths.iter().sorted();
    // sorted_paths.reverse();
    for path in sorted_paths {
        println!("{}", path);
    }

    str_paths.len() as u32
}

pub fn run() {
    // println!("{}", find_paths(SmallCaveMode::SingleVisit));
    println!("{}", find_paths(SmallCaveMode::DoubleVisit));
}