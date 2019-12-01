use fileutil;

fn pt1(lines: &Vec<String>) -> () {
    let sum: i32 = lines.iter()
        .map(|mass_str| (mass_str.parse::<i32>().unwrap() / 3) - 2)
        .sum();

    println!("{}", sum);
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/01.txt") {
        Ok(lines) => {
            pt1(&lines);
        }
        Err(e) => println!("{}", e)
    }
}
