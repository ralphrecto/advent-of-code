use fileutil;

fn pt1(lines: &[String]) -> () {
    println!("hi");
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2018/01.txt") {
        Ok(lines) => {
            pt1(&lines);
        }
        Err(e) => println!("{}", e)
    }
}
