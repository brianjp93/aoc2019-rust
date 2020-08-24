use super::super::utils;
use itertools::Itertools;

pub fn main() {
    let input = utils::read_cs_nums("./src/data/day7.txt");

    let max1 = find_max(input.clone(), false, vec![0, 1, 2, 3, 4]);
    println!("Part 1: {}", max1);

    let max = find_max(input.clone(), true, vec![5, 6, 7, 8, 9]);
    println!("Part 2: {}", max);
}

fn find_max(input: Vec<i64>, feedback: bool, allowed_ints: Vec<i64>) -> i64 {
    let mut max = 0;
    // for perm in permute::permute(allowed_ints) {
    for perm in allowed_ints.into_iter().permutations(5) {
        let out = recursive_run(input.clone(), perm, feedback);
        if out > max {
            max = out.clone();
        }
    }
    max
}


fn recursive_run(input: Vec<i64>, phase: Vec<i64>, feedback: bool) -> i64 {
    let mut amp_a = Computer::new(input.clone());
    let mut amp_b = Computer::new(input.clone());
    let mut amp_c = Computer::new(input.clone());
    let mut amp_d = Computer::new(input.clone());
    let mut amp_e = Computer::new(input.clone());
    amp_a.inputs.extend(vec![phase[0], 0]);
    amp_b.inputs.push(phase[1]);
    amp_c.inputs.push(phase[2]);
    amp_d.inputs.push(phase[3]);
    amp_e.inputs.push(phase[4]);

    while !amp_e.is_done {
        amp_a.inputs.append(&mut amp_e.outputs);
        amp_a.run();

        amp_b.inputs.append(&mut amp_a.outputs);
        amp_b.run();

        amp_c.inputs.append(&mut amp_b.outputs);
        amp_c.run();

        amp_d.inputs.append(&mut amp_c.outputs);
        amp_d.run();

        amp_e.inputs.append(&mut amp_d.outputs);
        amp_e.run();
        if !feedback {
            break;
        }
    }
    amp_e.outputs[0]
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
