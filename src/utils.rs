use std::fs;

pub fn read_cs_nums(filename: &str) -> Vec<i64> {
    let input = fs::read_to_string(filename).expect("Error");
    let mut vec: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    vec
}
