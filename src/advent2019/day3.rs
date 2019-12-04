use fileutil;
use std::i32;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: Point,
    end: Point
}

// Get line segments from the following directions in the given str
// beginning from the origin.
fn line_segments(start: Point, line: &str) -> Vec<Segment> {
    let mut x = 0;
    let mut y = 0;

    // vector of deltas in each component (dx, dy)
    let directions: Vec<Point> = line.split(",")
        .map(|cmp: &str| {
            let dir_str: &str = cmp.get(0..1).unwrap();
            let magnitude: i32 = cmp.get(1..)
                .and_then(|x| x.parse::<i32>().ok())
                .unwrap();

            let dir = match dir_str {
                "U" => Point {x: 0, y: 1},
                "R" => Point {x: 1, y: 0},
                "L" => Point {x: -1, y: 0},
                "D" => Point {x: 0, y: -1},
                _ => panic!("invalid direction {}", dir_str)
            };
            
            return Point {x: dir.x * magnitude, y: dir.y * magnitude};
        }).collect();

    let mut start = start;
    let mut line_segments: Vec<Segment> = Vec::with_capacity(directions.len());
    for direction in directions {
        let end = Point {x: start.x + direction.x, y: start.y + direction.y};
        line_segments.push(Segment {start: start, end: end});

        start = end;
    }

    return line_segments;
}

// This only handles vertical/horizontal intersections!
fn intersection(a: &Segment, b: &Segment) -> Option<Point> {
    let vert: &Segment;
    let horiz: &Segment;

    if a.start.x == a.end.x {
        // a is vertical, b is horizontal
        vert = a;
        horiz = b;
    } else if a.start.y == a.end.y {
        // a is horizontal, b is vertical
        vert = b;
        horiz = a;
    } else {
        panic!("expecting all segments to be either horizontal or vertical");
    }

    let x_ok = (horiz.start.x <= vert.start.x && vert.start.x <= horiz.end.x) ||
        (horiz.end.x <= vert.start.x && vert.start.x <= horiz.start.x);
    
    let y_ok = (vert.start.y <= horiz.start.y && horiz.start.y <= vert.end.y) ||
        (vert.end.y <= horiz.start.y && horiz.start.y <= vert.start.y);
    
    if x_ok && y_ok {
        return Option::Some(Point {x: vert.start.x, y: horiz.start.y});
    } else {
        return Option::None;
    }
}

fn manhattan_dist(a: Point, b: Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

fn pt1(line1: &str, line2: &str) -> () {
    let first_line: Vec<Segment> = line_segments(Point{x: 0, y: 0}, line1);
    let second_line: Vec<Segment> = line_segments(Point{x: 0, y: 0}, line2);

    let origin = Point {x: 0, y: 0};
    let mut min_dist: i32 = i32::MAX;
    for segment_a in first_line.iter() {
        for segment_b in second_line.iter() {
            match intersection(segment_a, segment_b) {
                Some(pt) => {
                    // Do not count intersecting at the central port
                    if pt == origin {
                        continue;
                    }

                    min_dist = min_dist.min(manhattan_dist(pt, origin));
                }
                None => ()
            }
        }
    }

    println!("{}", min_dist);
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/03.txt") {
        Ok(lines) => {
            assert!(lines.len() == 2);
            pt1(&lines[0], &lines[1]);
        }
        Err(e) => println!("{}", e)
    }
}