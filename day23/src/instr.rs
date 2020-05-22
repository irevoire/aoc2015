use anyhow::{anyhow, bail, Error};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            s => bail!("Unknown register {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Half(Register),
    Triple(Register),
    Increments(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.splitn(2, ' ').map(str::trim);
        let instr = s.next().ok_or(anyhow!("Was expecting an instruction"))?;

        match instr {
            "hlf" => Ok(Self::Half(
                s.next()
                    .ok_or(anyhow!("Half expected a register"))?
                    .parse()?,
            )),
            "tpl" => Ok(Self::Triple(
                s.next()
                    .ok_or(anyhow!("Triple expected a register"))?
                    .parse()?,
            )),
            "inc" => Ok(Self::Increments(
                s.next()
                    .ok_or(anyhow!("Increments expected a register"))?
                    .parse()?,
            )),
            "jmp" => Ok(Self::Jump(
                s.next()
                    .ok_or(anyhow!("Jump expected an offset"))?
                    .parse()?,
            )),
            "jie" => {
                let mut line = s
                    .next()
                    .ok_or(anyhow!("Jump if even expected a register and an offset"))?
                    .split(',')
                    .map(str::trim);
                let register = line.next().unwrap();
                let offset = line
                    .next()
                    .ok_or(anyhow!("Jump if even expected an offset"))?;
                Ok(Self::JumpIfEven(register.parse()?, offset.parse()?))
            }
            "jio" => {
                let mut line = s
                    .next()
                    .ok_or(anyhow!("Jump if even expected a register and an offset"))?
                    .split(',')
                    .map(str::trim);
                let register = line.next().unwrap();
                let offset = line
                    .next()
                    .ok_or(anyhow!("Jump if even expected an offset"))?;
                Ok(Self::JumpIfOne(register.parse()?, offset.parse()?))
            }
            s => bail!("Unknown register {}", s),
        }
    }
}
