use crate::dram::Dram;

// Base Address of RAM
pub const DRAM_BASE_ADDR: u64 = 0x8000_0000;

pub struct Bus {
    ram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Bus { ram: Dram::new() }
    }

    pub fn load(&mut self, code: Vec<u8>) {
        self.ram.load(code);
    }

    pub fn read(&self, addr: u64) -> u8 {
        let mem_addr = self.translate(addr);
        self.ram.read(mem_addr)
    }

    #[allow(dead_code)]
    pub fn write(&mut self, addr: u64, val: u8) {
        let mem_addr = self.translate(addr);
        self.ram.write(mem_addr, val);
    }

    fn translate(&self, addr: u64) -> u64 {
        assert!(
            addr >= DRAM_BASE_ADDR,
            "Bus error: invalid address 0x{:X}",
            addr
        );
        return addr - DRAM_BASE_ADDR;
    }
}
