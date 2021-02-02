use std::str::FromStr;
use std::num::ParseIntError;

const SPLIT_INDEX: usize = 8;

type Register = usize;
type Address = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    NOP,
    HLT,
    JMP(Register),
    PRN(Register),
    LDI(Register, u8),
}

#[derive(Clone, Debug)]
pub struct VM {
    pub ip: Address,
    pub terminated: bool,
    pub code: Vec<u8>,
    pub registers: [Register; 8],
}

#[derive(Debug)]
pub enum InstructionError {
    MissingOperation,
    MissingArgument,
    InvalidInstruction(String),
    InvalidArgumentValue(ParseIntError),
}

impl VM {
    fn decode(&mut self) -> Result<Instruction, InstructionError> {
        let decoded = match self.code[self.ip] {
            0 => Instruction::NOP,
            1 => Instruction::HLT,
            3 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::JMP(reg)
            },
            71 => {
                // PRN instruction 
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::PRN(reg)
            },
            130 => {
                // LDI instruction 
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                self.ip += 1;
                let arg = self.code[self.ip];
                Instruction::LDI(reg, arg)
            },
            code => return Err(InstructionError::InvalidInstruction(format!("{:?}", code))),
        };

        Ok(decoded)
    }

    fn execute(&mut self, inst: Instruction) -> bool {
        match inst {
            Instruction::NOP => true,
            Instruction::HLT => {
                self.terminated = true;
                true
            },
            Instruction::PRN(reg) => {
                let val = &self.registers[reg];
                println!("{:?}", val);
                true
            },
            Instruction::LDI(reg, arg) => {
                self.registers[reg] = arg as usize;
                true
            },
            _ => false,
        }
    }

    pub fn run(&mut self) {
        if !self.terminated { return; }

        self.terminated = false;

        while self.ip < self.code.len() && !self.terminated {
            // decode the next instruction and its arguments
            let inst = self.decode().expect("Failed to decode an instruction");
            let result = self.execute(inst);

            if !result {
                panic!("Failed to execute instruction at address: {}", self.ip);
            }

            self.ip += 1;
        }
    } 
}

impl FromStr for VM {
    type Err = InstructionError;

    fn from_str(program: &str) -> Result<Self, Self::Err> {
        let code = program.lines()
            .filter(|line| !line.starts_with('#') && line.len() >= 8)
            .map(|line| {
                let (inst, _) = line.split_at(SPLIT_INDEX);
                u8::from_str_radix(inst, 2)
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Ok(VM {
            ip: 0,
            terminated: true,
            code,
            registers: [0; 8],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_from_str() {
        let input = "10000010
00000000
00001000
01000111
00000000
00000001";
        
        let expected = vec![130, 0, 8, 71, 0, 1];
        let vm = VM::from_str(input).expect("Failed to parse input in `test_vm_from_str`");

        assert_eq!(vm.code, expected);
    }   

    #[test]
    fn test_vm_from_str_with_comments() {
        let input = "
# This comment and blank line is here to make sure
# they are handled correctly by the file reading code.

10000010 # LDI R0,8
00000000
00001000
01000111 # PRN R0
00000000
00000001 # HLT"; 

        let expected = vec![130, 0, 8, 71, 0, 1];
        let vm = VM::from_str(input).expect("Failed to parse input in `test_vm_from_str_with_comments`");

        assert_eq!(vm.code, expected);
    }
}

