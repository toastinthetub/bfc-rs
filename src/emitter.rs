use crate::ast::Instruction;

static mut LABEL_ID: usize = 0;

pub fn emit(instructions: &[Instruction]) -> String {
    let mut out = String::new();
    out.push_str("global _start\nsection .text\n\n_start:\n    mov r15, tape\n");
    emit_instructions(instructions, &mut out);
    out.push_str("\n    mov rax, 60 ; syscall: exit\n    xor rdi, rdi ; status 0\n    syscall\n\nsection .bss\ntape: resb 30000\n");
    out
}

fn emit_instructions(instructions: &[Instruction], out: &mut String) {
    for instr in instructions {
        match instr {
            Instruction::MovePtr(n) => {
                if *n != 0 {
                    out.push_str(&format!("    add r15, {}\n", n));
                }
            }
            Instruction::AddData(n) => {
                if *n != 0 {
                    out.push_str(&format!("    add byte [r15], {}\n", n));
                }
            }
            Instruction::Output => {
                out.push_str(
                    "    mov rax, 1\n    mov rdi, 1\n    mov rsi, r15\n    mov rdx, 1\n    syscall\n"
                );
            }
            Instruction::Input => {
                out.push_str(
                    "    mov rax, 0\n    mov rdi, 0\n    mov rsi, r15\n    mov rdx, 1\n    syscall\n"
                );
            }
            Instruction::Loop(body) => {
                let id = unsafe {
                    let id = LABEL_ID;
                    LABEL_ID += 1;
                    id
                };
                out.push_str(&format!("L{}_start:\n", id));
                out.push_str("    cmp byte [r15], 0\n    je L");
                out.push_str(&format!("{}_end\n", id));
                emit_instructions(body, out);
                out.push_str(&format!(
                    "    cmp byte [r15], 0\n    jne L{}_start\nL{}_end:\n",
                    id, id
                ));
            }
            Instruction::Nop => {}
        }
    }
}
