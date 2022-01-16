// Direct interpreter
mod opcode;

use opcode::{Opcode, Code};
use std::io::Write;
use std::io::Read;

struct Interpreter{
    stack: Vec<u8>
}

impl Interpreter {
    fn new() -> Self {
        Self {
            stack: vec![0; 1]
        }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = opcode::Code::from(data)?;
        let code_len = code.instructions.len();
        let mut program_counter = 0;
        let mut stack_pointer = 0;
        loop {
            if program_counter >= code_len {
                break;
            }
            match code.instructions[program_counter] {
                Opcode::SHL => {
                    if stack_pointer != 0 {
                        stack_pointer -= 1
                    }
                },
                Opcode::SHR => {
                    stack_pointer += 1;
                    if stack_pointer == self.stack.len() {
                        self.stack.push(0)
                    }
                }
                Opcode::ADD => {
                    self.stack[stack_pointer] = self.stack[stack_pointer].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[stack_pointer] = self.stack[stack_pointer].overflowing_sub(1).0;
                }
                Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[stack_pointer]])?;
                }
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[stack_pointer] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[stack_pointer] == 0x00 {
                        program_counter = code.jump_table[&program_counter];
                    }
                }
                Opcode::RB => {
                    if self.stack[stack_pointer] != 0x00 {
                        program_counter = code.jump_table[&program_counter];
                    }
                }
            }
            program_counter += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;
//    let code = Code::from(data)?;
//    println!("{:?}", code.instructions);
    let mut intepretor = Interpreter::new();
    intepretor.run(data);

    Ok(())
}
