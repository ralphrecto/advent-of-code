use fileutil;

fn pt1(line: &str) -> () {
    let mut ops: Vec<usize> = line.split(',')
        .map(|cmp| cmp.parse::<usize>().unwrap())
        .collect();

    // pre-run mutations
    ops[1] = 12;
    ops[2] = 2;

    let mut current_op_idx = 0;
    let mut current_op = ops[current_op_idx];
    while current_op != 99 {
        let lhs = ops[ops[current_op_idx + 1]];
        let rhs = ops[ops[current_op_idx + 2]];
        let loc = ops[current_op_idx + 3];

        if current_op == 1 {
            ops[loc] = lhs + rhs; 
        } else if current_op == 2 {
            ops[loc] = lhs * rhs; 
        } else {
            panic!("unknown op code {}", current_op);
        }

        current_op_idx += 4;
        current_op = ops[current_op_idx];
    }

    println!("{}", ops[0]);
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/02.txt") {
        Ok(lines) => {
            assert!(lines.len() == 1);
            pt1(&lines[0]);
        }
        Err(e) => println!("{}", e)
    }
}