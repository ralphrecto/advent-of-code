use fileutil;
use std::collections::HashMap;

pub fn run() -> () {
    match fileutil::read_lines("./2018/data/02.txt") {
        Ok(lines) => {
            // pt1.
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

            // pt2.
            for i in 0..lines.len() {
                for j in (i+1)..lines.len() {
                    let istr = &lines[i];
                    let jstr = &lines[j];

                    let mut diff_count = 0;
                    let mut last_diff_index = None;
                    for (idx, (ic, jc)) in istr.chars().zip(jstr.chars()).enumerate() {
                        if ic != jc {
                            diff_count += 1;
                            if diff_count > 1 {
                                break;
                            }

                            last_diff_index = Some(idx);
                        }
                    }

                    match (last_diff_index, diff_count) {
                        (Some(idx), 1) => {
                            let s: String = istr.chars()
                                .enumerate()
                                .filter(|&(c_idx, c)| idx != c_idx)
                                .map(|(_, c)| c)
                                .collect();

                            println!("pt2: {}", s);
                        }
                        _ => {}
                    }
                }
            }
        }
        Err(e) => println!("{}", e)
    }
}
