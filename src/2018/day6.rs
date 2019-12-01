use fileutil;
use std::i32;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn dist(&self, Coord {x, y}: &Self) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

// Returns the coordinate closest to c, or None if there is a tie.
fn closest(c: Coord, cand_coords: &Vec<Coord>) -> Option<Coord> {
    let mut distvec: Vec<(Coord, i32)> = cand_coords.iter()
        .map(|coord| (*coord, c.dist(coord)))
        .collect();

    distvec.sort_by_key(|pair| pair.1);

    match &distvec[..] {
        [] => Option::None,
        [(coord, dist)] => Option::Some(*coord),
        _ => {
            let (coord0, dist0) = distvec[0];
            let (coord1, dist1) = distvec[1];
            if dist0 == dist1 {
                Option::None
            } else {
                Option::Some(coord0)
            }
        }
    }
}

pub fn run() -> () {
    match fileutil::read_lines("./data/06.txt") {
        Ok(lines) => {
            // Part 1.
            let mut xmin = i32::MAX;
            let mut xmax = i32::MIN;
            let mut ymin = i32::MAX;
            let mut ymax = i32::MIN;

            let mut coords: Vec<Coord> = Vec::with_capacity(lines.len());
            for line in lines {
                let coord_vec : Vec<i32> = line.split(", ")
                    .map(|coord_str| coord_str.parse::<i32>().unwrap())
                    .collect();

                xmin = i32::min(xmin, coord_vec[0]);
                xmax = i32::max(xmax, coord_vec[0]);
                ymin = i32::min(ymin, coord_vec[1]);
                ymax = i32::max(ymax, coord_vec[1]);

                coords.push(Coord { x: coord_vec[0], y: coord_vec[1] });
            }

            let mut closest_count: HashMap<Coord, u32> = HashMap::with_capacity(coords.len());
            let mut infinite_area_coords: HashSet<Coord> = HashSet::new();

            /// High level idea: coordinates which are the closest for squares along the border
            /// of the grid will have an infinite area.
            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    match closest(Coord {x, y}, &coords) {
                        Some(c) => {
                            *closest_count.entry(c).or_insert(0) += 1;

                            // This predicate indicates being on the border of the grid.
                            if x == xmin || x == xmax || y == ymin || y == ymax {
                                infinite_area_coords.insert(c);
                            }
                        }
                        None => { }
                    }
                }
            }

            let max = closest_count.iter()
                .map(|(key, val)| {
                    if infinite_area_coords.contains(key) { 0 } else { *val }
                }).max()
                .unwrap();

            println!("part 1");
            println!("max finite area: {}", max);

            // Part 2.
            println!("part 2");

            let mut region_size = 0;
            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    let xycoord = Coord { x, y };
                    let mut totaldist : i32 = coords.iter()
                        .map(|coord| xycoord.dist(coord))
                        .sum();

                    if totaldist < 10000 {
                        region_size += 1;
                    }
                }
            }

            println!("region size: {}", region_size);
        }
        Err(e) => {
            panic!(e);
        }
    }
}