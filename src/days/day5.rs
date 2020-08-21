use super::super::utils;

pub fn main() {
    let input = utils::read_cs_nums("./src/data/day5.txt");
    println!("{:?}", input);
}

struct Computer {
    code: Vec<i64>,
    index: usize,
}

impl Computer {
    fn run(mut self) -> Vec<i64> {
        while self.get_opcode() != 99 {
            let [ans, pos, new_index] = match self.get_opcode() {
                1 => self.add(),
                2 => self.mul(),
                _ => break,
            };
            self.code[pos as usize] = ans;
            self.index = new_index as usize;
        }
        self.code
    }
    fn get_opcode(&self) -> i64 {
        self.code[self.index as usize]
    }
    fn add(&self) -> [i64; 3] {
        let pos1 = self.code[(self.index + 1) as usize] as usize;
        let pos2 = self.code[(self.index + 2) as usize] as usize;
        let pos3 = self.code[(self.index + 3) as usize];
        let ans = self.code[pos1] + self.code[pos2];
        [ans, pos3, self.index as i64 + 4]
    }
    fn mul(&self) -> [i64; 3] {
        let pos1 = self.code[(self.index + 1) as usize] as usize;
        let pos2 = self.code[(self.index + 2) as usize] as usize;
        let pos3 = self.code[(self.index + 3) as usize];
        let ans = self.code[pos1] * self.code[pos2];
        [ans, pos3, self.index as i64 + 4]
    }
}
