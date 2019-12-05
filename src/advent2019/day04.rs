fn pt1(low: i32, high: i32) -> () {
    let mut num_passwords = 0;
    for i in low..=high {
        // Assume i is a 6 digit number.            
        let d1 = i / 100000;
        let d2 = (i / 10000) % 10;
        let d3 = (i / 1000) % 10;
        let d4 = (i / 100) % 10;
        let d5 = (i / 10) % 10;
        let d6 = i % 10;
        
        let monotone = d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5 && d5 <= d6;
        let has_adjacent = d1 == d2 || d2 == d3 || d3 == d4 || d4 == d5 || d5 == d6;

        if monotone && has_adjacent {
            num_passwords += 1;
        }
    }

    println!("num passwords: {}", num_passwords);
}

pub fn run() -> () {
    pt1(284639, 748759);
}
