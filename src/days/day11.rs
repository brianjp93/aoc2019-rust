use std::collections::HashMap;
use super::super::utils;
use super::computer::Computer;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn main() {
    let data = utils::read_cs_nums("src/data/day11.txt");
    // println!("{:?}", data);
    let mut comp = Computer::new(data);
    comp.run();
    println!("{:?}", comp.outputs);
    let map = do_paint(&mut comp);
    println!("{:?}", map);
}

fn do_paint(comp: &mut Computer) -> HashMap<Point, String> {
    let mut map: HashMap<Point, String> = HashMap::new();
    comp.inputs.push(0);
    let mut paint: i64;
    let mut mov: i64;

    let loc = Point{x: 0, y: 0};

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut facing: i64 = 0;

    loop {
        comp.run();
        if comp.outputs.len() == 0 {
            break
        }
        paint = comp.outputs[0];
        mov = comp.outputs[1];
        comp.outputs = vec![];
        if paint == 0 {
            map[&loc] = ".".to_owned();
        }
        else {
            map[&loc] = "#".to_owned();
        }
        if mov == 0 {
            facing -= 1;
            if facing < 0 {
                facing = facing + directions.len() as i64
            }
        }
        else {
            facing += 1;
            if facing >= directions.len() as i64 {
                facing = facing - directions.len() as i64
            }
        }
        let loc = Point{x: loc.x + directions[facing as usize].0, y: loc.y + directions[facing as usize].1};
        if map[&loc] == "#" {
            comp.inputs.push(1);
        } else {
            comp.inputs.push(0);
        }
    }
    map
}
