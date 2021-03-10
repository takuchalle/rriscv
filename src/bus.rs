use crate::dram;
use crate::dram::{DRAM_BASE, DRAM_SIZE};

pub struct Bus {
    dram: dram::Dram,
}

impl Bus {
    pub fn new(bin: Vec<u8>) -> Self {
        Bus {
            dram: dram::Dram::new(bin),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            self.dram.load(addr, size)
        } else {
            Err(())
        }
    }
}
