.data
num1: .word 5
num2: .word 3
result: .word 0
.text
.globl main
main:
    lw $t0, num1          # Load num1 into $t0
    lw $t1, num2          # Load num2 into $t1
    add $t2, $t0, $t1     # Add $t0 and $t1, store result in $t2
    sw $t2, result        # Store the result in memory
    li $v0, 10            # Exit system call
    syscall
