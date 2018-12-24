use fileutil;
use std::ascii::AsciiExt;
use std::time::SystemTime;

fn reacts(c1: char, c2: char) -> bool {
    return c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2;
}

fn filter_reactions(chars: &Vec<char>) -> Vec<char> {
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

pub fn run() -> () {
    match fileutil::read_file("./data/05.txt") {
        Ok(orig) => {
            let orig_initial_len = orig.len();
            let mut orig_chars: Vec<char> = orig.chars().collect();

            loop {
                let filtered: Vec<char> = filter_reactions(&orig_chars);
                if (&filtered).len() == (&orig_chars).len() {
                    break;
                }
                orig_chars = filtered;
            }

            println!(
                "orig len: {}\nfinal len: {}",
                orig_initial_len,
                orig_chars.len()
            );
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
