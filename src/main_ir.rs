// Optimized interpreter
mod ir;
mod opcode;

use ir::{IR, Code};
use std::io::Write;
use std::io::Read;

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            stack: vec![0; 1]
        }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let opcode_code = opcode::Code::from(data)?;
        let code = Code::from(opcode_code.instructions)?;
        let code_len = code.instructions.len();
        let mut program_counter = 0;
        let mut stack_pointer = 0;

        loop {
            if program_counter >= code_len {
                break;
            }
            match code.instructions[program_counter] {
                IR::SHL(x) => {
                    for _ in 0..x {
                        if stack_pointer != 0 {
                            stack_pointer -= 1;
                        } else {
                            break;
                        }
                    }
                },
                IR::SHR(x) => {
                    stack_pointer += x as usize;
                    if stack_pointer >= self.stack.len() {
                        let expand = stack_pointer - self.stack.len() + 1;
                        for _ in 0..expand {
                            self.stack.push(0);
                        }
                    }
                },
                IR::ADD(x) => {
                    self.stack[stack_pointer] = self.stack[stack_pointer].overflowing_add(x).0;
                },
                IR::SUB(x) => {
                    self.stack[stack_pointer] = self.stack[stack_pointer].overflowing_sub(x).0;
                },
                IR::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[stack_pointer]])?;
                },
                IR::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[stack_pointer] = buf[0];
                },
                IR::JIZ(x) => {
                    if self.stack[stack_pointer] == 0x00 {
                        program_counter = x as usize;
                    }
                },
                IR::JNZ(x) => {
                    if self.stack[stack_pointer] != 0x00 {
                        program_counter = x as usize;
                    }
                }
            }
            program_counter += 1;
        }
        Ok(())
    }
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;
//    let opcode_code = opcode::Code::from(data)?;
//    let code = Code::from(opcode_code.instructions)?;
//    println!("{:?}", code.instructions);
    let mut intepretor = Interpreter::new();
    intepretor.run(data);

    Ok(())
}