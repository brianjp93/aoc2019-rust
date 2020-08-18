use super::super::utils;


struct Computer {
    code: Vec<i64>,
}


impl Computer {
    fn add(mut self, pos1: i64, pos2: i64, pos3: i64) {
        self.code[pos3 as usize] = self.code[pos1 as usize] + self.code[pos2 as usize];
    }
}


pub fn main() {
    let input = utils::read_cs_nums("./src/data/day2.txt");
    let comp = Computer { code: input };
    println!("{:?}", comp.code);
}
