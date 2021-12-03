use fileutil;

fn increasing_windows(window_size: usize) {
    let depths: Vec<i32> = fileutil::read_lines("data/2021/01.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut num_larger = 0;
    let mut last_sum: Option<i32> = None;
    for window in depths.windows(window_size) {
        let window_sum: i32 = window.iter().sum();
        num_larger += match last_sum {
            Some(last_sum_num) if last_sum_num < window_sum => 1,
            _ => 0
        };

        last_sum = Some(window_sum);
    }

    println!("{}", num_larger);
}

pub fn run() {
    // Part1
    // increasing_windows(1);
    
    // Part 2
    increasing_windows(3);
}