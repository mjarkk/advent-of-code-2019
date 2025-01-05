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

enum Interupt {
    Input,
    Output(i32),
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

    let mut runtime = Runtime::default();
    for code in puzzle.split(',') {
        let code: i32 = if let Ok(v) = code.parse() {
            v
        } else {
            code.split('\n').nth(0).unwrap().parse().unwrap()
        };
        runtime.source_memory.push(code);
    }

    let max_thrust = runtime.find_best_thrust_phase_settings_p1(0, 0, &mut [0, 0, 0, 0, 0]);
    println!("p1: {}", max_thrust);

    let max_thrust = runtime.find_best_thrust_phase_settings_p2(0, 0, &mut [0, 0, 0, 0, 0]);
    println!("p2: {}", max_thrust);

    println!("Elapsed: {:.2?}", now.elapsed());
}

#[derive(Default)]
struct Runtime {
    source_memory: Vec<i32>,
    ampifiers: [Program; 5],
}

impl Runtime {
    fn find_best_thrust_phase_settings_p1(
        &mut self,
        offset: usize,
        flags: u8,
        memory: &mut [i32; 5],
    ) -> i32 {
        let mut max_thrust = 0;

        for setting in 0..=4 {
            let flag = 1 << setting;
            if flags & flag != 0 {
                continue;
            }

            let new_flags = flags | flag;
            memory[offset] = setting;

            let thrust = if offset == 4 {
                self.calculate_thrust(memory)
            } else {
                self.find_best_thrust_phase_settings_p1(offset + 1, new_flags, memory)
            };
            if thrust > max_thrust {
                max_thrust = thrust;
            }
        }

        max_thrust
    }
    fn find_best_thrust_phase_settings_p2(
        &mut self,
        offset: usize,
        flags: u8,
        memory: &mut [i32; 5],
    ) -> i32 {
        let mut max_thrust = 0;

        for setting in 5..=9 {
            let flag = 1 << (setting - 5);
            if flags & flag != 0 {
                continue;
            }

            let new_flags = flags | flag;
            memory[offset] = setting as i32;

            let thrust = if offset == 4 {
                self.calculate_thrust(memory)
            } else {
                self.find_best_thrust_phase_settings_p2(offset + 1, new_flags, memory)
            };
            if thrust > max_thrust {
                max_thrust = thrust;
            }
        }

        max_thrust
    }

    fn calculate_thrust(&mut self, phase_settings: &mut [i32; 5]) -> i32 {
        self.reset_ampifiers();

        let mut inputs = Vec::new();
        let mut last_out = 0;
        for (idx, amp) in self.ampifiers.iter_mut().enumerate() {
            inputs.clear();
            inputs.push(phase_settings[idx]);
            inputs.push(last_out);

            match amp.start(&mut inputs) {
                Interupt::Input => {
                    panic!("this should never be called");
                }
                Interupt::Output(out) => {
                    last_out = out;
                }
                Interupt::Halt => {
                    return last_out;
                }
            }
        }

        loop {
            for amp in self.ampifiers.iter_mut() {
                inputs.clear();
                inputs.push(last_out);

                match amp.start(&mut inputs) {
                    Interupt::Input => {
                        panic!("this should never be called");
                    }
                    Interupt::Output(out) => {
                        last_out = out;
                    }
                    Interupt::Halt => {
                        return last_out;
                    }
                }
            }
        }
    }
    fn reset_ampifiers(&mut self) {
        for amp in self.ampifiers.iter_mut() {
            amp.memory = self.source_memory.clone();
            amp.instruction_pointer = 0;
        }
    }
}

#[derive(Clone, Default)]
struct Program {
    memory: Vec<i32>,
    instruction_pointer: usize,
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
    fn start(&mut self, input: &mut Vec<i32>) -> Interupt {
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

                    if input.len() == 0 {
                        return Interupt::Input;
                    }

                    self.memory[a_ptr as usize] = input.remove(0);
                    self.instruction_pointer += 2;
                }
                Inst::Output => {
                    let a_ptr = self.memory[self.instruction_pointer + 1];

                    self.instruction_pointer += 2;
                    return Interupt::Output(self.memory[a_ptr as usize]);
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
                    return Interupt::Halt;
                }
            }
        }
    }
}
