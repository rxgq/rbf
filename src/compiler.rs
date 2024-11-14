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
        Compiler {
            options,
            ast
        }
    }

    pub fn compile(&self) -> std::io::Result<()> {
        let mut output = String::new();
        
        // Start with `.data` section
        output.push_str("section .data\n");
        output.push_str("    hello: db 'Hello, World!', 10\n"); // Sample message
        output.push_str("    helloLen: equ $-hello\n");

        // `.text` section and start label
        output.push_str("\nsection .text\n");
        output.push_str("    global _start\n");
        output.push_str("_start:\n");
        
        // Initialize a pointer (rbx register) to memory for Brainfuck tape
        output.push_str("    mov rbx, tape\n"); // `tape` would be defined later

        for node in &self.ast.body {
            self.compile_node(node, &mut output);
        }

        // Exit program
        output.push_str("    mov eax, 1       ; sys_exit\n");
        output.push_str("    mov ebx, 0       ; Exit code 0\n");
        output.push_str("    int 0x80         ; Call kernel\n");

        // Write to file
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
                output.push_str("    mov eax, 4       ; sys_write\n");
                output.push_str("    mov ebx, 1       ; stdout\n");
                output.push_str("    mov ecx, rbx     ; pointer to cell\n");
                output.push_str("    mov edx, 1       ; write 1 byte\n");
                output.push_str("    int 0x80         ; Call kernel\n");
            },
            ASTNode::InputNode => {
                output.push_str("    mov eax, 3       ; sys_read\n");
                output.push_str("    mov ebx, 0       ; stdin\n");
                output.push_str("    mov ecx, rbx     ; pointer to cell\n");
                output.push_str("    mov edx, 1       ; read 1 byte\n");
                output.push_str("    int 0x80         ; Call kernel\n");
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
        // Generate a unique label based on a base name
        static mut LABEL_COUNTER: usize = 0;
        unsafe {
            LABEL_COUNTER += 1;
            format!("{}_{}", base, LABEL_COUNTER)
        }
    }
}
