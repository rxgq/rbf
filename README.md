# Brainfuck Compiler

Useless Brainfuck to x86_64 Assembly compiler.

Run the compiler with: 
```
cargo run <path_to_source_file> <mode> <output_path>
```

For the `mode`, pass `-d` for debug to display compiler data or `-n` for minimal output.


### Features

- [x] Compilation
- [x] Comments
- [x] Compiler warnings
- [ ] Code optimisation
- [ ] Multi-platform support (windows assembly)

<br/>


### Example

```
++++++++++ 

[
    >+++++++>++++++++++>+++>+<<<<-
]

>++.                # H
>+.                 # E
+++++++.            # L
.                   # L
+++.                # O
>++.                # 
<<+++++++++++++++.  # W
>.                  # O
+++.                # R
------.             # L
--------.           # D
>+.                 # !
```

Compiles to:

```
section .bss
    tape: resb 30000

section .text
    global _start
_start:
    mov rbx, tape
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
loop_start_1:
    cmp byte [rbx], 0
    je loop_end_2
    inc rbx
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc rbx
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc rbx
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc rbx
    inc byte [rbx]
    dec rbx
    dec rbx
    dec rbx
    dec rbx
    dec byte [rbx]
    jmp loop_start_1
loop_end_2:
    inc rbx
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    inc rbx
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    inc rbx
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    dec rbx
    dec rbx
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    inc rbx
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    inc rbx
    inc byte [rbx]
    mov rax, 1           ; sys_write
    mov rdi, 1           ; file descriptor: stdout
    mov rsi, rbx         ; pointer to the current cell
    mov rdx, 1           ; number of bytes to write
    syscall              ; Call kernel
    mov rax, 60          ; sys_exit
    mov rdi, 0           ; Exit code 0
    syscall              ; Call kernel

```