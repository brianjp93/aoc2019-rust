use std::fs;
use std::collections::HashSet;

pub fn main() {
    let data = fs::read_to_string("./src/data/day10.txt").unwrap();
    let data: Vec<&str> = data.trim().split("\n").collect();
    let field = Field {map: get_point_vector(data)};
    let count = field.find_most_visible();
    println!("{}", count);
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn dist(&self, other: &Self) -> f64 {
        let xdiff = self.x as f64 - other.x as f64;
        let ydiff = self.y as f64 - other.y as f64;
        (xdiff.powf(2.0) + ydiff.powf(2.0)).sqrt()
    }
    fn angle_to(&self, other: &Point) -> f64 {
        let x = other.x as f64 - self.x as f64;
        let y = other.y as f64 - self.y as f64;
        let angle = y.atan2(x);
        angle
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.x
    }
}

#[derive(Debug)]
struct Field {
    map: Vec<Point>,
}

impl Field {
    fn count_visible(&self, point: &Point) -> i64 {
        let mut angles = HashSet::<i32>::new();
        for other in &self.map {
            if point != other {
                let angle = (point.angle_to(other) * 100000_f64) as i32;
                angles.insert(angle);
            }
        }
        angles.len() as i64
    }
    fn find_most_visible(&self) -> i64 {
        let mut max = 0;
        for point in &self.map {
            let count = self.count_visible(&point);
            if count > max {
                max = count
            }
        }
        max
    }
}

fn get_point_vector(data: Vec<&str>) -> Vec<Point> {
    let mut points = Vec::<Point>::new();
    for (y, row) in data.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                points.push(Point {x: x as i64, y: y as i64})
            }
        }
    }
    points
}
