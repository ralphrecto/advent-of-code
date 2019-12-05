use fileutil;
use std::i32;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

const ORIGIN: Point = Point {x: 0, y: 0};

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

fn intersections(first_line: &Vec<Segment>, second_line: &Vec<Segment>) -> Vec<Point> {
    let origin = Point {x: 0, y: 0};
    let mut intersections: Vec<Point> = Vec::new();
    for segment_a in first_line.iter() {
        for segment_b in second_line.iter() {
            match intersection(segment_a, segment_b) {
                Some(pt) => {
                    // Do not count intersecting at the central port
                    if pt == origin {
                        continue;
                    }

                    intersections.push(pt);
                },
                None => ()
            }
        }
    }

    return intersections;
}

fn manhattan_dist(a: &Point, b: &Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

fn length(s: &Segment) -> i32 {
    return manhattan_dist(&(*s).start, &(*s).end);
}

fn contains_point(s: &Segment, p: &Point) -> bool {
    let x_ok = (s.start.x <= p.x && p.x <= s.end.x) ||
        (s.end.x <= p.x && p.x <= s.start.x);

    let y_ok = (s.start.y <= p.y && p.y <= s.end.y) ||
        (s.end.y <= p.y && p.y <= s.start.y);

    return x_ok && y_ok;
}

fn pt1(first_line: &Vec<Segment>, second_line: &Vec<Segment>) -> () {
    let mut min_dist: i32 = i32::MAX;
    for intersection in intersections(first_line, second_line) {
        min_dist = min_dist.min(manhattan_dist(&intersection, &ORIGIN));
    }

    println!("{}", min_dist);
}

fn delay(line: &Vec<Segment>, p: &Point) -> i32 {
    let mut delay_acc: i32 = 0;
    for segment in line {
        if contains_point(segment, p) {
            let truncated = Segment {start: segment.start, end: *p};
            delay_acc += length(&truncated);
            return delay_acc;
        } else {
            delay_acc += length(segment);
        }
    }

    panic!("point {:?} was not in line {:?}", p, line);
}

fn pt2(first_line: &Vec<Segment>, second_line: &Vec<Segment>) -> () {
    let mut min_dist: i32 = i32::MAX;
    for intersection in intersections(first_line, second_line) {
        let first_delay = delay(first_line, &intersection);
        let second_delay = delay(second_line, &intersection);
        min_dist = min_dist.min(first_delay + second_delay);
    }

    println!("{}", min_dist);
}


pub fn run() -> () {
    match fileutil::read_lines("./data/2019/03.txt") {
        Ok(lines) => {
            assert!(lines.len() == 2);

            let first_line: Vec<Segment> = line_segments(Point{x: 0, y: 0}, &lines[0]);
            let second_line: Vec<Segment> = line_segments(Point{x: 0, y: 0}, &lines[1]);
            
            pt1(&first_line, &second_line);
            pt2(&first_line, &second_line);
        }
        Err(e) => println!("{}", e)
    }
}