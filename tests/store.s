.global _start
_start:
    addi t0, sp, -256

    addi t1, x0, -85
    sb t1, 0(t0)

    addi t2, x0, 0x1FF
    sh t2, 4(t0)

    addi t3, x0, 0x7FF
    sw t3, 8(t0)

    ebreak
