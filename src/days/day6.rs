use std::fs;
use std::collections::HashMap;


pub fn main() {
    println!("hello");
    let data = fs::read_to_string("./src/data/day6.txt").expect("Error");
    let data_list: Vec<_> = data.trim().split("\n").collect();
    let mut data: HashMap<String, String> = HashMap::new();
    for elt in data_list {
        let parts = elt.split(")");
        println!("{:?}", parts);
    }
    println!("{:?}", data);
}
