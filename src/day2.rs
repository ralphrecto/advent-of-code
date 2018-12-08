use fileutil;
use std::collections::HashMap;

pub fn run() -> () {
    match fileutil::read_lines("./data/02.txt") {
        Ok(lines) => {
            let mut twice: u32 = 0;
            let mut thrice: u32 = 0;

            for line in &lines {
                let mut char_count: HashMap<char, u32> = HashMap::new();
                for char in line.chars() {
                    *char_count.entry(char).or_insert(0) += 1;
                }

                let mut has_twice = false;
                let mut has_thrice = false;
                for count in char_count.values() {
                    has_twice |= *count == 2;
                    has_thrice |= *count == 3;
                }

                twice += if has_twice { 1 } else { 0 };
                thrice += if has_thrice { 1 } else { 0 };
            }

            println!("checksum: {}", twice * thrice);
        }
        Err(e) => println!("{}", e)
    }
}