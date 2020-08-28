use super::super::utils;
use super::computer::Computer;
use itertools::Itertools;

pub fn main() {
    let input = utils::read_cs_nums("./src/data/day7.txt");
    let max1 = find_max(input.clone(), vec![0, 1, 2, 3, 4]);
    println!("");
    println!("___Day 7___");
    println!("Part 1: {}", max1);
    let max = find_max(input.clone(), vec![5, 6, 7, 8, 9]);
    println!("Part 2: {}", max);
}

fn find_max(input: Vec<i64>, allowed_ints: Vec<i64>) -> i64 {
    let mut max = 0;
    for perm in allowed_ints.into_iter().permutations(5) {
        let out = recursive_run(input.clone(), perm);
        if out > max {
            max = out;
        }
    }
    max
}

fn recursive_run(input: Vec<i64>, phase: Vec<i64>) -> i64 {
    let mut circuit = (0..5).map(|_| Computer::new(input.clone())).collect_vec();
    let circuit_len = circuit.len();
    for i in 0..5 {
        circuit[i].inputs.push(phase[i]);
        if i == 0 {
            circuit[i].inputs.push(0)
        };
    }
    while !circuit[circuit_len - 1].is_done {
        for i in 0..circuit_len {
            let index: usize = if i == 0 { circuit_len - 1 } else { i - 1 };
            let outputs: Vec<i64> = circuit[index].outputs.drain(..).collect();
            circuit[i].inputs.extend(outputs);
            circuit[i].run();
        }
    }
    circuit[circuit_len - 1].outputs[0]
}
