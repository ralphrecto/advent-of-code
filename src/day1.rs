use fileutil;

pub fn run() -> () {
    match fileutil::read_lines("./data/01.txt") {
        Ok(lines) => {
            let sum: i32 = lines
                .iter()
                .map(|x: &String| {
                    let n: i32 = x.parse().unwrap();
                    return n;
                }).fold(0, |acc, x| acc + x);

            println!("{}", sum);
        }
        Err(e) => println!("{}", e)
    }
}