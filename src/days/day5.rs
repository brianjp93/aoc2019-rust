use super::computer::Computer;
use super::super::utils;

pub fn main() {
    let input = utils::read_cs_nums("./src/data/day5.txt");
    let mut comp = Computer::new(input.clone());
    comp.inputs.push(1);
    comp.run();
    println!("");
    println!("___Day 5___");
    println!("Part 1: {}", comp.outputs.last().unwrap());

    let mut comp = Computer::new(input.clone());
    comp.inputs.push(5);
    comp.run();
    println!("Part 2: {}", comp.outputs.last().unwrap());
}
