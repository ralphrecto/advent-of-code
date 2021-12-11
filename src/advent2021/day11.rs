use std::collections::HashSet;

use fileutil;
use itertools::Itertools;

#[derive(PartialEq,Eq)]
enum Mode {
    NUM_FLASHES,
    FIRST_SYNC
}

fn adjacent<'a>(energy_levels: &'a Vec<Vec<u32>>, row: usize, col: usize) -> Box<dyn Iterator<Item=(usize, usize)> + 'a> {
    let iter = (-1..=1).cartesian_product(-1..=1)
        .map(move |(di, dj)| {
            let row_cand = di + row as i32;
            let col_cand = dj + col as i32;

            (row_cand as usize, col_cand as usize)
        }).filter(move |(row_cand, col_cand)| {
            *row_cand >= 0 && *row_cand < energy_levels.len() &&
            *col_cand >= 0 && *col_cand < energy_levels[0].len() &&
            (*row_cand != row || *col_cand != col)
        });

    Box::new(iter)
}

fn sim(steps: usize, mode: Mode) -> u32 {
    let mut energy_levels: Vec<Vec<u32>> = fileutil::read_lines("data/2021/11.txt")
        .unwrap()
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let height= energy_levels.len();
    let width = energy_levels[0].len();
    let mut buffer = energy_levels.clone();
   
    let mut num_total_flashes: u32 = 0;
    for step_num in 1..=steps {
        // Energy increase phase
        for i in 0..height {
            for j in 0..width {
                buffer[i][j] = buffer[i][j] + 1;
            }
        }

        // Adjacent cells increase phase.
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        loop {
            let mut fixed = true;
            for i in 0..height {
                for j in 0..width {
                    if buffer[i][j] > 9 && !seen.contains(&(i, j)) {
                        seen.insert((i, j));
                        fixed = false;
                        for (adj_i, adj_j) in adjacent(&energy_levels, i, j) {
                            buffer[adj_i][adj_j] += 1;
                        }
                    }
                }
            }

            if fixed {
                break;
            }
        }

        // Flash phase
        let mut num_step_flashes: u32 = 0;
        for i in 0..height {
            for j in 0..width {
                if buffer[i][j] > 9 {
                    num_total_flashes += 1;
                    num_step_flashes += 1;
                    buffer[i][j] = 0;
                }
            }
        }

        if num_step_flashes == (height * width) as u32 && mode == Mode::FIRST_SYNC {
            return step_num as u32;
        }

        // Copy buffer phase
        for i in 0..height {
            for j in 0..width {
                energy_levels[i][j] = buffer[i][j];
            }
        }

        let tmp = energy_levels;
        energy_levels = buffer;
        buffer = tmp;
    }

    match mode {
        Mode::NUM_FLASHES => num_total_flashes,
        Mode::FIRST_SYNC => panic!("Did not find first sync time")
    }
}

pub fn run() {
    println!("{}", sim(usize::MAX, Mode::FIRST_SYNC));
}