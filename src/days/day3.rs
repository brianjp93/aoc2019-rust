use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("./src/data/day3.txt").expect("blah");
    let input: Vec<_> = input.split("\n").map(get_wire).collect();
    let [wire1, wire2] = [&input[0], &input[1]];
    let coords1 = get_coord_set(wire1.to_vec());
    let coords2 = get_coord_set(wire2.to_vec());
    let intersection = coords1.intersection(&coords2);
    let distances: Vec<i32> = intersection.clone().map(|x| get_distance(&x)).collect();
    let min_dist = distances.iter().min();

    let steps: Vec<i32> = intersection
        .map(|x| get_steps_to(x.x, x.y, &wire1) + get_steps_to(x.x, x.y, &wire2))
        .collect();
    let min_steps = steps.iter().min();

    println!("");
    println!("___Day 3___");
    println!("Part 1: {}", min_dist.unwrap());
    println!("Part 2: {}", min_steps.unwrap());
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn get_moves() -> HashMap<&'static str, Point> {
    let mut m = HashMap::new();
    m.insert("U", Point { x: 0, y: 1 });
    m.insert("R", Point { x: 1, y: 0 });
    m.insert("D", Point { x: 0, y: -1 });
    m.insert("L", Point { x: -1, y: 0 });
    m
}

fn get_distance(coord: &Point) -> i32 {
    coord.x.abs() + coord.y.abs()
}

fn get_steps_to(x: i32, y: i32, wire: &Vec<String>) -> i32 {
    let mut coord = Point { x: 0, y: 0 };
    let destination = Point { x: x, y: y };
    let mut dist: i32 = 0;
    let moves = get_moves();
    for mov in wire {
        let dir = &mov[0..1];
        let change = moves.get(&dir).unwrap();
        let n = &mov[1..];
        let n: i32 = n.parse().unwrap();
        for _i in 0..n {
            dist += 1;
            coord.x += change.x;
            coord.y += change.y;
            if coord == destination {
                return dist;
            }
        }
    }
    -1
}

fn get_coord_set(wire: Vec<String>) -> HashSet<Point> {
    let mut out = HashSet::new();
    let mut coord = Point { x: 0, y: 0 };
    let moves = get_moves();
    for mov in wire {
        let dir = &mov[0..1];
        let change = moves.get(&dir).unwrap();
        let n = &mov[1..];
        let n: i32 = n.parse().unwrap();
        for _i in 0..n {
            coord.x += change.x;
            coord.y += change.y;
            out.insert(Point { ..coord });
        }
    }
    out
}

fn get_wire(line: &str) -> Vec<String> {
    line.split(",").map(|x| x.parse().unwrap()).collect()
}
