use std::str::FromStr;
use std::num::ParseIntError;

type Register = usize;
type Address = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    NOP,
    HLT,
    JMP(Register),
    PRN(Register),
    LDI(Register, i32),
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

// impl VM {
//     fn decode(&mut self) -> Instruction {
//         match self.instructions[self.ip] {
//             0 => Instruction::NOP,
//             1 => Instruction::HLT,
//             3 => {
//                 self.ip += 1;
//                 let reg = self.instructions[self.ip] as Register;
//                 Instruction::JMP(reg)
//             },
//             71 => {
                // PRN instruction 
//                 self.ip += 1;
//                 let reg = self.instructions[self.ip] as Register;
//                 Instruction::PRN(reg)
//             },
//             130 => {
                // LDI instruction 
//                 self.ip += 1;
//                 let reg = self.instructions[self.ip] as Register;
//                 self.ip += 1;
//                 let arg = self.instruction[self.ip];
//                 Instruction::LDI(reg, arg)
//             }
//         }
//     }

//     pub fn run(&mut self) {
        // vm is already running 
//         if !self.terminated { return; }

//         self.terminated = false;

//         while self.ip < self.instructions.len() && !self.terminated {
            // decode the next instruction and its arguments
//             let inst = self.decode();
//             let result = self.execute();

//             if !result {
//                 panic!("Failed to execute instruction at address: {}", self.ip);
//             }

//             self.ip += 1;
//         }
//     } 
// }

impl FromStr for VM {
    type Err = InstructionError;

    fn from_str(program: &str) -> Result<Self, Self::Err> {
        let code = program.lines()
            .map(|inst| {
                let inst = inst.trim();
                u8::from_str_radix(inst, 2)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(VM {
            ip: 0,
            terminated: false,
            code,
            registers: [0; 8],
        })
    }
}

#[cfg(test)]
#[test]
fn test_parse_instructions() {
    let input = "10000010
00000000
00001000
01000111
00000000
00000001";
    
    let expected = vec![130, 0, 8, 71, 0, 1];
    let vm = VM::from_str(input).expect("Failed to parse input in `test_parse_instructions`");

    assert_eq!(vm.code, expected);
}
