use fileutil;

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

fn pt1(grid: &mut [[i32; 1000]; 1000], claims: &Vec<Claim>) -> () {
    let mut intersecting_sq = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if grid[y][x] == -1 {
                intersecting_sq += 1;
            }
        }
    }

    println!("intersecting sq: {}", intersecting_sq);
}

fn pt2(grid: &mut [[i32; 1000]; 1000], claims: &Vec<Claim>) -> () {
    let claim_intact = |claim: &Claim| {
        for y in claim.origin_y..(claim.origin_y + claim.height) {
            for x in claim.origin_x..(claim.origin_x + claim.width) {
                let ux = x as usize;
                let uy = y as usize;
                if grid[uy][ux] != claim.id {
                    return false;
                }
            }
        }

        return true;
    };

    for claim in claims {
        if claim_intact(claim) {
            println!("intact claim: {:?}", claim);
        }
    }
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2018/03.txt") {
        Ok(lines) => {
            let claims: Vec<Claim> = lines.iter()
                .map(|line| parse_line(line))
                .collect();

            let mut grid: [[i32; 1000]; 1000] = [[0; 1000]; 1000];

            for claim in &claims {
                for y in claim.origin_y..(claim.origin_y + claim.height) {
                    for x in claim.origin_x..(claim.origin_x + claim.width) {
                        let ux = x as usize;
                        let uy = y as usize;
                        if grid[uy][ux] != 0 {
                            grid[uy][ux] = -1;
                        } else {
                            grid[uy][ux] = claim.id;
                        }
                    }
                }
            }

            pt1(&mut grid,&claims);
            pt2(&mut grid,&claims);
        }
        Err(e) => panic!(e)
    }
}
