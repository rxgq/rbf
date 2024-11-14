use std::{fs::File, io::Write};
use crate::ast_node::{ASTNode, AST};

pub struct CompilerOptions {
    pub output_path: String
}

pub struct Compiler {
    options: CompilerOptions,
    ast: AST
}

impl Compiler {
    pub fn new(options: CompilerOptions, ast: AST) -> Compiler {
        Compiler { options, ast }
    }

    pub fn compile(&self) -> std::io::Result<()> {
        let mut output = String::new();

        // reserve 30,000 bytes as uninitialized data
        output.push_str("section .bss\n");
        output.push_str("    tape: resb 30000\n");

        // essentially the 'main' entry point
        output.push_str("\nsection .text\n");
        output.push_str("    global _start\n");
        output.push_str("_start:\n");
        
        // point the rbx register to the tape 
        // rbx is used to reference the current tape position
        output.push_str("    mov rbx, tape\n");

        // compile each node
        for node in &self.ast.body {
            self.compile_node(node, &mut output);
        }

        // load the exit syscall number (60) into the rax register
        // load 0 as an argument to indicate success
        // invoke the syscall
        output.push_str("    mov rax, 60          ; sys_exit\n");
        output.push_str("    mov rdi, 0           ; Exit code 0\n");
        output.push_str("    syscall              ; Call kernel\n");

        let mut file = File::create(&self.options.output_path)?;
        file.write_all(output.as_bytes())?;

        Ok(())
    }

    fn compile_node(&self, node: &ASTNode, output: &mut String) {
        match node {
            ASTNode::IncPtrNode => output.push_str("    inc rbx\n"),
            ASTNode::DecPtrNode => output.push_str("    dec rbx\n"),
            ASTNode::IncValNode => output.push_str("    inc byte [rbx]\n"),
            ASTNode::DecValNode => output.push_str("    dec byte [rbx]\n"),
            ASTNode::OutputNode => {
                output.push_str("    mov rax, 1           ; sys_write\n");
                output.push_str("    mov rdi, 1           ; file descriptor: stdout\n");
                output.push_str("    mov rsi, rbx         ; pointer to the current cell\n");
                output.push_str("    mov rdx, 1           ; number of bytes to write\n");
                output.push_str("    syscall              ; Call kernel\n");
            },
            ASTNode::InputNode => {
                output.push_str("    mov rax, 0           ; sys_read\n");
                output.push_str("    mov rdi, 0           ; file descriptor: stdin\n");
                output.push_str("    mov rsi, rbx         ; pointer to the current cell\n");
                output.push_str("    mov rdx, 1           ; number of bytes to read\n");
                output.push_str("    syscall              ; Call kernel\n");
            },
            ASTNode::LoopNode(body) => {
                let loop_start = self.unique_label("loop_start");
                let loop_end = self.unique_label("loop_end");

                output.push_str(&format!("{}:\n", loop_start));
                output.push_str("    cmp byte [rbx], 0\n");
                output.push_str(&format!("    je {}\n", loop_end));

                for inner_node in body {
                    self.compile_node(inner_node, output);
                }

                output.push_str(&format!("    jmp {}\n", loop_start));
                output.push_str(&format!("{}:\n", loop_end));
            }
        }
    }

    fn unique_label(&self, base: &str) -> String {
        static mut LABEL_COUNTER: usize = 0;
        unsafe {
            LABEL_COUNTER += 1;
            format!("{}_{}", base, LABEL_COUNTER)
        }
    }
}