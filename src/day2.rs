use fileutil;
use std::collections::HashSet;

pub fn run() -> () {
    match fileutil::read_lines("./data/01.txt") {
        Ok(lines) => {
            let mut seen: HashSet<i32> = HashSet::new();
            let mut rolling: i32 = 0;
            loop {
                let mut triggered: bool = false;
                for line in &lines {
                    let num: i32 = line.parse().unwrap();
                    rolling += num;
                    if (seen.contains(&rolling)) {
                        println!("{}", rolling);
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
        Err(e) => println!("{}", e)
    }
}

