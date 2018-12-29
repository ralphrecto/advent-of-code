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

fn value(mut numlist: &mut LinkedList<i32>) -> i32 {
    let num_children = numlist.pop_front().unwrap();
    let num_metadata = numlist.pop_front().unwrap();

    let children_value: Vec<i32> = (0..num_children)
        .map(|child| value(&mut numlist))
        .collect();

    let mut node_value = 0;
    for _ in 0..num_metadata {
        let metadata= numlist.pop_front().unwrap();

        if num_children > 0 {
            if metadata > 0 && metadata <= num_children {
                node_value += children_value[(metadata - 1) as usize];
            }
        } else {
            node_value += metadata
        }
    }

    node_value
}

pub fn run() -> () {
    match fileutil::read_file("./data/08.txt") {
        Ok(s) => {
            let mut numlist: LinkedList<i32> = s.split(" ")
                .map(|numstr| numstr.parse::<i32>().unwrap())
                .collect();

            println!("part 1");
            println!("metadata sum: {:?}", metadata_sum(&mut numlist.clone()));

            println!("part 2");
            println!("value: {:?}", value(&mut numlist.clone()));
        }
        Err(e) => {
            panic!(e);
        }
    }
}