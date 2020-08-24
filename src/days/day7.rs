use super::super::utils;
use itertools::Itertools;

pub fn main() {
    let input = utils::read_cs_nums("./src/data/day7.txt");
    let max1 = find_max(input.clone(), vec![0, 1, 2, 3, 4]);
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

struct Computer {
    code: Vec<i64>,
    index: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    is_done: bool,
    is_waiting: bool,
}

impl Computer {
    fn new(code: Vec<i64>) -> Self {
        let mut x = Computer {
            code,
            index: 0,
            inputs: Vec::<i64>::new(),
            outputs: Vec::<i64>::new(),
            is_done: false,
            is_waiting: false,
        };
        x.code.extend(vec![0; 10000]);
        x
    }
    fn run(&mut self) {
        self.is_waiting = false;
        'loops: loop {
            match self.get_opcode() {
                1 => self.add(),
                2 => self.mul(),
                3 => {
                    self.get_input();
                    if self.is_waiting {
                        break 'loops;
                    }
                }
                4 => self.output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                99 => {
                    self.is_done = true;
                    break 'loops;
                }
                _ => {
                    println!("wtf");
                    break 'loops;
                }
            };
        }
    }
    fn get_opcode(&self) -> i64 {
        self.code[self.index] % 100
    }
    fn get_positions(&self) -> [usize; 3] {
        let opcode = self.code[self.index];
        let c = (opcode / 100) % 10;
        let b = (opcode / 1000) % 10;
        let a = (opcode / 10000) % 10;
        let mut out = [0 as usize; 3];
        for (i, mode) in [c, b, a].iter().enumerate() {
            out[i] = if *mode == 0 {
                self.code[self.index + i + 1] as usize
            } else {
                self.index + i + 1
            };
        }
        out
    }
    fn less_than(&mut self) {
        let [pos1, pos2, pos3] = self.get_positions();
        self.code[pos3] = if self.code[pos1] < self.code[pos2] {
            1
        } else {
            0
        };
        self.index += 4;
    }
    fn equals(&mut self) {
        let [pos1, pos2, pos3] = self.get_positions();
        self.code[pos3] = if self.code[pos1] == self.code[pos2] {
            1
        } else {
            0
        };
        self.index += 4;
    }
    fn jump_if_true(&mut self) {
        let [pos1, pos2, _] = self.get_positions();
        self.index = if self.code[pos1] != 0 {
            self.code[pos2] as usize
        } else {
            self.index + 3
        };
    }
    fn jump_if_false(&mut self) {
        let [pos1, pos2, _] = self.get_positions();
        self.index = if self.code[pos1] == 0 {
            self.code[pos2] as usize
        } else {
            self.index + 3
        };
    }
    fn get_input(&mut self) {
        if self.inputs.len() > 0 {
            let [pos1, _, _] = self.get_positions();
            let val = self.inputs.remove(0);
            self.code[pos1] = val;
            self.index += 2;
        } else {
            self.is_waiting = true;
        }
    }
    fn output(&mut self) {
        let [pos1, _, _] = self.get_positions();
        self.outputs.push(self.code[pos1]);
        self.index += 2;
    }
    fn add(&mut self) {
        let [pos1, pos2, pos3] = self.get_positions();
        self.code[pos3] = self.code[pos1] + self.code[pos2];
        self.index += 4;
    }
    fn mul(&mut self) {
        let [pos1, pos2, pos3] = self.get_positions();
        self.code[pos3] = self.code[pos1] * self.code[pos2];
        self.index += 4;
    }
}
