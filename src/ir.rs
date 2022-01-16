use super::opcode;

#[derive(Debug, PartialEq)]

pub enum IR {
    SHL(u32),  // >>>> equals to SHR(4)
    SHR(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32), //jump if zero
    JNZ(u32)  //jump if not zero
}

pub struct Code {
    pub instructions: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<opcode::Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instructions: Vec<IR> = Vec::new();
        let mut jump_stack: Vec<u32> = Vec::new();
        for e in data {
            match e {
                opcode::Opcode::SHL => {
                    match instructions.last_mut() {
                        Some(IR::SHL(x)) => {
                            *x += 1
                        },
                        _ => {
                            instructions.push(IR::SHL(1))
                        }
                    }
                },
                opcode::Opcode::SHR => {
                    match instructions.last_mut() {
                        Some(IR::SHR(x)) => {
                            *x += 1
                        },
                        _ => {
                            instructions.push(IR::SHR(1))
                        }
                    }
                }
                opcode::Opcode::ADD => {
                    match instructions.last_mut() {
                        Some(IR::ADD(x)) => {
                            let (b, _) = x.overflowing_add(1);
                            *x = b;
                        },
                        _ => {
                            instructions.push(IR::ADD(1))
                        }
                    }
                }
                opcode::Opcode::SUB => {
                    match instructions.last_mut() {
                        Some(IR::SUB(x)) => {
                            let (b, _) = x.overflowing_add(1);
                            *x = b;
                        },
                        _ => {
                            instructions.push(IR::SUB(1))
                        }
                    }
                }
                opcode::Opcode::PUTCHAR => {
                    instructions.push(IR::PUTCHAR);
                }
                opcode::Opcode::GETCHAR => {
                    instructions.push(IR::GETCHAR);
                }
                opcode::Opcode::LB => {
                    instructions.push(IR::JIZ(0));
                    jump_stack.push((instructions.len() - 1) as u32);
                }
                opcode::Opcode::RB => {
                    let j = jump_stack.pop().ok_or("pop from empty list")?;
                    instructions.push(IR::JNZ(j));
                    let instructions_len = instructions.len();
                    match &mut instructions[j as usize] {
                        IR::JIZ(x) => {
                            *x = (instructions_len - 1) as u32;
                        },
                        _ => unreachable!()
                    }
                }
            }
        }

        Ok(Code {instructions})
    }

}