pub fn run() -> () {
    let mut num_passwords = 0;
    for i in 284639..=748759 {
        // Assume i is a 6 digit number.            
        let d1 = i / 100000;
        let d2 = (i / 10000) % 10;
        let d3 = (i / 1000) % 10;
        let d4 = (i / 100) % 10;
        let d5 = (i / 10) % 10;
        let d6 = i % 10;
        
        let monotone = d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5 && d5 <= d6;

        if !monotone {
            continue;
        }

        let adjacent_ok = (d1 == d2 && d2 != d3) ||
            (d2 == d3 && d1 != d2 && d3 != d4) ||
            (d3 == d4 && d2 != d3 && d4 != d5) ||
            (d4 == d5 && d3 != d4 && d5 != d6) ||
            (d5 == d6 && d4 != d5);

        if monotone && adjacent_ok {
            num_passwords += 1;
        }
    }

    println!("num passwords: {}", num_passwords);
}
