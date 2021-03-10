pub const DRAM_SIZE: u64 = 128 * 1024 * 1024;
// pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_BASE: u64 = 0x0000_0000;

pub(crate) struct Dram {
    memory: Vec<u8>,
}

impl Dram {
    pub(crate) fn new(bin: Vec<u8>) -> Self {
        let mut mem = vec![0; DRAM_SIZE as usize];
        mem.splice(..bin.len(), bin.iter().cloned());
        Dram { memory: mem }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(()),
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, data: u64) -> Result<(), ()> {
        match size {
            8 => Ok(self.store8(addr, data)),
            16 => Ok(self.store16(addr, data)),
            32 => Ok(self.store32(addr, data)),
            64 => Ok(self.store64(addr, data)),
            _ => Err(()),
        }
    }

    fn load8(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return self.memory[index] as u64;
    }

    fn load16(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.memory[index] as u64) | (self.memory[index + 1] as u64) << 8;
    }

    fn load32(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.memory[index] as u64)
            | (self.memory[index + 1] as u64) << 8
            | (self.memory[index + 2] as u64) << 16
            | (self.memory[index + 3] as u64) << 24;
    }

    fn load64(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.memory[index] as u64)
            | (self.memory[index + 1] as u64) << 8
            | (self.memory[index + 2] as u64) << 16
            | (self.memory[index + 3] as u64) << 24
            | (self.memory[index + 4] as u64) << 32
            | (self.memory[index + 5] as u64) << 40
            | (self.memory[index + 6] as u64) << 48
            | (self.memory[index + 7] as u64) << 56;
    }

    fn store8(&mut self, addr: u64, data: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.memory[index] = data as u8;
    }

    fn store16(&mut self, addr: u64, data: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.memory[index] = (data & 0x00ff) as u8;
        self.memory[index + 1] = ((data & 0xff00) >> 8) as u8;
    }

    fn store32(&mut self, addr: u64, data: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.memory[index] = (data & 0x000000ff) as u8;
        self.memory[index + 1] = ((data & 0x0000ff00) >> 8) as u8;
        self.memory[index + 2] = ((data & 0x00ff0000) >> 16) as u8;
        self.memory[index + 3] = ((data & 0xff000000) >> 24) as u8;
    }

    fn store64(&mut self, addr: u64, data: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.memory[index] = (data & 0x00000000000000ff) as u8;
        self.memory[index + 1] = ((data & 0x000000000000ff00) >> 8) as u8;
        self.memory[index + 2] = ((data & 0x0000000000ff0000) >> 16) as u8;
        self.memory[index + 3] = ((data & 0x00000000ff000000) >> 24) as u8;
        self.memory[index + 4] = ((data & 0x000000ff00000000) >> 32) as u8;
        self.memory[index + 5] = ((data & 0x0000ff0000000000) >> 40) as u8;
        self.memory[index + 6] = ((data & 0x00ff000000000000) >> 48) as u8;
        self.memory[index + 7] = ((data & 0xff00000000000000) >> 56) as u8;
    }
}
