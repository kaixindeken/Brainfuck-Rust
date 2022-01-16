// Optimized interpreter with JIT
mod ir;
mod opcode;

use dynasm::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};
use std::io::Write;

unsafe extern "sysv64" fn putchar(char: u8) {
    std::io::stdout().write_all(&[char]).unwrap()
}

struct Interpreter {}

impl Interpreter {
    fn new() -> Self {
        Self {}
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let opcode_code = opcode::Code::from(data)?;
        let code = ir::Code::from(opcode_code.instructions)?;
        //jump for "[" and "]"
        let mut loops = vec![];

        let mut ops = dynasmrt::x64::Assembler::new()?;
        let entry_point = ops.offset();

        //rdi: first param call by linux
        dynasm!(ops
            ; .arch x64
            ; mov rcx, rdi
        );

        for ir in code.instructions {
            match ir {
                ir::IR::SHL(x) => dynasm!(ops
                    ; sub rcx, x as i32
                ),
                ir::IR::SHR(x) => dynasm!(ops
                    ; add rcx, x as i32
                ),
                ir::IR::ADD(x) => dynasm!(ops
                    ; add BYTE [rcx], x as i8
                ),
                ir::IR::SUB(x) => dynasm!(ops
                    ; sub BYTE [rcx], x as i8
                ),
                ir::IR::PUTCHAR => dynasm!(ops
                    ; mov r12, rcx
                    ; mov rdi, [rcx]
                    ; mov rax, QWORD putchar as _
                    ; call rax
                    ; mov rcx, r12
                ),
                //考虑到输入获取较少，所以留空
                ir::IR::GETCHAR => {}
                ir::IR::JIZ(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    loops.push((l, r));
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jz => r
                        ; => l
                    )
                }
                ir::IR::JNZ(_) => {
                    let (l, r) = loops.pop().unwrap();
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jnz => l
                        ; => r
                    )
                }
            }
        }
        dynasm!(ops
            ; ret
        );

        //writeable and executeable memory
        let exec_buffer = ops.finalize().unwrap();
        //location of bf code in memory
        let mut memory: Box<[u8]> = vec![0; 65536].into_boxed_slice();
        let memory_addr_from = memory.as_mut_ptr();
        let memory_addr_to = unsafe { memory_addr_from.add(memory.len()) };
        let fun: fn(memory_addr_from: *mut u8, memory_addr_to: *mut u8) =
            unsafe { std::mem::transmute(exec_buffer.ptr(entry_point)) };
        fun(memory_addr_from, memory_addr_to);
        Ok(())
    }

}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;
    let mut intepretor = Interpreter::new();
    intepretor.run(data);

    Ok(())
}