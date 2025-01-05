use std::fs;
use std::time::Instant;

enum Inst {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

struct Opcode {
    inst: Inst,
    flags: u8,
}

impl Opcode {
    fn from_code(mut code: i32) -> Self {
        let inst = match code % 100 {
            1 => Inst::Add,
            2 => Inst::Multiply,
            3 => Inst::Input,
            4 => Inst::Output,
            5 => Inst::JumpIfTrue,
            6 => Inst::JumpIfFalse,
            7 => Inst::LessThan,
            8 => Inst::Equals,
            99 => Inst::Halt,
            _ => panic!("unknown opcode {}", code),
        };

        let mut flags = 0;
        code = code / 100;
        if code % 10 == 1 {
            flags |= 1;
        };
        code = code / 10;
        if code % 10 == 1 {
            flags |= 2;
        };
        code = code / 10;
        if code % 10 == 1 {
            flags |= 4;
        };

        Self { inst, flags }
    }
}

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut program = Program::default();
    for code in puzzle.split(',') {
        let code: i32 = if let Ok(v) = code.parse() {
            v
        } else {
            code.split('\n').nth(0).unwrap().parse().unwrap()
        };
        program.memory.push(code);
    }

    let mut part_1_program = program.clone();
    part_1_program.input.push(1);
    part_1_program.run();
    println!("{}", part_1_program.output.pop().unwrap());

    program.input.push(5);
    program.run();
    println!("{}", program.output.pop().unwrap());

    println!("Elapsed: {:.2?}", now.elapsed());
}

#[derive(Debug, Default, Clone)]
struct Program {
    memory: Vec<i32>,
    instruction_pointer: usize,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl Program {
    fn a(&self, opcode: &Opcode) -> i32 {
        let raw_value = self.memory[self.instruction_pointer + 1];
        let parameter_mode = opcode.flags & 1;
        match parameter_mode {
            0 /* Ptr to value */ => self.memory[raw_value as usize],
            1 /* Litteral value */ => raw_value,
            _ => panic!("unknown flag"),
        }
    }
    fn b(&self, opcode: &Opcode) -> i32 {
        let raw_value = self.memory[self.instruction_pointer + 2];
        let parameter_mode = opcode.flags >> 1 & 1;
        match parameter_mode {
            0 /* Ptr to value */ => self.memory[raw_value as usize],
            1 /* Litteral value */ => raw_value,
            _ => panic!("unknown flag"),
        }
    }
    // fn c(&self, opcode: &Opcode) -> i32 {
    //     let raw_value = self.memory[self.instruction_pointer + 3];
    //     let parameter_mode = opcode.flags >> 2 & 1;
    //     match parameter_mode {
    //         0 /* Ptr to value */ => self.memory[raw_value as usize],
    //         1 /* Litteral value */ => raw_value,
    //         _ => panic!("unknown flag"),
    //     }
    // }

    fn run(&mut self) {
        loop {
            let opcode = Opcode::from_code(self.memory[self.instruction_pointer]);

            match opcode.inst {
                Inst::Add => {
                    let c_ptr = self.memory[self.instruction_pointer + 3];

                    self.memory[c_ptr as usize] = self.a(&opcode) + self.b(&opcode);
                    self.instruction_pointer += 4;
                }
                Inst::Multiply => {
                    let c_ptr = self.memory[self.instruction_pointer + 3];

                    self.memory[c_ptr as usize] = self.a(&opcode) * self.b(&opcode);
                    self.instruction_pointer += 4;
                }
                Inst::Input => {
                    let a_ptr = self.memory[self.instruction_pointer + 1];

                    if let Some(v) = self.input.pop() {
                        self.memory[a_ptr as usize] = v;
                    } else {
                        self.memory[a_ptr as usize] = 0;
                    }
                    self.instruction_pointer += 2;
                }
                Inst::Output => {
                    let a_ptr = self.memory[self.instruction_pointer + 1];

                    self.output.push(self.memory[a_ptr as usize]);
                    self.instruction_pointer += 2;
                }
                Inst::JumpIfTrue => {
                    let a = self.a(&opcode);
                    let b = self.b(&opcode);

                    if a != 0 {
                        self.instruction_pointer = b as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Inst::JumpIfFalse => {
                    if self.a(&opcode) == 0 {
                        self.instruction_pointer = self.b(&opcode) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Inst::LessThan => {
                    let c_ptr = self.memory[self.instruction_pointer + 3];

                    self.memory[c_ptr as usize] = if self.a(&opcode) < self.b(&opcode) {
                        1
                    } else {
                        0
                    };
                    self.instruction_pointer += 4;
                }
                Inst::Equals => {
                    let c_ptr = self.memory[self.instruction_pointer + 3];

                    self.memory[c_ptr as usize] = if self.a(&opcode) == self.b(&opcode) {
                        1
                    } else {
                        0
                    };
                    self.instruction_pointer += 4;
                }
                Inst::Halt => {
                    break;
                }
            }
        }
    }
}
