use fileutil;
use std::collections::HashSet;

fn pt1(lines: &Vec<String>) -> () {
    let sum: i32 = lines
        .iter()
        .map(|x: &String| {
            let n: i32 = x.parse().unwrap();
            return n;
        }).fold(0, |acc, x| acc + x);

    println!("pt 1: {}", sum);
}

fn pt2(lines: &Vec<String>) -> () {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut rolling: i32 = 0;
    loop {
        let mut triggered: bool = false;
        for line in lines {
            let num: i32 = line.parse().unwrap();
            rolling += num;
            if (seen.contains(&rolling)) {
                println!("pt2: {}", rolling);
                triggered = true;
                break;
            } else {
                seen.insert(rolling);
            }
        }

        if (triggered) {
            break;
        }
    }
}

pub fn run() -> () {
    match fileutil::read_lines("./data/01.txt") {
        Ok(lines) => {
            pt1(&lines);
            pt2(&lines);
        }
        Err(e) => println!("{}", e)
    }
}