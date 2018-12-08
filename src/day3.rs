use fileutil;
use std::collections::HashMap;

#[derive(Debug)]
struct Claim {
    id: i32,
    origin_x: i32,
    origin_y: i32,
    width: i32,
    height: i32
}

fn parse_line(line: &String) -> Claim {
    let id_and_coords: Vec<&str> = line
        .split("@")
        .map(|x| x.trim())
        .collect();

    let id: i32 = id_and_coords[0][1..].parse().unwrap();

    let origin_and_dim: Vec<&str> = id_and_coords[1]
        .split(":")
        .map(|x| x.trim())
        .collect();

    let origin: Vec<i32> = origin_and_dim[0]
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let dim: Vec<i32> = origin_and_dim[1]
        .split("x")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    return Claim {
        id: id,
        origin_x: origin[0],
        origin_y: origin[1],
        width: dim[0],
        height: dim[1]
    };
}

pub fn run() -> () {
    match fileutil::read_lines("./data/03.txt") {
        Ok(lines) => {
            let claims: Vec<Claim> = lines.iter()
                .map(|line| parse_line(line))
                .collect();

            // General idea: do pairwise intersection to generate all possible intersecting claims.
            // Then, union all pairwise intersections.
        }
        Err(e) => panic!(e)
    }
}
