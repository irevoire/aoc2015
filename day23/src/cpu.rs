use crate::*;

#[derive(Default, Debug, Clone)]
pub struct Cpu {
    // registers
    pub a: u32,
    pub b: u32,

    // program counter
    pub pc: usize,

    pub memory: Vec<Instr>,
}

impl Cpu {
    pub fn with_instructions(mut self, memory: Vec<Instr>) -> Self {
        self.memory = memory;
        self
    }

    pub fn cycle(&mut self) {
        let instruction = *self
            .memory
            .get(self.pc)
            .expect(&format!("Invalid acces to address {}", self.pc));
        self.pc += 1;
        match instruction {
            Instr::Half(reg) => *self.get_mut(reg) /= 2,
            Instr::Triple(reg) => *self.get_mut(reg) *= 3,
            Instr::Increments(reg) => *self.get_mut(reg) += 1,
            Instr::Jump(offset) => self.pc = (self.pc as isize + offset - 1) as usize,
            Instr::JumpIfEven(reg, offset) if self.get(reg) % 2 == 0 => {
                self.pc = (self.pc as isize + offset - 1) as usize
            }
            Instr::JumpIfOne(reg, offset) if self.get(reg) == 1 => {
                self.pc = (self.pc as isize + offset - 1) as usize
            }
            _ => (),
        }
    }

    pub fn finished(&self) -> bool {
        self.pc >= self.memory.len()
    }

    pub fn get(&self, r: Register) -> u32 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    pub fn get_mut(&mut self, r: Register) -> &mut u32 {
        match r {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}
