mod cpu;

pub struct Emulator {
    cpu: cpu::Cpu,
}

impl Emulator {
    pub fn new(bin: Vec<u8>) -> Self {
        Emulator {
            cpu: cpu::Cpu::new(bin),
        }
    }

    pub fn run(&mut self) {
        while self.cpu.pc < self.cpu.memory.len() {
            let inst = self.cpu.fetch();

            self.cpu.pc += 4;

            self.cpu.execute(inst);
        }

        self.cpu.print_regs();
    }
}
