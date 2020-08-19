use std::fs::File;
use std::io::{BufRead, BufReader};

// #[allow(dead_code)]
pub fn main() {
    let f = File::open("./src/data/day1.txt").unwrap();
    let reader = BufReader::new(f);

    let mut part1 = 0;
    let mut part2 = 0;
    for line in reader.lines() {
        let input = line.unwrap().parse::<i32>().unwrap();
        part1 += find_fuel(input);
        part2 += find_fuel_recursive(input);
    }

    println!("___Day 1___");
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn find_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn find_fuel_recursive(mass: i32) -> i32 {
    let x = (mass / 3) - 2;
    if x > 0 {
        x + find_fuel_recursive(x)
    } else {
        0
    }
}
