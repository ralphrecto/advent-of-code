use fileutil;
use std::i32;

fn fuel_needed(mass: i32) -> i32 {
    let needed = (mass / 3) - 2;
    if needed < 0 { 0 } else { needed }
}

fn pt1(lines: &Vec<String>) -> () {
    let sum: i32 = lines.iter()
        .map(|mass_str| mass_str.parse::<i32>().unwrap()) 
        .map(fuel_needed)
        .sum();

    println!("{}", sum);
}

fn pt2(lines: &Vec<String>) -> () {
    let mut acc = 0; 
    for module_mass_str in lines.iter() {
        let module_mass: i32 = module_mass_str.parse().unwrap();
        let mut component = module_mass;
        let mut needed = -1;
        let mut module_acc = 0;

        while needed != 0 {
            needed = fuel_needed(component);
            module_acc += needed;
            component = needed;
        }

        acc += module_acc;
    }

    println!("{}", acc);
}

pub fn run() -> () {
    match fileutil::read_lines("./data/2019/01.txt") {
        Ok(lines) => {
            // pt1(&lines);
            pt2(&lines);
        }
        Err(e) => println!("{}", e)
    }
}
