use fileutil;
use advent2019::intcode;

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/05.txt") {
        Ok(lines) => {
            assert!(lines.len() == 1);
            let mut mem = intcode::parse_prog(&lines[0]);
            // let mut mem = intcode::parse_prog("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
            intcode::run_prog(&mut mem);
        }
        Err(e) => println!("{}", e)
    }
}