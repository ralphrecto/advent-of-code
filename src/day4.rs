extern crate chrono;
use fileutil;
use std::collections::HashMap;

type datetime = chrono::DateTime<chrono::FixedOffset>;

#[derive(Debug)]
enum EventType {
    BeginShift(i32),
    FallAsleep,
    WakeUp
}

#[derive(Debug)]
struct Event {
    timestamp: chrono::DateTime<chrono::offset::FixedOffset>,
    event_type: EventType
}

fn parse_line(line: &str) -> Event {
    let timestamp = chrono::DateTime::parse_from_str(&format!("{}{}", &line[1..17], " +0000"),
    "%Y-%m-%d %H:%M %z")
        .unwrap();

    let event_type_str: &str = &line[19..];

    let event_type = if event_type_str.starts_with("Guard") {
        let splits: Vec<&str> = event_type_str.split(" ").collect();
        EventType::BeginShift(splits[1][1..].parse::<i32>().unwrap())
    } else if event_type_str.starts_with("falls") {
        EventType::FallAsleep
    } else {
        EventType::WakeUp
    };

    Event { timestamp, event_type }
}

pub fn run() -> () {
    match fileutil::read_lines("./data/04.txt") {
        Ok(lines) => {
            let mut chronological_events: Vec<Event> = (&lines).iter()
                .map(|line| parse_line(&line))
                .collect::<Vec<Event>>();

            chronological_events.sort_by_key(|e| e.timestamp);

            let mut sleeps_by_id = HashMap::new();
            let mut curr_id = Option::None;
            let mut begin_sleep = Option::None;
            for e in chronological_events {
                match e.event_type {
                    EventType::BeginShift(id) => {
                        curr_id = Option::Some(id);
                    }
                    EventType::FallAsleep => {
                        begin_sleep = Option::Some(e.timestamp);
                    }
                    EventType::WakeUp => {
                        sleeps_by_id.entry(curr_id.unwrap())
                            .or_insert(Vec::<(datetime, datetime)>::new())
                            .push((begin_sleep.unwrap(), e.timestamp));

                        begin_sleep = Option::None;
                    }
                }
            }

            let max_id: i32 = sleeps_by_id
                .iter()
                .map(|(id, sleeps): (&i32, &Vec<(datetime, datetime)>)| {
                    let sum: i64 = sleeps
                        .iter()
                        .map(|&(start_t, end_t)|
                            start_t.signed_duration_since(end_t).num_minutes()
                        ).sum();

                    (id, sum)
                }).max_by_key(|&(id, sum)| sum)
                .map(|(id, sum)| id)
                .unwrap();

            println!("{}", max_id);
        }
        Err(e) => panic!(e)
    }
}
