use core::panic;
use std::fs;
use std::time::Instant;

enum Inst {
    Add,                // 1
    Multiply,           // 2
    Input,              // 3
    Output,             // 4
    JumpIfTrue,         // 5
    JumpIfFalse,        // 6
    LessThan,           // 7
    Equals,             // 8
    AdjustRelativeBase, // 9
    Halt,               // 99
}

enum Interupt {
    Input,
    Output(i64),
    Halt,
}

struct Opcode {
    inst: Inst,
    parameter_mode: u32,
}

struct ParameterMode {
    mode: u8,
    offset: usize,
}

impl Opcode {
    fn from(code: i64) -> Self {
        let inst = match code % 100 {
            1 => Inst::Add,
            2 => Inst::Multiply,
            3 => Inst::Input,
            4 => Inst::Output,
            5 => Inst::JumpIfTrue,
            6 => Inst::JumpIfFalse,
            7 => Inst::LessThan,
            8 => Inst::Equals,
            9 => Inst::AdjustRelativeBase,
            99 => Inst::Halt,
            _ => panic!("unknown opcode {}", code),
        };

        Self {
            inst,
            parameter_mode: code as u32,
        }
    }
    fn a(&self) -> ParameterMode {
        ParameterMode {
            mode: (self.parameter_mode / 100 % 10) as u8,
            offset: 1,
        }
    }
    fn b(&self) -> ParameterMode {
        ParameterMode {
            mode: (self.parameter_mode / 1000 % 10) as u8,
            offset: 2,
        }
    }
    fn c(&self) -> ParameterMode {
        ParameterMode {
            mode: (self.parameter_mode / 10000 % 10) as u8,
            offset: 3,
        }
    }
}

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut runtime = Runtime::default();
    for code in puzzle.split(',') {
        let code: i64 = if let Ok(v) = code.parse() {
            v
        } else {
            code.split('\n').nth(0).unwrap().parse().unwrap()
        };
        runtime.source_memory.push(code);
    }

    let out = runtime.run(1);
    println!("{}", out);

    // For benchmarking
    // for _ in 0..3000 {
    //     runtime.run(2);
    // }

    let out = runtime.run(2);
    println!("{}", out);

    println!("Elapsed: {:.2?}", now.elapsed());
}

#[derive(Default)]
struct Runtime {
    source_memory: Vec<i64>,
    program: Program,
}

impl Runtime {
    fn run(&mut self, mode: i64) -> i64 {
        self.reset();

        let mut last_out = 0;
        loop {
            match self.program.start(&mut vec![mode]) {
                Interupt::Halt => break,
                Interupt::Input => panic!("unexpected interupt input"),
                Interupt::Output(out) => {
                    last_out = out;
                    // print!("{},", out)
                }
            };
        }
        // println!();

        last_out
    }
    fn reset(&mut self) {
        self.program.memory = self.source_memory.clone();
        self.program.instruction_pointer = 0;
        self.program.relative_base = 0;
    }
}

#[derive(Clone, Default)]
struct Program {
    memory: Vec<i64>,
    instruction_pointer: usize,
    relative_base: i64,
}

impl Program {
    fn param(&self, param: ParameterMode) -> i64 {
        let raw_value = self.memory[self.instruction_pointer + param.offset];

        let addr = match param.mode {
            0 /* Ptr to value */ => raw_value,
            1 /* Litteral value */ => return raw_value,
            2 /* Relative base */ => self.relative_base + raw_value,
            f => panic!("unknown flag {}", f),
        };

        if addr < 0 {
            return 0;
        }

        let addr_usize = addr as usize;
        if addr_usize >= self.memory.len() {
            return 0;
        }

        self.memory[addr_usize]
    }
    fn write(&mut self, param: ParameterMode, value: i64) {
        let mut addr = self.memory[self.instruction_pointer + param.offset];
        if param.mode == 2 {
            addr += self.relative_base;
        }
        let addr_usize = addr as usize;

        if addr_usize >= self.memory.len() {
            self.memory.resize(addr_usize + 100, 0);
        }

        self.memory[addr_usize] = value;
    }
    fn start(&mut self, input: &mut Vec<i64>) -> Interupt {
        loop {
            let opcode = Opcode::from(self.memory[self.instruction_pointer]);

            match opcode.inst {
                Inst::Add => {
                    let value = self.param(opcode.a()) + self.param(opcode.b());
                    self.write(opcode.c(), value);
                    self.instruction_pointer += 4;
                }
                Inst::Multiply => {
                    let value = self.param(opcode.a()) * self.param(opcode.b());
                    self.write(opcode.c(), value);
                    self.instruction_pointer += 4;
                }
                Inst::Input => {
                    if input.len() == 0 {
                        return Interupt::Input;
                    }

                    self.write(opcode.a(), input.remove(0));
                    self.instruction_pointer += 2;
                }
                Inst::Output => {
                    let a = self.param(opcode.a());

                    self.instruction_pointer += 2;

                    return Interupt::Output(a);
                }
                Inst::JumpIfTrue => {
                    let a = self.param(opcode.a());
                    let b = self.param(opcode.b());

                    if a != 0 {
                        self.instruction_pointer = b as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Inst::JumpIfFalse => {
                    if self.param(opcode.a()) == 0 {
                        self.instruction_pointer = self.param(opcode.b()) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Inst::LessThan => {
                    let a = self.param(opcode.a());
                    let b = self.param(opcode.b());
                    let value = if a < b { 1 } else { 0 };
                    self.write(opcode.c(), value);
                    self.instruction_pointer += 4;
                }
                Inst::Equals => {
                    let a = self.param(opcode.a());
                    let b = self.param(opcode.b());
                    let value = if a == b { 1 } else { 0 };
                    self.write(opcode.c(), value);
                    self.instruction_pointer += 4;
                }
                Inst::AdjustRelativeBase => {
                    let a = self.param(opcode.a());

                    self.relative_base += a;
                    self.instruction_pointer += 2;
                }
                Inst::Halt => {
                    return Interupt::Halt;
                }
            }
        }
    }
}
