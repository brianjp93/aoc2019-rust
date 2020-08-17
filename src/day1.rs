use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let f = File::open("./src/data/day1.txt").unwrap();
    let reader = BufReader::new(f);

    let mut total: i32 = 0;
    let mut x: i32;
    for line in reader.lines() {
        let input = line.unwrap().parse::<i32>().unwrap();
        x = find_fuel(input);
        // println!("{}", x);
        total += x;
    }
    println!("total: {}", total);
}

fn find_fuel(mass: i32) -> i32 {
    let x = (mass / 3) - 2;
    return x;
}
