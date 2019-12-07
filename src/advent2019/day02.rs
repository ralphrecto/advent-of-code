use fileutil;
use advent2019::intcode;

fn grav_assist_prog(prog: &str, noun: i32, verb: i32) -> i32 {
    let mut ops: Vec<i32> = prog.split(',')
        .map(|cmp| cmp.parse::<i32>().unwrap())
        .collect();

    // pre-run mutations
    ops[1] = noun;
    ops[2] = verb;

    intcode::run_prog(&mut ops);
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