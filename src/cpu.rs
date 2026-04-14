use crate::bus::Bus;
use crate::bus::DRAM_BASE_ADDR;
use crate::dram::DRAM_SIZE;

// RISC-V base opcode map
const OPCODE_OP: u8 = 0b0110011;
const OPCODE_OP_IMM: u8 = 0b0010011;
const OPCODE_LOAD: u8 = 0b0000011;
const OPCODE_STORE: u8 = 0b0100011;

// Instruction field bitmasks
const MASK_RD: u32 = 0x1F;
const MASK_RS1: u32 = 0x1F;
const MASK_RS2: u32 = 0x1F;
const MASK_FUNCT3: u32 = 0x07;
const MASK_FUNCT7: u32 = 0x7F;
const MASK_OPCODE: u32 = 0x7F;

#[allow(dead_code)]
enum Instruction {
    // R-type
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Xor { rd: usize, rs1: usize, rs2: usize },
    Or { rd: usize, rs1: usize, rs2: usize },
    And { rd: usize, rs1: usize, rs2: usize },

    // I-type (ALU)
    Addi { rd: usize, rs1: usize, imm: i64 },
    Xori { rd: usize, rs1: usize, imm: i64 },
    Ori { rd: usize, rs1: usize, imm: i64 },
    Andi { rd: usize, rs1: usize, imm: i64 },

    // I-type (Load)
    Lw { rd: usize, rs1: usize, imm: i64 },
    Lh { rd: usize, rs1: usize, imm: i64 },
    Lb { rd: usize, rs1: usize, imm: i64 },
    Lhu { rd: usize, rs1: usize, imm: i64 },
    Lbu { rd: usize, rs1: usize, imm: i64 },

    // S-type
    Sw { rs1: usize, rs2: usize, imm: i64 },
    Sh { rs1: usize, rs2: usize, imm: i64 },
    Sb { rs1: usize, rs2: usize, imm: i64 },
}

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub bus: Bus,
}

impl Cpu {
    pub fn new() -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_BASE_ADDR + DRAM_SIZE; // sp (x2)

        Cpu {
            regs,
            pc: DRAM_BASE_ADDR,
            bus: Bus::new(),
        }
    }

    pub fn load(&mut self, code: Vec<u8>) {
        self.bus.load(code);
    }

    pub fn run(&mut self) {
        loop {
            self.regs[0] = 0;
            let raw = self.fetch();

            if raw == 0 {
                println!("Hit zero instruction at pc = 0x{:X}, halting.", self.pc);
                break;
            }

            let instr = self.decode(raw);
            self.execute(instr);
        }
    }

    pub fn print_registers(&self) {
        println!("pc = 0x{:X}", self.pc);

        for i in 0..32 {
            if self.regs[i] != 0 {
                println!("x{:<2} = 0x{:X} ({})", i, self.regs[i], self.regs[i] as i64);
            }
        }
    }
}

impl Cpu {
    fn fetch(&self) -> u32 {
        return u32::from_le_bytes([
            self.bus.read(self.pc),
            self.bus.read(self.pc + 1),
            self.bus.read(self.pc + 2),
            self.bus.read(self.pc + 3),
        ]);
    }

    fn decode(&self, inst: u32) -> Instruction {
        let opcode = (inst & MASK_OPCODE) as u8;

        match opcode {
            OPCODE_OP => {
                let (rd, rs1, rs2, funct3, funct7) = r_type(inst);
                match (funct3, funct7) {
                    (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 },
                    (0x0, 0x20) => Instruction::Sub { rd, rs1, rs2 },
                    (0x4, 0x00) => Instruction::Xor { rd, rs1, rs2 },
                    (0x6, 0x00) => Instruction::Or { rd, rs1, rs2 },
                    (0x7, 0x00) => Instruction::And { rd, rs1, rs2 },
                    _ => panic!("Unknown OP funct3 = {:03b} funct7 = {:07b}", funct3, funct7),
                }
            }

            OPCODE_OP_IMM => {
                let (rd, rs1, imm, funct3) = i_type(inst);
                match funct3 {
                    0x0 => Instruction::Addi { rd, rs1, imm },
                    0x4 => Instruction::Xori { rd, rs1, imm },
                    0x6 => Instruction::Ori { rd, rs1, imm },
                    0x7 => Instruction::Andi { rd, rs1, imm },
                    _ => panic!("Unknown OP-IMM funct3 = {:03b}", funct3),
                }
            }

            OPCODE_LOAD => {
                let (rd, rs1, imm, funct3) = i_type(inst);
                match funct3 {
                    0x0 => Instruction::Lb { rd, rs1, imm },
                    0x1 => Instruction::Lh { rd, rs1, imm },
                    0x2 => Instruction::Lw { rd, rs1, imm },
                    0x4 => Instruction::Lbu { rd, rs1, imm },
                    0x5 => Instruction::Lhu { rd, rs1, imm },
                    _ => panic!("Unknown LOAD funct3 = {:03b}", funct3),
                }
            }

            OPCODE_STORE => {
                let (rs1, rs2, imm, funct3) = s_type(inst);
                match funct3 {
                    0x0 => Instruction::Sb { rs1, rs2, imm },
                    0x1 => Instruction::Sh { rs1, rs2, imm },
                    0x2 => Instruction::Sw { rs1, rs2, imm },
                    _ => panic!("Unknown STORE funct3 = {:03b}", funct3),
                }
            }

            _ => panic!("Unknown opcode = {:07b} at pc = 0x{:X}", opcode, self.pc),
        }
    }

    fn execute(&mut self, inst: Instruction) {
        match inst {
            // R-type
            Instruction::Add { rd, rs1, rs2 } => {
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                self.pc += 4;
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                self.pc += 4;
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
                self.pc += 4;
            }
            Instruction::Or { rd, rs1, rs2 } => {
                self.regs[rd] = self.regs[rs1] | self.regs[rs2];
                self.pc += 4;
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.regs[rd] = self.regs[rs1] & self.regs[rs2];
                self.pc += 4;
            }

            // I-type (ALU)
            Instruction::Addi { rd, rs1, imm } => {
                self.regs[rd] = self.regs[rs1].wrapping_add(imm as u64);
                self.pc += 4;
            }
            Instruction::Xori { rd, rs1, imm } => {
                self.regs[rd] = self.regs[rs1] ^ (imm as u64);
                self.pc += 4;
            }
            Instruction::Ori { rd, rs1, imm } => {
                self.regs[rd] = self.regs[rs1] | (imm as u64);
                self.pc += 4;
            }
            Instruction::Andi { rd, rs1, imm } => {
                self.regs[rd] = self.regs[rs1] & (imm as u64);
                self.pc += 4;
            }
            _ => unimplemented!("This instruction is unimplemented!"),
        }
    }
}

fn r_type(inst: u32) -> (usize, usize, usize, u32, u32) {
    let rd = ((inst >> 7) & MASK_RD) as usize;
    let rs1 = ((inst >> 15) & MASK_RS1) as usize;
    let rs2 = ((inst >> 20) & MASK_RS2) as usize;
    let funct3 = (inst >> 12) & MASK_FUNCT3;
    let funct7 = (inst >> 25) & MASK_FUNCT7;
    return (rd, rs1, rs2, funct3, funct7);
}

fn i_type(inst: u32) -> (usize, usize, i64, u32) {
    let rd = ((inst >> 7) & MASK_RD) as usize;
    let rs1 = ((inst >> 15) & MASK_RS1) as usize;
    let funct3 = (inst >> 12) & MASK_FUNCT3;
    let imm = ((inst as i32) >> 20) as i64;
    return (rd, rs1, imm, funct3);
}

fn s_type(inst: u32) -> (usize, usize, i64, u32) {
    let rs1 = ((inst >> 15) & MASK_RS1) as usize;
    let rs2 = ((inst >> 20) & MASK_RS2) as usize;
    let funct3 = (inst >> 12) & MASK_FUNCT3;
    let imm = ((((inst & 0xFE000000) as i32) >> 20) | ((inst >> 7) & 0x1F) as i32) as i64;
    return (rs1, rs2, imm, funct3);
}
