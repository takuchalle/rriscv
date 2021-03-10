use crate::bus;

pub struct Cpu {
    regs: [u64; 32],
    // Program Counter
    pub pc: usize,
    pub bus: bus::Bus,
}

impl Cpu {
    pub fn new(bin: Vec<u8>) -> Self {
        Cpu {
            regs: [0; 32],
            pc: 0,
            bus: bus::Bus::new(bin),
        }
    }

    pub fn fetch(&self) -> u32 {
        match self.bus.load(self.pc as u64, 32) {
            Ok(inst) => inst as u32,
            Err(_e) => unimplemented!(),
        }
    }

    pub fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _ => {
                dbg!("not implemented yet");
            }
        };
        // x0 register is hardwired 0
        self.regs[0] = 0;
    }

    pub fn print_regs(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x}, x{:02}({})={:>#18x}, x{:02}({})={:>#18x}, x{:02}({})={:>#18x},",
                    i,
                    abi[i],
                    self.regs[i],
                    i + 1,
                    abi[i + 1],
                    self.regs[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.regs[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }
}
