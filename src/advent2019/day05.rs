use fileutil;
use advent2019::intcode;

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/05.txt") {
        Ok(lines) => {
            assert!(lines.len() == 1);
            let mut mem = intcode::parse_prog(&lines[0]);
            intcode::run_prog(&mut mem);
        }
        Err(e) => println!("{}", e)
    }
}