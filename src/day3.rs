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

fn intersects(a: &Claim, b: &Claim) -> bool {
    let b_left = b.origin_x + 1;
    let b_top = b.origin_y + 1;
    let b_right = b.origin_x + b.width;
    let b_bottom = b.origin_y + b.height;

    within(b_left, b_top, a) ||
        within(b_right, b_top, a) ||
        within(b_left, b_bottom, a) ||
        within(b_right, b_bottom, a)
}

fn pt1(claims: &Vec<Claim>) -> () {
    let mut intersecting_sq = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let mut num_claims_containing = 0;

            for claim in claims.iter() {
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

fn pt2(claims: &Vec<Claim>) -> () {
    for a in claims {
        let mut clear = true;
        for b in claims {
            if a.id != b.id && intersects(a, b) {
                println!("{:?}, {:?}", a, b);
                clear = false;
                break;
            }
        }

        if clear {
//            println!("{:?}", a);
        }
    }
}

#[cfg(test)]
mod tests {
    use day3::*;

    #[test]
    fn intersect_test() {
        assert!(
            intersects(
                &Claim { id: 1, origin_x: 0, origin_y: 0, width: 5, height: 5 },
                &Claim { id: 2, origin_x: 1, origin_y: 1, width: 6, height: 6 }
            )
        );

        assert!(
            !intersects(
                &Claim { id: 1, origin_x: 0, origin_y: 0, width: 3, height: 3 },
                &Claim { id: 2, origin_x: 3, origin_y: 3, width: 3, height: 3 }
            )
        );
    }
}

pub fn run() -> () {
    match fileutil::read_lines("./data/03.txt") {
        Ok(lines) => {
            let claims: Vec<Claim> = lines.iter()
                .map(|line| parse_line(line))
                .collect();

//            pt1(&claims);
            pt2(&claims);
        }
        Err(e) => panic!(e)
    }
}
