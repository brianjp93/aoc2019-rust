use std::fs;

pub fn main() {
    let input = fs::read_to_string("./src/data/day2.txt")
        .expect("Error");
    let mut vec: Vec<i64> = input.trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", vec);
}
