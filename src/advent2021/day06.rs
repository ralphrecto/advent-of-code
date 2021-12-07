use std::{collections::HashMap, iter::Map};

use fileutil;

fn run_sim(num_steps: usize) {
    let init_ages: Vec<usize> = fileutil::read_file("data/2021/06.txt")
        .unwrap()
        .split(",")
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // There are 9 distinct, contiguous possible fish ages.
    let mut age_arr: Vec<i64> = vec![0; 9];
    for init_age in init_ages {
        age_arr[init_age] += 1;
    }

    for _ in 0..num_steps {
        let mut last = -1;
        for age in (0..=8 as usize).rev() {
            if age == 0 {
                let num = age_arr[age]; 
                age_arr[age] = last;
                age_arr[8] += num;
                age_arr[6] += num;
            } else if age == 8 {
                last = age_arr[age];
                age_arr[age] = 0;
            } else {
                let num = age_arr[age];
                age_arr[age] = last;
                last = num;
            }
        }


        let total_num: i64 = age_arr.iter().sum();
        println!("{}, {:?}", total_num, age_arr);
    }
}

pub fn run() {
    // run_sim(80);
    run_sim(256);
}