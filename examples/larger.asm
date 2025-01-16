.data
num1: .word 12
num2: .word 20
larger: .word 0
.text
.globl main
main:
    lw $t0, num1          # Load num1 into $t0
    lw $t1, num2          # Load num2 into $t1
    bge $t0, $t1, num1_larger  # If $t0 >= $t1, jump to num1_larger
    sw $t1, larger             # Otherwise, num2 is larger
    j end                      # Jump to end
num1_larger:
    sw $t0, larger             # Store num1 as the larger number
end:
    li $v0, 10                 # Exit system call
    syscall
