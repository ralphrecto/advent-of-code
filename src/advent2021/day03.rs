use fileutil;
use bits;
use std::collections::HashSet;

fn part1(lines: &Vec<String>) {
    let num_bits = lines[0].len();
    let mut counter: Vec<i32> = vec![0; num_bits];
    for line in lines {
        for (i, char) in line.chars().enumerate() {
            counter[i] += match char { '1' => 1, _ => 0 };
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for bitindex in 0..num_bits {
        let bit = 1 << (num_bits - 1 - bitindex);
        if counter[bitindex] > (lines.len() as i32 - counter[bitindex]) {
            gamma |= bit;
        } else {
            epsilon |= bit;
        }
    }

    println!("{}", gamma * epsilon);
}

fn part2(lines: &Vec<String>, use_most_common: bool) -> i32 {
    let num_bits = lines[0].len();

    let mut remain: HashSet<usize> = HashSet::new();
    for i in 0..lines.len() {
        remain.insert(i);
    }

    for bitindex in 0..num_bits {
        if remain.len() == 1 {
            break;
        }

        let mut counter: i32 = 0;
        for line_index in &remain {
            counter += match lines[*line_index].as_bytes()[bitindex] as char {
                '1' => 1,
                _ => 0
            };
        }

        let num_non_keep = remain.len() as i32 - counter;
        // use 2 to indicate tie
        let most_common= if counter > num_non_keep {
            '1'
        } else if counter == num_non_keep {
            '2'
        } else {
            '0'
        };

        let keep_char = if use_most_common {
            if most_common == '2' {
                '1'
            } else {
                most_common
            }
        } else {
            if most_common == '2' || most_common == '1' {
                '0'
            } else {
                '1'
            }
        };

        let mut new_remain: HashSet<usize> = HashSet::new();
        for line_index in &remain {
            let cur_char = lines[*line_index].as_bytes()[bitindex] as char;
            if keep_char == cur_char {
                new_remain.insert(*line_index);
            }
        }

        remain = new_remain;
    }
    
    let generator = &lines[*remain.iter().nth(0).unwrap()];
    bits::bitstring_to_i32(generator)
}

pub fn run() {
    let lines = fileutil::read_lines("data/2021/03.txt").unwrap();
    part1(&lines);

    let oxygen = part2(&lines, true);
    let co2 = part2(&lines, false);

    println!("{}", oxygen * co2);
}