// 12 MB of RAM
pub const DRAM_SIZE: u64 = 1024 * 1024 * 12;

pub struct Dram {
    data: Vec<u8>,
}

impl Dram {
    pub fn new() -> Self {
        Dram {
            data: vec![0; DRAM_SIZE as usize],
        }
    }

    pub fn read(&self, addr: u64) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u64, val: u8) {
        self.data[addr as usize] = val;
    }

    pub fn load(&mut self, code: Vec<u8>) {
        self.data[..code.len()].copy_from_slice(&code);
    }
}
