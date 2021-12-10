use std::collections::HashSet;

use fileutil;

fn get_adjacent(heightmap: &Vec<Vec<i32>>, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let deltas: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let (pos_y, pos_x) = pos;

    let adj: HashSet<(usize, usize)> = deltas.iter()
        .map(|(dy, dx)| (pos_y as i32 + dy, pos_x as i32 + dx))
        .filter(|(y, x)|
            0 <= *y && *y < heightmap.len() as i32 &&
            0 <= *x && *x < heightmap[0].len() as i32
        ).map(|(y, x)| (y as usize, x as usize))
        .collect();

    adj
}

pub fn run() {
    let heightmap: Vec<Vec<i32>> = fileutil::read_lines("data/2021/09.txt")
        .unwrap()
        .iter()
        .map(|line|
            line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32).collect()
        ).collect();

    let mut sum_risk_levels = 0;
    let mut lowpoints: Vec<(usize, usize)> = vec![];
    for i in 0..heightmap.len() as i32 {
        for j in 0..heightmap[0].len() as i32 {
            let cur_height = heightmap[i as usize][j as usize];

            let is_lowpoint = get_adjacent(&heightmap, (i as usize, j as usize))
                .iter()
                .all(|(y, x)| heightmap[*y][*x] > cur_height);

            if is_lowpoint {
                let risk_level = cur_height + 1;
                sum_risk_levels += risk_level;

                lowpoints.push((i as usize, j as usize));
            }
        }
    }

    let mut stack: Vec<(usize, usize)> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut basin_sizes: Vec<i32> = Vec::new();
    for lowpoint in lowpoints {
        let mut basin_size = 0;
        stack.push(lowpoint);

        while !stack.is_empty() {
            let (cur_y, cur_x) = stack.pop().unwrap();
            if visited.contains(&(cur_y, cur_x)) {
                continue;
            }

            basin_size += 1;
            visited.insert((cur_y, cur_x));

            let cur_height = heightmap[cur_y][cur_x];

            let adj = get_adjacent(&heightmap, (cur_y, cur_x));
            for (adj_y, adj_x) in adj.difference(&visited) {
                let adj_height = heightmap[*adj_y][*adj_x];
                if  cur_height < adj_height && adj_height != 9 {
                    stack.push((*adj_y, *adj_x));
                }
            }
        }

        basin_sizes.push(basin_size);
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    let basin_mult = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

    println!("{}", sum_risk_levels);
    println!("{}", basin_mult);
}