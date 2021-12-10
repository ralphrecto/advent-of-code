use std::collections::{HashMap, HashSet};
use literal::{set, SetLiteral};
use bigint::U256;

use fileutil;

fn opening_char(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("unknown char {}", c)
    }
}

fn closing_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unknown char {}", c)
    }
}

fn error_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unknown char {}", c)
    }
}

fn autocomplete_score(c: char) -> U256 {
    let score = match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unknown char {}", c)
    };

    U256::from(score)
}

pub fn run() {
    let lines = fileutil::read_lines("data/2021/10.txt").unwrap();

    let closing_chars: HashSet<char> = set!{')', ']', '}', '>'};

    let mut stack: Vec<char> = Vec::new();
    let mut tot_error_score = 0;
    let mut autocomplete_scores: Vec<U256> = Vec::new();
    'lineloop: for line in &lines {
        stack.drain(..);

        for c in line.chars() {
            if closing_chars.contains(&c) {
                let open_c = opening_char(c);
                let popped = stack.pop().unwrap();

                if popped != open_c {
                    tot_error_score += error_score(c);
                    continue 'lineloop;
                }
            } else {
                stack.push(c);
            }
        }

        let mut cur_autocomplete_score: U256 = U256::zero();
        while !stack.is_empty() {
            let popped = stack.pop().unwrap();
            let score = autocomplete_score(closing_char(popped));

            cur_autocomplete_score = cur_autocomplete_score * U256::from(5) + score;
        }

        autocomplete_scores.push(cur_autocomplete_score);
    }

    autocomplete_scores.sort();
    let autocomplete_median = autocomplete_scores[(autocomplete_scores.len() / 2) as usize];

    println!("{}", tot_error_score);
    println!("{}", autocomplete_median);
}