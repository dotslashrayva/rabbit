mod bus;
mod cpu;
mod dram;

use std::env;
use std::fs;

use crate::cpu::Cpu;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: rabbit <filename>");
        return;
    }

    let code = fs::read(&args[0]).unwrap();
    let num_inst = code.len() / 4;

    let mut cpu = Cpu::new();
    cpu.load(code);

    println!("Rabbit: RISC-V (RV64I) Emulator");
    println!("Loaded {} instruction(s)\n", num_inst);

    cpu.run();
    cpu.print_registers();

    println!("\nExecution Completed!");
}
