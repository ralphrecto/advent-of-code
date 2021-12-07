use fileutil;

fn compute_sum<F>(objective: F) where F: Fn(i32,&i32) -> i32 {
    let poslist: Vec<i32> = fileutil::read_file("data/2021/07.txt")
        .unwrap()
        .split(",")
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let pos_min = *poslist.iter().min().unwrap();
    let pos_max = *poslist.iter().max().unwrap();

    let mut min_sum = i32::MAX;
    for cand in pos_min..=pos_max {
        let cand_sum: i32 = poslist.iter()
            .map(|p| objective(cand, p))
            .sum();

        min_sum = min_sum.min(cand_sum);
    }

    println!("{}", min_sum);
}

pub fn run() {
    // compute_sum(|cand, p| i32::abs(*p - cand));
    compute_sum(|cand, p| {
        let d = i32::abs(*p - cand);
        (d * (d+1)) / 2
    });
}