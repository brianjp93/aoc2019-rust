use super::super::utils;

pub fn main() {
    let input = utils::read_cs_nums("./src/data/day5.txt");
    let mut comp = Computer::new(input.clone());
    comp.inputs.push(1);
    comp.run();
}

struct Computer {
    code: Vec<i64>,
    index: usize,
    inputs: Vec<i64>,
}

impl Computer {
    fn new(code: Vec<i64>) -> Self {
        let mut x = Computer { code, index: 0, inputs: Vec::<i64>::new() };
        x.code.extend((1..1000).map(|_x| 0).collect::<Vec<i64>>());
        x
    }
    fn run(&mut self) {
        while self.get_opcode() != 99 {
            let [ans, pos, new_index] = match self.get_opcode() {
                1 => self.add(),
                2 => self.mul(),
                3 => self.get_input(),
                4 => self.output(),
                _ => {
                    println!("wtf");
                    break
                },
            };
            if self.get_opcode() != 4 {
                self.code[pos as usize] = ans;
            }
            self.index = new_index as usize;
        }
    }
    fn get_opcode(&self) -> i64 {
        self.code[self.index as usize] % 100
    }
    fn get_positions(&self) -> [i64; 3] {
        let opcode = self.code[self.index as usize];
        // let e = &opcode % 100;
        let c = (opcode / 100) % 10;
        let b = (opcode / 1000) % 10;
        let a = (opcode / 10000) % 10;
        let mut out = [0, 0, 0];
        for (i, mode) in [c, b, a].iter().enumerate() {
            if *mode == 0 {
                out[i] = self.code[self.index + i + 1];
            } else {
                out[i] = (self.index + i + 1) as i64;
            }
        }
        out
    }
    fn get_input(&mut self) -> [i64; 3] {
        let [pos1, _, _] = self.get_positions();
        let val = self.inputs.pop().unwrap();
        [val, pos1, self.index as i64 + 2]
    }
    fn output(&self) -> [i64; 3] {
        let [pos1, _, _] = self.get_positions();
        println!("output: {}", self.code[pos1 as usize]);
        [0, 0, self.index as i64 + 2]
    }
    fn add(&self) -> [i64; 3] {
        let [pos1, pos2, pos3] = self.get_positions();
        let ans = self.code[pos1 as usize] + self.code[pos2 as usize];
        [ans, pos3, self.index as i64 + 4]
    }
    fn mul(&self) -> [i64; 3] {
        let [pos1, pos2, pos3] = self.get_positions();
        let ans = self.code[pos1 as usize] * self.code[pos2 as usize];
        [ans, pos3, self.index as i64 + 4]
    }
}
