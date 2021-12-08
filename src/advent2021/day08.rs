use std::collections::HashMap;

use fileutil;

pub fn run() {
    let lines = fileutil::read_lines("data/2021/08.txt").unwrap();
    let entries: Vec<(Vec<&str>, Vec<&str>)> = lines
        .iter()
        .map(|s| {
            let mut pipe_split= s.split("|");

            let signals: Vec<&str> = pipe_split.next().unwrap().trim().split(" ").collect();
            let outputs: Vec<&str> = pipe_split.next().unwrap().trim().split(" ").collect();

            (signals, outputs)
        }).collect();


    // num segments -> digit
    let mut segment_map: HashMap<u32, u32> = HashMap::new();
    segment_map.insert(2, 1);
    segment_map.insert(4, 4);
    segment_map.insert(3, 7);
    segment_map.insert(7, 8);

    let mut sum1478 = 0;
    for (signals, outputs) in entries {
        for output in outputs {
            if segment_map.contains_key(&(output.len() as u32)) {
                sum1478 += 1;
            }
        }
    }

    println!("{}", sum1478);
}