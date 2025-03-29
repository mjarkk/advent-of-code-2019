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

#[derive(Debug)]
pub enum Interupt {
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

#[derive(Clone)]
pub enum Flag {
    Unflagged,
    Inst,
    Param,
    ReadWrite,
    Read,
    Write,
}

impl Default for Flag {
    fn default() -> Self {
        Flag::Unflagged
    }
}

#[derive(Clone, Default)]
pub struct Program {
    pub memory: Vec<i64>,
    pub memory_flags: Vec<Flag>,
    pub instruction_pointer: usize,
    pub relative_base: i64,
}

impl Program {
    pub fn reset(&mut self, memory: Vec<i64>) {
        self.memory = memory;
        self.memory_flags = Vec::new();
        self.memory_flags.resize(self.memory.len(), Flag::Unflagged);
        self.instruction_pointer = 0;
        self.relative_base = 0;
    }
    fn param(&mut self, param: ParameterMode) -> i64 {
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

        self.flag_read(addr_usize);
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

        self.flag_write(addr_usize);
        self.memory[addr_usize] = value;
    }
    fn flag(&mut self) {
        if self.instruction_pointer >= self.memory_flags.len() {
            self.memory_flags
                .resize(self.instruction_pointer + 1, Flag::Unflagged);
        }
        match self.memory_flags[self.instruction_pointer] {
            Flag::Unflagged | Flag::Read | Flag::Write | Flag::ReadWrite => {}
            Flag::Inst => return,
            Flag::Param => panic!("want to flag inst but already flagged param"),
        }
        self.memory_flags[self.instruction_pointer] = Flag::Inst;
    }
    fn flag_a(&mut self) {
        self.flag_param(1);
    }
    fn flag_b(&mut self) {
        self.flag_param(1);
        self.flag_param(2);
    }
    fn flag_c(&mut self) {
        self.flag_param(1);
        self.flag_param(2);
        self.flag_param(3);
    }
    fn flag_param(&mut self, offset: usize) {
        let addr = self.instruction_pointer + offset;
        if addr >= self.memory_flags.len() {
            self.memory_flags.resize(addr + 1, Flag::Unflagged);
        }
        match self.memory_flags[addr] {
            Flag::Unflagged | Flag::Read | Flag::Write | Flag::ReadWrite => {}
            Flag::Inst => panic!("want to flag param but already flagged inst"),
            Flag::Param => return,
        }
        self.memory_flags[addr] = Flag::Param;
    }
    fn flag_read(&mut self, offset: usize) {
        let addr = self.instruction_pointer + offset;
        if addr >= self.memory_flags.len() {
            self.memory_flags.resize(addr + 1, Flag::Unflagged);
        }

        self.memory_flags[addr] = match self.memory_flags[addr] {
            Flag::Read | Flag::ReadWrite | Flag::Inst | Flag::Param => return,
            Flag::Write => Flag::ReadWrite,
            Flag::Unflagged => Flag::Read,
        };
    }
    fn flag_write(&mut self, offset: usize) {
        let addr = self.instruction_pointer + offset;
        if addr >= self.memory_flags.len() {
            self.memory_flags.resize(addr + 1, Flag::Unflagged);
        }

        self.memory_flags[addr] = match self.memory_flags[addr] {
            Flag::Write | Flag::ReadWrite | Flag::Inst | Flag::Param => return,
            Flag::Read => Flag::ReadWrite,
            Flag::Unflagged => Flag::Write,
        };
    }
    pub fn run(&mut self, input: &mut Vec<i64>) -> Interupt {
        loop {
            let opcode = Opcode::from(self.memory[self.instruction_pointer]);
            self.flag();

            match opcode.inst {
                Inst::Add => {
                    self.flag_c();
                    let value = self.param(opcode.a()) + self.param(opcode.b());
                    self.write(opcode.c(), value);
                    self.instruction_pointer += 4;
                }
                Inst::Multiply => {
                    self.flag_c();
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
                    self.flag_a();
                    let a = self.param(opcode.a());

                    self.instruction_pointer += 2;

                    return Interupt::Output(a);
                }
                Inst::JumpIfTrue => {
                    self.flag_b();
                    let a = self.param(opcode.a());
                    let b = self.param(opcode.b());

                    if a != 0 {
                        self.instruction_pointer = b as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Inst::JumpIfFalse => {
                    self.flag_b();
                    if self.param(opcode.a()) == 0 {
                        self.instruction_pointer = self.param(opcode.b()) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Inst::LessThan => {
                    self.flag_c();
                    let a = self.param(opcode.a());
                    let b = self.param(opcode.b());
                    let value = if a < b { 1 } else { 0 };
                    self.write(opcode.c(), value);
                    self.instruction_pointer += 4;
                }
                Inst::Equals => {
                    self.flag_c();
                    let a = self.param(opcode.a());
                    let b = self.param(opcode.b());
                    let value = if a == b { 1 } else { 0 };
                    self.write(opcode.c(), value);
                    self.instruction_pointer += 4;
                }
                Inst::AdjustRelativeBase => {
                    self.flag_a();
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
