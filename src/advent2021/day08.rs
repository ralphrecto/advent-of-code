use std::{collections::{HashMap, HashSet}, process::Output};
use literal::{set, SetLiteral};
use fileutil;

fn add_if_not_in(
    num_segments_to_rotated_chars: &mut HashMap<i32, Vec<HashSet<char>>>,
    output: &str
) {
    let num_segment_charsets = num_segments_to_rotated_chars
    .entry(output.len() as i32)
    .or_insert_with(|| Vec::new());

    let charset: HashSet<char> = output.chars().collect();
    let already_in = num_segment_charsets
        .iter()
        .any(|existing_charset| existing_charset.eq(&charset));

    if !already_in {
        num_segment_charsets.push(charset);
    }
}


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

    let mut segment_str_to_digit: HashMap<String, i32> = HashMap::new();
    for (digit, segment) in digits_to_segment.iter() {
        let mut segment_str_vec: Vec<char> = segment.iter().map(|c| *c).collect();
        segment_str_vec.sort();

        let segment_str: String = segment_str_vec.iter().collect();
        segment_str_to_digit.insert(segment_str, *digit);
    }

    let mut num_segments_to_chars: HashMap<i32, Vec<&HashSet<char>>> = HashMap::new();
    for (digit, segments) in &digits_to_segment {
        let num_segment_charsets = num_segments_to_chars
        .entry(segments.len() as i32)
        .or_insert_with(|| Vec::new());

        num_segment_charsets.push(digits_to_segment.get(digit).unwrap());
    }

    let mut sum1478 = 0;
    let mut output_sums = 0;
    for (signals, outputs) in &entries {
        let mut num_segments_to_rotated_chars: HashMap<i32, Vec<HashSet<char>>> = HashMap::new();
        for signal in signals {
            add_if_not_in(&mut num_segments_to_rotated_chars, signal);
        }

        for output in outputs {
            if num_segments_to_chars.get(&(output.len() as i32)).unwrap()[0].len() == 1 {
                sum1478 += 1;
            }

            add_if_not_in(&mut num_segments_to_rotated_chars, output);
        }

        let mut segment_cand_map: HashMap<char, HashSet<char>> = HashMap::new();

        // 7 - 1
        let seven_chars = &num_segments_to_rotated_chars.get(&3).unwrap()[0];
        let one_chars = &num_segments_to_rotated_chars.get(&2).unwrap()[0];

        let seven_minus_one = seven_chars.difference(one_chars).map(|c| *c).collect::<HashSet<char>>();
        segment_cand_map.insert('a', seven_minus_one);

        // 4 - 1
        let four_chars = &num_segments_to_rotated_chars.get(&4).unwrap()[0];
        let four_minus_one = four_chars.difference(one_chars).map(|c| *c).collect::<HashSet<char>>();

        segment_cand_map.insert('b', four_minus_one.clone());
        segment_cand_map.insert('d', four_minus_one);

        // 1
        segment_cand_map.insert('c', one_chars.clone());
        segment_cand_map.insert('f', one_chars.clone());

        // 8 - 4 - 7
        let eight_chars = &num_segments_to_rotated_chars.get(&7).unwrap()[0];
        let eight_min_four: HashSet<char> = eight_chars.difference(four_chars).map(|c| *c).collect();
        let eight_min_four_min_seven: HashSet<char> = eight_min_four.difference(seven_chars).map(|c| *c).collect();

        segment_cand_map.insert('e', eight_min_four_min_seven.clone());
        segment_cand_map.insert('g',eight_min_four_min_seven);

        let mut candsets: Vec<HashMap<char, char>> = vec![HashMap::new()];
        for (segment, cands) in segment_cand_map {

            let mut new_candsets: Vec<HashMap<char, char>> = Vec::new();
            for cand in cands {
                for candset in candsets.iter() {
                    if !candset.contains_key(&cand) {
                        let mut candset_clone = candset.clone();
                        candset_clone.insert(cand, segment);
                        new_candsets.push(candset_clone);
                    }
                }
            }

            candsets = new_candsets;
        }

        let mut valid_candsets: Vec<HashMap<char, char>> = Vec::new();
        for candset in candsets {
            // let unique_vals: HashSet<char> = candset.keys().map(|c| *c).collect();

            // // candset maps maps multiple segments to one, which is invalid
            // if unique_vals.len() < 7 {
            //     continue;
            // }

            let mapped_signals: Vec<HashSet<char>> = signals
                .iter()
                .map(|charset|
                    charset.chars()
                        .map(|c| *candset.get(&c).unwrap())
                        .collect()
                ).collect();

            let mapped_all_real = mapped_signals
                .iter()
                .all(|mapped_signal|
                    digits_to_segment
                    .values()
                    .any(|segment| segment.eq(mapped_signal))
                );

            if mapped_all_real {
                valid_candsets.push(candset);
            }

        }

        assert!(valid_candsets.len() == 1);
        let valid_candset = &valid_candsets[0];

        println!("{:?}", valid_candset);
        let output: i32 = outputs
            .iter()
            .map(|output| {
                println!("{}", output);
                let mut mapped_output_vec: Vec<char> = output.chars()
                .map(|c| *valid_candset.get(&c).unwrap())
                .collect();
                mapped_output_vec.sort();

                let mapped_output: String = mapped_output_vec.iter().collect();

                return *segment_str_to_digit.get(&mapped_output).unwrap();
            }).rev()
            .enumerate()
            .map(|(i, digit)| 10i32.pow(i as u32) * digit)
            .sum();

        output_sums += output;
    }

    // println!("Part 1: {}", sum1478);
    println!("Part 2: {}", output_sums);
}