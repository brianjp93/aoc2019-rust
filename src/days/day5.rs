use super::super::utils;

pub fn main() {
    let input = utils::read_cs_nums("./src/data/day5.txt");
    let mut comp = Computer::new(input.clone());
    comp.inputs.push(1);
    comp.run();
    println!("Part 1: {}", comp.outputs.last().unwrap());

    let mut comp = Computer::new(input.clone());
    comp.inputs.push(5);
    comp.run();
    println!("Part 2: {}", comp.outputs.last().unwrap());
}

struct Computer {
    code: Vec<i64>,
    index: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl Computer {
    fn new(code: Vec<i64>) -> Self {
        let mut x = Computer {
            code,
            index: 0,
            inputs: Vec::<i64>::new(),
            outputs: Vec::<i64>::new(),
        };
        x.code.extend((1..100).map(|_| 0).collect::<Vec<i64>>());
        x
    }
    fn run(&mut self) {
        loop {
            match self.get_opcode() {
                1 => self.add(),
                2 => self.mul(),
                3 => self.get_input(),
                4 => self.output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                99 => {
                    break;
                }
                _ => {
                    println!("wtf");
                    break;
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
        let mut out = [0 as usize, 0 as usize, 0 as usize];
        for (i, mode) in [c, b, a].iter().enumerate() {
            if *mode == 0 {
                out[i] = self.code[self.index + i + 1] as usize;
            } else {
                out[i] = self.index + i + 1;
            }
        }
        out
    }
    fn less_than(&mut self) {
        let [pos1, pos2, pos3] = self.get_positions();
        if self.code[pos1] < self.code[pos2] {
            self.code[pos3] = 1;
        } else {
            self.code[pos3] = 0;
        }
        self.index += 4;
    }
    fn equals(&mut self) {
        let [pos1, pos2, pos3] = self.get_positions();
        if self.code[pos1] == self.code[pos2] {
            self.code[pos3] = 1;
        } else {
            self.code[pos3] = 0;
        }
        self.index += 4;
    }
    fn jump_if_true(&mut self) {
        let [pos1, pos2, _] = self.get_positions();
        if self.code[pos1] != 0 {
            self.index = self.code[pos2] as usize;
        } else {
            self.index += 3;
        }
    }
    fn jump_if_false(&mut self) {
        let [pos1, pos2, _] = self.get_positions();
        if self.code[pos1] == 0 {
            self.index = self.code[pos2] as usize;
        } else {
            self.index += 3;
        }
    }
    fn get_input(&mut self) {
        let [pos1, _, _] = self.get_positions();
        let val = self.inputs.pop().unwrap();
        self.code[pos1] = val;
        self.index += 2;
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
