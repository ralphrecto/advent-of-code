use std::collections::HashMap;

use fileutil;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn parse_point(point_str: &str) -> Point {
        let mut splits = point_str.split(",");
        
        Point {
            x: splits.next().and_then(|s| s.trim().parse::<i32>().ok()).unwrap(),
            y: splits.next().and_then(|s| s.trim().parse::<i32>().ok()).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point
}

impl Segment {
    pub fn parse_segment(line: &str) -> Segment {
        let mut splits = line.split("->");

        Segment {
            start: Point::parse_point(splits.next().unwrap()),
            end: Point::parse_point(splits.next().unwrap())
        }
    }

    pub fn points(&self) -> Vec<Point> {
        let raw_dx = self.end.x - self.start.x;
        let raw_dy = self.end.y - self.start.y;
        let dx = i32::signum(raw_dx);
        let dy = i32::signum(raw_dy);

        let mut ret = Vec::with_capacity(i32::abs(raw_dx.min(raw_dy)) as usize);
        let mut cur_point = self.start;

        while cur_point != self.end {
            ret.push(cur_point);
            cur_point = Point {
                x: cur_point.x + dx,
                y: cur_point.y + dy
            }
        }
        ret.push(self.end);

        ret
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

fn run_part(vert_or_horiz_only: bool) {
    let lines = fileutil::read_lines("data/2021/05.txt").unwrap();

    let segments: Vec<Segment> = lines.into_iter()
    .map(|s| Segment::parse_segment(&s))
    .collect();

    let mut points: HashMap<Point, i32> = HashMap::new();
    for segment in segments {
        if vert_or_horiz_only && !segment.is_vertical() && !segment.is_horizontal() {
            continue;
        }

        for point in segment.points() {
            let entry = points.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    let ge2_overlaps = points.values().filter(|v| **v >= 2).count();

    println!("{}", ge2_overlaps);
}

pub fn run() {
    run_part(false);
}