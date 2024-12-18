use std::fs::read_to_string;

type Num = u32;

struct Computer {
    a: Num,
    b: Num,
    c: Num,
    instruction_pointer: usize,
    program: Vec<Num>,
    out: Vec<Num>,
}

impl Computer {
    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) {
        let operand = self.program[self.instruction_pointer + 1];
        match self.program[self.instruction_pointer] {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("opcode greater than 7"),
        }
    }

    fn combo_operand(&self, operand: Num) -> Num {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("combo operand greater than 6"),
        }
    }

    fn adv(&mut self, operand: Num) {
        let operand = self.combo_operand(operand);
        self.a >>= operand;
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, operand: Num) {
        self.b ^= operand;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, operand: Num) {
        let operand = self.combo_operand(operand);
        self.b = operand & 7;
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, operand: Num) {
        if self.a != 0 {
            self.instruction_pointer = operand.try_into().unwrap();
        } else {
            self.instruction_pointer += 2;
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
        self.instruction_pointer += 2;
    }

    fn out(&mut self, operand: Num) {
        let operand = self.combo_operand(operand);
        self.out.push(operand & 7);
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, operand: Num) {
        let operand = self.combo_operand(operand);
        self.b = self.a >> operand;
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, operand: Num) {
        let operand = self.combo_operand(operand);
        self.c = self.a >> operand;
        self.instruction_pointer += 2;
    }
}

fn main() {
    let mut computer = parse_input(&read_to_string("input.txt").unwrap());
    computer.run();
    println!(
        "{}",
        computer
            .out
            .iter()
            .map(Num::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn parse_input(input: &str) -> Computer {
    let mut input = input.lines();
    let a = input
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let b = input
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let c = input
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    input.next();
    let program = input
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Computer {
        a,
        b,
        c,
        instruction_pointer: 0,
        program,
        out: Vec::new(),
    }
}
