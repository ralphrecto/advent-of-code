use fileutil;
use std::collections::{HashMap, HashSet, BTreeSet};

#[derive(Clone, Copy)]
struct Edge {
    src: char,
    dest: char
}

// Implementation of Kahn's algorithm for computing a topological sort.
fn kahns_algorithm(edges: &Vec<Edge>) -> Vec<char> {
    // (node -> list of neighbors with incoming edges into node)
    let mut adj: HashMap<char, HashSet<char>> = HashMap::new();

    for Edge { src, dest } in edges.iter() {
        adj.entry(*src).or_insert(HashSet::new());
        adj.entry(*dest).or_insert(HashSet::new()).insert(*src);
    }

    let mut topolist: Vec<char> = Vec::with_capacity(adj.len());
    let mut candset: BTreeSet<char> = adj.iter()
        .filter(|(dest, srclist)| srclist.is_empty())
        .map(|(dest, _)| *dest)
        .collect();

    loop {
        if candset.is_empty() {
            break;
        }

        let c: char = *candset.iter().next().unwrap();
        candset.remove(&c);
        topolist.push(c);

        for (_, srcset) in adj.iter_mut() {
            srcset.remove(&c);
        }

        for (dest, srcset) in adj.iter() {
            if srcset.is_empty() &&
                c != *dest &&
                !candset.contains(dest) &&
                !topolist.contains(dest)
            {
                candset.insert(*dest);
            }
        }
    }

    return topolist;
}

pub fn run() -> () {
    match fileutil::read_lines("./data/07.txt") {
        Ok(lines) => {
            let edges: Vec<Edge> = lines.iter()
                .map(|line| {
                    let charvec: Vec<char> = line.chars().collect();
                    Edge { src: charvec[5], dest: charvec[36] }
                }).collect();

            println!("{:?}", kahns_algorithm(&edges).iter().collect::<String>());
        }
        Err(e) => {
            panic!(e);
        }
    }
}