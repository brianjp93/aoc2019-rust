use super::super::utils;


struct Computer {
    code: Vec<i64>,
    index: usize,
}


impl Computer {
    fn default() -> Self {
        Self {
            code: Vec::<i64>::new(),
            index: 0,
        }
    }
    fn next(self) {
        let mut opcode = self.code[self.index as usize];
        while opcode != 99 {
            match opcode {
                1 => self.add(),
                2 => self.mul(),
                _ => print!("unknown option"),
            };
            opcode = self.code[self.index as usize];
        }
    }
    fn add(mut self) {
        let pos1 = self.code[(self.index + 1) as usize] as usize;
        let pos2 = self.code[(self.index + 2) as usize] as usize;
        let pos3 = self.code[(self.index + 3) as usize] as usize;
        self.code[pos3] = self.code[pos1] + self.code[pos2];
        self.index += 4;
    }
    fn mul(mut self) {
        let pos1 = self.code[(self.index + 1) as usize] as usize;
        let pos2 = self.code[(self.index + 2) as usize] as usize;
        let pos3 = self.code[(self.index + 3) as usize] as usize;
        self.code[pos3] = self.code[pos1] * self.code[pos2];
        self.index += 4;
    }
}


pub fn main() {
    let input = utils::read_cs_nums("./src/data/day2.txt");
    let comp = Computer { code: input, ..Computer::default() };
    println!("{:?}", comp.code);
    println!("{}", comp.index);
}
