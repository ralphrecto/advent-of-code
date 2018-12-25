use fileutil;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
struct Edge {
    src: char,
    dest: char
}

// Implementation of Kahn's algorithm for computing a topological sort.
fn kahns_algorithm(edges: &Vec<Edge>) -> Vec<char> {
    // (node -> list of neighbors with incoming edges into node)
    let mut adj: HashMap<char, Vec<char>> = HashMap::new();

    for Edge { src, dest } in edges.iter() {
        adj.entry(*src).or_insert(Vec::new());
        adj.entry(*dest).or_insert(Vec::new()).push(*src);
    }

    let mut topolist: Vec<char> = Vec::with_capacity(adj.len());
    let mut srcset: HashSet<char> = adj.iter()
        .filter(|(dest, srclist)| srclist.is_empty())
        .map(|(dest, _)| *dest)
        .collect();

//    L ← Empty list that will contain the sorted elements
//    S ← Set of all nodes with no incoming edge
//    while S is non-empty do
    //    remove a node n from S
    //    add n to tail of L
    //    for each node m with an edge e from n to m do
        //    remove edge e from the graph
        //    if m has no other incoming edges then
            //    insert m into S
//    if graph has edges then
//    return error   (graph has at least one cycle)
    //    else
//    return L   (a topologically sorted order)

    loop {
        println!("adj: {:?}", &adj);
        println!("srcset: {:?}", &srcset);
        if srcset.is_empty() {
            break;
        }

        let c: char = *srcset.iter().next().unwrap();
        srcset.remove(&c);
        topolist.push(c);

        // Insert neighbors of c into srcset, if possible.
        {
            let c_adj_set: &Vec<char> = adj.get(&c).unwrap();

            for c_neighbor in c_adj_set.iter() {
                if adj.get(c_neighbor).unwrap().is_empty() {
                    srcset.insert(*c_neighbor);
                }
            }
        }

        // Clear out the edges going into c.
        adj.get_mut(&c).unwrap().clear();
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

            println!("{:?}", kahns_algorithm(&edges));
        }
        Err(e) => {
            panic!(e);
        }
    }
}