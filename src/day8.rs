use fileutil;
use std::collections::LinkedList;

fn metadata_sum(mut numlist: &mut LinkedList<i32>) -> i32 {
    let num_children = numlist.pop_front().unwrap();
    let num_metadata = numlist.pop_front().unwrap();

    let mut children_sum = 0;
    for _ in 0..num_children {
        children_sum += metadata_sum(&mut numlist);
    }

    let mut node_sum = 0;
    for _ in 0..num_metadata {
        node_sum += numlist.pop_front().unwrap();
    }

    children_sum + node_sum
}

pub fn run() -> () {
    match fileutil::read_file("./data/08.txt") {
        Ok(s) => {
            let mut numlist: LinkedList<i32> = s.split(" ")
                .map(|numstr| numstr.parse::<i32>().unwrap())
                .collect();

            println!("part 1");
            println!("metadata sum: {:?}", metadata_sum(&mut numlist));
        }
        Err(e) => {
            panic!(e);
        }
    }
}