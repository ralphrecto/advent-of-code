use std::collections::{HashMap, HashSet};
use literal::{set, SetLiteral};
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

    // digit -> segments involved
    let mut digits_to_segment: HashMap<i32, HashSet<char>> = HashMap::new();
    digits_to_segment.insert(0, set!{'a', 'b', 'c', 'e', 'f', 'g'});
    digits_to_segment.insert(1, set!{'c', 'f'});
    digits_to_segment.insert(2, set!{'a', 'c', 'd', 'e', 'g'});
    digits_to_segment.insert(3, set!{'a', 'c', 'd', 'f', 'g'});
    digits_to_segment.insert(4, set!{'b', 'c', 'd', 'f'});
    digits_to_segment.insert(5, set!{'a', 'b', 'd', 'f', 'g'});
    digits_to_segment.insert(6, set!{'a', 'b', 'd', 'e', 'f', 'g'});
    digits_to_segment.insert(7, set!{'a', 'c', 'f'});
    digits_to_segment.insert(8, set!{'a', 'b', 'c', 'd', 'e', 'f', 'g'});
    digits_to_segment.insert(9, set!{'a', 'b', 'c', 'd', 'f', 'g'});

    // num segments -> digit
    let mut num_segments_to_chars: HashMap<i32, Vec<&HashSet<char>>> = HashMap::new();
    for (digit, segments) in &digits_to_segment {
        let num_segment_charsets = num_segments_to_chars
        .entry(segments.len() as i32)
        .or_insert_with(|| Vec::new());

        num_segment_charsets.push(digits_to_segment.get(digit).unwrap());
    }

    let mut sum1478 = 0;
    for (signals, outputs) in &entries {
        let mut num_segments_to_rotated_chars: HashMap<i32, Vec<HashSet<char>>> = HashMap::new();
        for signal in signals {
            let num_segment_charsets = num_segments_to_rotated_chars
            .entry(signal.len() as i32)
            .or_insert_with(|| Vec::new());

            let charset: HashSet<char> = signal.chars().collect();
            num_segment_charsets.push(charset);
        }

        for output in outputs {
            if num_segments_to_chars.get(&(output.len() as i32)).unwrap()[0].len() == 1 {
                sum1478 += 1;
            }

            let num_segment_charsets = num_segments_to_rotated_chars
            .entry(output.len() as i32)
            .or_insert_with(|| Vec::new());

            let charset: HashSet<char> = output.chars().collect();
            num_segment_charsets.push(charset);
        }

        println!("{:?}", num_segments_to_rotated_chars);
    }

    println!("Part 1: {}", sum1478);
}