mod bus;
mod cpu;
mod dram;

use cpu::Cpu;

pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    pub fn new(bin: Vec<u8>) -> Self {
        Emulator { cpu: Cpu::new(bin) }
    }

    pub fn run(&mut self) {
        while let Ok(inst) = self.cpu.fetch() {
            self.cpu.pc += 4;

            self.cpu.execute(inst);
        }
    }
}
