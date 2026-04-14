.global _start
_start:
    addi t0, x0, 0b1100
    addi t1, x0, 0b1010
    addi t5, x0, 0b1111

    xor t2, t0, t1
    or t3, t0, t1
    and t4, t0, t1

    xori t1, t5, 0b0011
    ori t2, t5, 0b0011
    andi t3, t5, 0b0011
