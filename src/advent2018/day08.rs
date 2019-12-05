use fileutil;
use std::slice::Iter;

fn metadata/2018_sum(mut numlist: &mut Iter<i32>) -> i32 {
    let num_children = numlist.next().unwrap();
    let num_metadata/2018 = numlist.next().unwrap();

    let mut children_sum = 0;
    for _ in 0..*num_children {
        children_sum += metadata/2018_sum(&mut numlist);
    }

    let mut node_sum = 0;
    for _ in 0..*num_metadata/2018 {
        node_sum += numlist.next().unwrap();
    }

    children_sum + node_sum
}

fn value(mut numlist: &mut Iter<i32>) -> i32 {
    let num_children = numlist.next().unwrap();
    let num_metadata/2018 = numlist.next().unwrap();

    let children_value: Vec<i32> = (0..*num_children)
        .map(|child| value(&mut numlist))
        .collect();

    let mut node_value = 0;
    for _ in 0..*num_metadata/2018 {
        let metadata/2018= numlist.next().unwrap();

        if *num_children > 0 {
            if *metadata/2018 > 0 && *metadata/2018 <= *num_children {
                node_value += children_value[(metadata/2018 - 1) as usize];
            }
        } else {
            node_value += *metadata/2018
        }
    }

    node_value
}

pub fn run() -> () {
    match fileutil::read_file("./data/2018/08.txt") {
        Ok(s) => {
            let mut numlist: Vec<i32> = s.split(" ")
                .map(|numstr| numstr.parse::<i32>().unwrap())
                .collect();
            let whole_slice: &[i32] = &numlist[..];

            println!("part 1");
            println!("metadata/2018 sum: {:?}", metadata/2018_sum(&mut whole_slice.iter()));

            println!("part 2");
            println!("value: {:?}", value(&mut whole_slice.iter()));
        }
        Err(e) => {
            panic!(e);
        }
    }
}
