use fileutil;

fn grav_assist_prog(prog: &str, noun: usize, verb: usize) -> usize {
    let mut ops: Vec<usize> = prog.split(',')
        .map(|cmp| cmp.parse::<usize>().unwrap())
        .collect();

    // pre-run mutations
    ops[1] = noun;
    ops[2] = verb;

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

    return ops[0];
}

fn pt2(prog: &str) -> () {
    for noun in 0..100 {
        for verb in 0..100 {
            if grav_assist_prog(prog, noun, verb) == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                return;
            }
        }
    }
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/02.txt") {
        Ok(lines) => {
            assert!(lines.len() == 1);
            // pt. 1
            // println!("{}", grav_assist_prog(&lines[0], 12, 2));

            pt2(&lines[0]);
        }
        Err(e) => println!("{}", e)
    }
}