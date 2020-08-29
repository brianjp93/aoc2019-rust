use std::fs;

pub fn main() {
    println!("hello");
    let data = fs::read_to_string("./src/data/day10.txt").unwrap();
    let data = String::from(data);
    let data: Vec<String> = data.trim().split("\n").collect();
    println!("{:?}", data);
}
