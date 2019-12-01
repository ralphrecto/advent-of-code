use fileutil;
use std::ascii::AsciiExt;
use std::time::SystemTime;
use std::collections::HashSet;

fn reacts(c1: char, c2: char) -> bool {
    return c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2;
}

fn react(chars: &Vec<char>) -> Vec<char> {
    let mut newchars: Vec<char> = Vec::with_capacity(chars.len());
    let mut prev: Option<char> = Option::None;
    for (idx, c) in chars.iter().enumerate() {
        match prev {
            Some(prev_char) => {
                if reacts(prev_char, *c) {
                    prev = Option::None;
                } else {
                    newchars.push(prev_char);
                    prev = Option::Some(*c);
                }
            }
            None => {
                prev = Option::Some(*c);
            }
        }
    }

    match prev {
        Some(prev_char) => {
            newchars.push(prev_char);
        }
        None => {}
    }

    return newchars;
}

fn react_fixedpoint(orig: &str) -> usize {
    let orig_initial_len = orig.len();
    let mut orig_chars: Vec<char> = orig.chars().collect();

    loop {
        let filtered: Vec<char> = react(&orig_chars);
        if (&filtered).len() == (&orig_chars).len() {
            break;
        }
        orig_chars = filtered;
    }

    return orig_chars.len();
}

fn part1(orig: &str) -> () {
    println!("part 1");
    println!("fully reacted length: {}", react_fixedpoint(orig));
}

fn part2(orig: &str) -> () {
    println!("part 2");

    let mut unitset: HashSet<char> = HashSet::new();
    for c in orig.chars() {
        unitset.insert(c.to_ascii_lowercase());
    }

    for unit in unitset.iter() {
        let filtered: String = orig.chars()
            .filter(|c| c.to_ascii_lowercase() != *unit)
            .collect();

        let fully_reacted_len = react_fixedpoint(&filtered);
        println!("filtering unit {}, fully reacted length: {}", unit, fully_reacted_len);
    }
}

pub fn run() -> () {
    match fileutil::read_file("./2018/data/05.txt") {
        Ok(orig) => {
            part1(&orig);
            part2(&orig);
        }
        Err(e) => println!("oh no!")
    }
}

pub fn timed() -> () {
    let init = SystemTime::now();
    run();
    let fin = SystemTime::now();
    let ttc = fin.duration_since(init).unwrap();
    println!("time to complete: {} secs and {} millis", ttc.as_secs(), ttc.subsec_nanos() /
        1000000);
}
