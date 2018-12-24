use fileutil;
use std::collections::HashMap;

#[derive(Debug)]
enum EventType {
    BeginShift(i32),
    FallAsleep(i32),
    WakeUp(i32)
}

#[derive(Debug)]
struct Event<'a> {
    timestamp: &'a str,
    event_type: EventType
}

fn parse_line(line: &str) -> Event {
    let timestamp: &str = &line[1..17];
    let mins: i32 = (&line[15..17]).parse().unwrap();
    let event_type_str: &str = &line[19..];

    let event_type = if event_type_str.starts_with("Guard") {
        let splits: Vec<&str> = event_type_str.split(" ").collect();
        EventType::BeginShift(splits[1][1..].parse::<i32>().unwrap())
    } else if event_type_str.starts_with("falls") {
        EventType::FallAsleep(mins)
    } else {
        EventType::WakeUp(mins)
    };

    Event { timestamp, event_type }
}

fn tabulate_awake_mins(sleeps: &Vec<(i32, i32)>, awake_count: &mut[u32; 60]) -> () {
    for &(start, end) in sleeps.iter() {
        for min in start..end {
            awake_count[min as usize] += 1;
        }
    }
}

// given an awake_count array, returns the pair (sleepiest mins, # times asleep on that min)
fn calc_sleepiest_min(awake_count: &[u32; 60]) -> (usize, u32) {
    let mut max_idx = 0;
    let mut max_idx_curr_count = 0;
    for i in 0..awake_count.len() {
        if awake_count[i] > max_idx_curr_count {
            max_idx = i;
            max_idx_curr_count = awake_count[i];
        }
    }

    return (max_idx, max_idx_curr_count);
}

fn part1(sleeps_by_id: &HashMap<i32, Vec<(i32, i32)>>) -> () {
    println!("part 1");

    // Task 1: find the guard who sleeps the most
    let mut curr_max_id = None;
    let mut curr_max_mins = 0;
    for (id, sleeps) in sleeps_by_id {
        let sum: i32 = sleeps.iter()
            .map(|pair| pair.1 - pair.0)
            .sum();

        if sum > curr_max_mins {
            curr_max_mins = sum;
            curr_max_id = Option::Some(id);
        }
    }

    println!("guard {} slept the most ({} mins)", curr_max_id.unwrap(), curr_max_mins);

    let max_id_sleeps: &Vec<(i32, i32)> =
        (sleeps_by_id).get(curr_max_id.unwrap()).unwrap();

    // Task 2: find the minute they sleep the most
    let mut awake_count: [u32; 60] = [0; 60];
    tabulate_awake_mins(max_id_sleeps, &mut awake_count);
    let (sleepiest_min, times_asleep) = calc_sleepiest_min(&awake_count);

    println!("max min asleep was {}", sleepiest_min);
}

fn part2(sleeps_by_id: &HashMap<i32, Vec<(i32, i32)>>) -> () {
    // Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
    println!("part 2");

    // List with elements (guard id, awake_mins array)
    let mut awake_list: Vec<(i32, [u32; 60])> = Vec::new();
    for (id, sleeps) in sleeps_by_id.iter() {
        let mut awake_mins: [u32; 60] = [0; 60];
        tabulate_awake_mins(sleeps, &mut awake_mins);
        awake_list.push((*id, awake_mins));
    }

    // Global maximum of how many times any guard has been asleep on any minute
    let mut sleepiest_min_count: u32 = 0;
    let mut sleepiest_guard_min: Option<(i32, usize)> = Option::None;
    for &(id, awake_mins) in awake_list.iter() {
        let (sleepiest_min, mins_count) = calc_sleepiest_min(&awake_mins);
        if sleepiest_min_count < mins_count {
            sleepiest_min_count  = mins_count;
            sleepiest_guard_min = Option::Some((id, sleepiest_min));
        }
    }

    let (id, sleepiest_min) = sleepiest_guard_min.unwrap();
    println!(
        "guard {} was asleep the most on minute {} ({} times)",
        id, sleepiest_min, sleepiest_min_count
    );
}

pub fn run() -> () {
    match fileutil::read_lines("./data/04.txt") {
        Ok(lines) => {
            let mut chronological_events: Vec<Event> = (&lines).iter()
                .map(|line| parse_line(&line))
                .collect::<Vec<Event>>();

            chronological_events.sort_by_key(|e| e.timestamp);

            // map of form (guard id -> list of pairs of begin/end sleep mins)
            let mut sleeps_by_id: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
            let mut curr_id = Option::None;
            let mut begin_sleep = Option::None;
            for e in chronological_events {
                match e.event_type {
                    EventType::BeginShift(id) => {
                        curr_id = Option::Some(id);
                    }
                    EventType::FallAsleep(sleep_min) => {
                        begin_sleep = Option::Some(sleep_min);
                    }
                    EventType::WakeUp(wake_min) => {
                        sleeps_by_id.entry(curr_id.unwrap())
                            .or_insert(Vec::<(i32, i32)>::new())
                            .push((begin_sleep.unwrap(), wake_min));

                        begin_sleep = Option::None;
                    }
                }
            }

            part1(&sleeps_by_id);
            part2(&sleeps_by_id);
        }
        Err(e) => panic!(e)
    }
}
