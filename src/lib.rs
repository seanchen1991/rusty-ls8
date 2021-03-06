use std::str::FromStr;
use std::num::ParseIntError;

const SPLIT_INDEX: usize = 8;

type Register = usize;
type Address = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    NOP,
    HLT,
    RET,
    JMP(Register),
    PRN(Register),
    CALL(Register),
    PUSH(Register),
    POP(Register),
    LDI(Register, u8),
    MUL(Register, Register),
    DIV(Register, Register),
}

#[derive(Clone, Debug)]
pub struct VM {
    pub ip: Address,
    pub terminated: bool,
    pub code: Vec<u8>,
    pub stack: Vec<u8>,
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
            17 => Instruction::RET,
            69 => {
                // PUSH instruction
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::PUSH(reg)
            },
            70 => {
                // POP instruction   
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::POP(reg)
            },
            71 => {
                // PRN instruction 
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::PRN(reg)
            },
            80 => {
                // CALL instruction   
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::CALL(reg)
            },
            130 => {
                // LDI instruction 
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                self.ip += 1;
                let arg = self.code[self.ip];
                Instruction::LDI(reg, arg)
            },
            162 => {
                // MUL instruction
                self.ip += 1;
                let areg = self.code[self.ip] as Register;
                self.ip += 1;
                let breg = self.code[self.ip] as Register;
                Instruction::MUL(areg, breg)
            },
            163 => {
                // DIV instruction
                self.ip += 1;
                let areg = self.code[self.ip] as Register;
                self.ip += 1;
                let breg = self.code[self.ip] as Register;
                Instruction::DIV(areg, breg)
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
            Instruction::JMP(reg) => {
                let dest = self.registers[reg] as Address;
                self.ip = dest;
                true
            },
            Instruction::PUSH(reg) => {
                let val = self.registers[reg] as u8;
                self.stack.push(val);
                true
            },
            Instruction::POP(reg) => {
                if let Some(top) = self.stack.pop() {
                    self.registers[reg] = top as usize;
                    true 
                } else {
                    false
                }
            },
            Instruction::CALL(reg) => {
                self.stack.push(self.ip as u8 + 1);
                self.execute(Instruction::JMP(reg))
            },
            Instruction::RET => {
                match self.stack.pop() {
                    Some(v) => {
                        self.ip = v as Address;
                        true
                    },
                    None => false,
                }   
            },
            Instruction::LDI(reg, arg) => {
                self.registers[reg] = arg as usize;
                true
            },
            Instruction::MUL(areg, breg) => {
                let aval = &self.registers[areg];
                let bval = &self.registers[breg];
                self.registers[areg] = aval * bval;
                true
            },
            Instruction::DIV(areg, breg) => {
                let aval = &self.registers[areg];
                let bval = &self.registers[breg];
                self.registers[areg] = aval / bval;
                true
            }
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
            stack: Vec::new(),
            registers: [0; 8],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FILENAME: &str = "asm/print8.ls8";

    #[test]
    fn test_vm_from_str() {
        let input = fs::read_to_string(TEST_FILENAME).expect("Failed to read file in `test_vm_from_str_with_comments`");

        let expected = vec![130, 0, 8, 71, 0, 1];
        let vm = VM::from_str(&input).expect("Failed to parse input in `test_vm_from_str_with_comments`");

        assert_eq!(vm.code, expected);
    }
}

