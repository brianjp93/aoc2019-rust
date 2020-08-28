use super::super::utils;
use super::computer::Computer;

pub fn main() {
    let data = utils::read_cs_nums("./src/data/day9.txt");
    let mut comp = Computer::new(data.clone());
    comp.inputs.push(1);
    comp.run();
    println!("");
    println!("___Day 9___");
    println!("Part 1: {:?}", comp.outputs);

    let mut comp = Computer::new(data.clone());
    comp.inputs.push(2);
    comp.run();
    println!("Part 2: {:?}", comp.outputs);
}
