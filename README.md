# Rabbit

A RISC-V (RV64I) emulator written from scratch in Rust.

No frameworks, no dependencies, just a fetch-decode-execute loop over raw machine code. Rabbit emulates a 64-bit RISC-V CPU with 32 registers, a typed bus layer with address translation, and 12 MB of flat DRAM starting at `0x8000_0000`.

## Supported Instructions

| Type | Instructions |
|------|-------------|
| R-type | `ADD` `SUB` `XOR` `OR` `AND` |
| I-type (ALU) | `ADDI` `XORI` `ORI` `ANDI` |
| I-type (Load) | `LB` `LH` `LW` `LBU` `LHU` *(decode only)* |
| S-type (Store) | `SB` `SH` `SW` *(decode only)* |

## Quick Start

Rabbit takes flat binaries, not ELF. Assemble with the RISC-V toolchain:

```bash
riscv64-unknown-elf-gcc -march=rv64i -mabi=lp64 -Wl,-Ttext=0x0 -nostdlib -o prog.o prog.s
riscv64-unknown-elf-objcopy -O binary prog.o prog.bin
cargo run -- prog.bin
```

A Makefile is included in `tests/` — drop `.s` files there and run `make`.

## Example

```asm
.global _start
_start:
    addi x28, x0, 87
    addi x29, x0, 18
    add  x30, x29, x28
    sub  x31, x28, x29
```

```
Rabbit: RISC-V (RV64I) Emulator
Loaded 4 instruction(s)

Hit zero instruction at pc = 0x80000010, halting.
pc = 0x80000010
x28 = 0x57 (87)
x29 = 0x12 (18)
x30 = 0x69 (105)
x31 = 0x45 (69)

Execution Completed!
```

## Roadmap

- [ ] Load/store execution
- [ ] Multi-byte bus operations
- [ ] Branches (B-type)
- [ ] `LUI`, `AUIPC`, `JAL`, `JALR`
- [ ] Shifts and comparisons
- [ ] RV64I word ops (`ADDW`, `SUBW`, etc.)
- [ ] `ECALL` / `EBREAK`
- [ ] UART

## License

MIT
