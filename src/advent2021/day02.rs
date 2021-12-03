use fileutil;

pub fn run() {
    let instructions: Vec<String> = fileutil::read_lines("data/2021/02.txt").unwrap();

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instr in instructions {
        let mut instr_splits = instr.split(" ");
        let direction = instr_splits.next().unwrap();
        let amt = instr_splits.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                x += amt;
                depth += aim * amt;
            },
            "up" => { aim -= amt },
            "down" => { aim += amt }
            _ => { panic!("Unknown direction {}", direction) }
        }
    }

    println!("{}", x * depth);
}