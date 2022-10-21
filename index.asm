format ELF64 executable
entry start
segment readable executable
start:
mov rsi, mem
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; right
inc rsi
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; opb
b0:
mov al, byte [rsi]
test al, al
jz e0
; sub
dec byte [rsi]
; left
dec rsi
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; add
inc byte [rsi]
; right
inc rsi
; endb
mov al, byte [rsi]
test al, al
jnz b0
e0:
; left
dec rsi
; write
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
mov rax, 60
mov rdi, 0
syscall
segment readable writable
mem: times 30000 db 0