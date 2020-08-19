use std::collections::HashSet;
use std::fs;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let input = fs::read_to_string("./src/data/day3.txt").expect("blah");
    let input: Vec<_> = input.split("\n").map(get_wire).collect();
    let [wire1, wire2] = [&input[0], &input[1]];
    let coords1 = get_coord_set(wire1.to_vec());
    let coords2 = get_coord_set(wire2.to_vec());
    let intersection = coords1.intersection(&coords2);
    let distances: Vec<i32> = intersection.map(|x| get_distance(&x)).collect();
    let min_dist = distances.iter().min();

    println!("");
    println!("___Day 3___");
    println!("Part 1: {}", min_dist.unwrap());
}

fn get_distance(coord: &Point) -> i32 {
    coord.x.abs() + coord.y.abs()
}

fn get_coord_set(wire: Vec<String>) -> HashSet<Point> {
    let mut out = HashSet::new();
    let mut coord = Point { x: 0, y: 0 };
    // out.insert(Point { ..coord });
    for mov in wire {
        let mov: Vec<char> = mov.chars().collect();
        let dir = mov[0].to_string();
        let n = &mov[1..].iter().cloned().collect::<String>();
        let n: i32 = n.parse().unwrap();
        if dir == "U" {
            for _i in 0..n {
                coord.y += 1;
                out.insert(Point { ..coord });
            }
        } else if dir == "R" {
            for _i in 0..n {
                coord.x += 1;
                out.insert(Point { ..coord });
            }
        } else if dir == "D" {
            for _i in 0..n {
                coord.y -= 1;
                out.insert(Point { ..coord });
            }
        } else if dir == "L" {
            for _i in 0..n {
                coord.x -= 1;
                out.insert(Point { ..coord });
            }
        }
    }
    out
}

fn get_wire(line: &str) -> Vec<String> {
    line.split(",").map(|x| x.parse().unwrap()).collect()
}
