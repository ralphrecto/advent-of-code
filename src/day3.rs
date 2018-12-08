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

fn within(x: i32, y: i32, claim: &Claim) -> bool {
    claim.origin_x < x &&
        claim.origin_y < y &&
        (claim.origin_x + claim.width) >= x &&
        (claim.origin_y + claim.height) >= y
}

pub fn run() -> () {
    match fileutil::read_lines("./data/03.txt") {
        Ok(lines) => {
            let claims: Vec<Claim> = lines.iter()
                .map(|line| parse_line(line))
                .collect();

            let mut intersecting_sq = 0;
            for x in 0..1000 {
                for y in 0..1000 {
                    let mut num_claims_containing = 0;

                    for claim in &claims  {
                        num_claims_containing += if within(x, y, claim) { 1 } else { 0 };
                        if num_claims_containing > 1 {
                            intersecting_sq += 1;
                            break;
                        }
                    }
                }
            }

            println!("intersecting sq: {}", intersecting_sq);
        }
        Err(e) => panic!(e)
    }
}
