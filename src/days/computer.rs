pub struct Computer {
    pub code: Vec<i64>,
    pub index: usize,
    pub relative_base: isize,
    pub inputs: Vec<i64>,
    pub outputs: Vec<i64>,
    pub is_done: bool,
    pub is_waiting: bool,
}

impl Computer {
    pub fn new(code: Vec<i64>) -> Self {
        let mut x = Computer {
            code,
            index: 0,
            relative_base: 0,
            inputs: Vec::<i64>::new(),
            outputs: Vec::<i64>::new(),
            is_done: false,
            is_waiting: false,
        };
        x.code.extend(vec![0; 1000]);
        x
    }
    pub fn run(&mut self) {
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
                9 => self.adjust_base(),
                99 => {
                    self.is_done = true;
                    break 'loops;
                }
                _ => {
                    println!("wtf: {}", self.get_opcode());
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
            } else if *mode == 1 {
                self.index + i + 1
            } else {
                (self.code[self.index + i + 1] as isize + self.relative_base) as usize
            };
        }
        out
    }
    fn adjust_base(&mut self) {
        let [pos1, _, _] = self.get_positions();
        self.relative_base += self.code[pos1] as isize;
        self.index += 2;
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
