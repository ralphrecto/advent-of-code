use fileutil;
use itertools::Itertools;

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

fn sim(steps: usize) -> u32 {
    let mut energy_levels: Vec<Vec<u32>> = fileutil::read_lines("data/2021/11-tmp.txt")
        .unwrap()
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut buffer = energy_levels.clone();
   
    for level in energy_levels.iter() {
        println!("{:?}", level);
    }
    println!("");

    let mut num_flashes: u32 = 0;
    for _ in 0..steps {

        // Energy increase phase
        for i in 0..energy_levels.len() {
            for j in 0..energy_levels[0].len() {
                buffer[i][j] = buffer[i][j] + 1;

                if buffer[i][j] > 9 {
                    for (adj_i, adj_j) in adjacent(&energy_levels, i, j) {
                        // println!("i {}, j {}, add_i {}, adj_j {}", i, j, adj_i, adj_j);
                        buffer[adj_i][adj_j] += 1;
                    }
                }
            }
        }

        for level in energy_levels.iter() {
            println!("{:?}", level);
        }
        println!("flash");

        // Flash phase
        for i in 0..energy_levels.len() {
            for j in 0..energy_levels[0].len() {
                if buffer[i][j] > 9 {
                    num_flashes += 1;
                    buffer[i][j] = 0;
                }
            }
        }

        // Copy buffer phase
        for i in 0..energy_levels.len() {
            for j in 0..energy_levels[0].len() {
                energy_levels[i][j] = buffer[i][j];
            }
        }

        let tmp = energy_levels;
        energy_levels = buffer;
        buffer = tmp;
    }

    for level in energy_levels.iter() {
        println!("{:?}", level);
    }
    // println!("buffer");

    // for level in buffer.iter() {
    //     println!("{:?}", level);
    // }
    println!("");

    num_flashes
}

pub fn run() {
    println!("{}", sim(2));
}