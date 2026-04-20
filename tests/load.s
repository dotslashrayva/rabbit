.global _start
_start:
    addi t0, sp, -256

    addi t1, x0, -85
    sb t1, 0(t0)

    addi t1, x0, -1
    sh t1, 4(t0)

    addi t1, x0, 0x7FF
    sw t1, 8(t0)

    lb t2, 0(t0)
    lh t3, 4(t0)
    lw t4, 8(t0)

    lbu t5, 0(t0)
    lhu t6, 4(t0)

    ebreak
