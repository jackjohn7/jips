.data
msg: .asciiz "Hello, MIPS!\n"
.text
.globl main
main:
    li $v0, 4          # System call for printing a string
    la $a0, msg        # Load the address of the string into $a0
    syscall
    li $v0, 10         # Exit system call
    syscall
