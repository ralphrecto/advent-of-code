use fileutil;
use std::collections::{HashMap, HashSet, BTreeSet};
use std::u32;

#[derive(Clone, Copy)]
struct Edge {
    src: char,
    dest: char
}

// Implementation of Kahn's algorithm for computing a topological sort.
fn kahns_algorithm(adj_orig: &HashMap<char, HashSet<char>>) -> Vec<char> {
    let mut adj = adj_orig.clone();
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

fn assign(workers: &mut Vec<Option<(char, u32)>>, task: char) -> () {
    let is_assigned = workers.iter()
        .any(|worker| {
            match *worker {
                Some((worker_task, _)) => worker_task == task,
                None => false
            }
        });

    if is_assigned {
        return;
    }

    for worker in workers.iter_mut() {
        match *worker {
            Some(_) => { }
            None => {
                let work = compute_work(task);
                *worker = Some((task, work));
                return;
            }
        }
    }
}

fn simulate_work(workers: &mut Vec<Option<(char, u32)>>) -> (HashSet<char>, u32) {
    let mut ticks_to_adv = u32::MAX;
    for worker in workers.iter() {
        match *worker {
            Some((task, worker_work)) => {
                ticks_to_adv = ticks_to_adv.min(worker_work);
            }
            None => { }
        }
    }

    if ticks_to_adv == u32::MAX {
        return (HashSet::new(), 0);
    }

    let mut completed: HashSet<char> = HashSet::new();
    for worker in workers.iter_mut() {
        match *worker {
            Some((task, worker_work)) => {
                if ticks_to_adv >= worker_work {
                    *worker = None;
                    completed.insert(task);
                } else {
                    *worker = Some((task, worker_work - ticks_to_adv));
                }
            }
            None => { }
        }
    }

    (completed, ticks_to_adv)
}

fn compute_work(task: char) -> u32 {
    let a_digit = 'a' as u32;
    let task_digit = (task.to_ascii_lowercase()) as u32;
    task_digit - a_digit + 1 + 60
}

fn compute_total_work(
    adj: &HashMap<char, HashSet<char>>,
    topolist: &Vec<char>,
    num_workers: usize) -> u32 {
    // Each index represents a worker, each value represents ticks left on current work.
    let mut workers: Vec<Option<(char, u32)>> = Vec::with_capacity(num_workers);
    workers.resize(num_workers, Option::None);
    let mut total_ticks = 0;
    let mut complete: HashSet<char> = HashSet::new();

    let mut ticks = 0;
    loop {
        if complete.len() == topolist.len() {
            break;
        }

        let (just_completed, ticks_adv) = simulate_work(&mut workers);
        for just_completed_task in just_completed.iter() {
            complete.insert(*just_completed_task);
        }
        total_ticks += ticks_adv;

        for task in topolist {
            match adj.get(task) {
                Some(deps) => {
                    if !complete.contains(task) && deps.difference(&complete).count() == 0 {
                        assign(&mut workers, *task);
                    }
                }
                None => {
                    println!("no deps set! :{}", task)
                }
            }
        }
    }

    total_ticks
}

pub fn run() -> () {
    match fileutil::read_lines("./data/07.txt") {
        Ok(lines) => {
            let edges: Vec<Edge> = lines.iter()
                .map(|line| {
                    let charvec: Vec<char> = line.chars().collect();
                    Edge { src: charvec[5], dest: charvec[36] }
                }).collect();

            let mut adj: HashMap<char, HashSet<char>> = HashMap::new();

            for Edge { src, dest } in edges.iter() {
                adj.entry(*src).or_insert(HashSet::new());
                adj.entry(*dest).or_insert(HashSet::new()).insert(*src);
            }

            let topolist: Vec<char> = kahns_algorithm(&adj);
            println!("part 1.");
            println!("topological sort: {:?}", topolist.iter().collect::<String>());

            println!("part 2.");
            println!("total work: {}", compute_total_work(&adj, &topolist, 5));
        }
        Err(e) => {
            panic!(e);
        }
    }
}