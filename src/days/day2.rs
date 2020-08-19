use super::super::utils;

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


fn find_pair(code: Vec<i64>) -> String {
    let mut output: Vec<i64>;
    for x in 0..100 {
        for y in 0..100 {
            let mut temp_code = code.clone();
            temp_code[1] = x;
            temp_code[2] = y;
            let comp = Computer { code: temp_code, index: 0 };
            output = comp.run();
            if output[0] == 19690720 {
                return String::from(format!("{:02}{:02}", x, y))
            }
        }
    }
    String::from("")
}

pub fn main() {
    let mut input = utils::read_cs_nums("./src/data/day2.txt");
    let extra = (1..1000).map(|_x| 0).collect::<Vec<i64>>();
    input.extend(extra);
    let input2 = input.clone();
    input[1] = 12;
    input[2] = 2;
    let comp = Computer {
        code: input,
        index: 0,
    };
    let output = comp.run();
    let output2 = find_pair(input2);
    println!();
    println!("___Day 2___");
    println!("Part 1: {}", output[0]);
    println!("Part 2: {}", output2);
}
