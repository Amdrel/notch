#[derive(Default)]
pub struct Cpu {
    // Program counter.
    pub pc: u16,

    // The stack.
    stack: [u16; 12],

    // Stack pointer.
    sp: u8,

    // General purpose registers v0-vf.
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,

    // Address register "I".
    i: u16,

    // Timer and sound registers.
    dt: u8,
    st: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }

    #[inline(always)]
    pub fn execute_instruction(&mut self, instr: u16) {
        let opcode = (instr >> 12) as u8;

        match opcode {
            0x6 => {
                let reg = ((instr << 4) >> 12) as u8;
                let byte = ((instr << 8) >> 8) as u8;

                println!("regval: {:#x}", self.va);
                self.set_reg(reg, byte);

                //println!("reg: {:#x}", reg);
                //println!("byte: {:#x}", byte);
            },
            _ => {
                panic!("Found unknown opcode at addr: {:#x}", instr);
            }
        }

        //println!("opcode: {:#x}", opcode);
        //println!("addr: {:#x}", instr);
    }

    fn set_reg(&mut self, reg: u8, byte: u8) {
        match reg {
            0x0 => self.v0 = byte,
            0x1 => self.v1 = byte,
            0x2 => self.v2 = byte,
            0x3 => self.v3 = byte,
            0x4 => self.v4 = byte,
            0x5 => self.v5 = byte,
            0x6 => self.v6 = byte,
            0x7 => self.v7 = byte,
            0x8 => self.v8 = byte,
            0x9 => self.v9 = byte,
            0xa => self.va = byte,
            0xb => self.vb = byte,
            0xc => self.vc = byte,
            0xd => self.vd = byte,
            0xe => self.ve = byte,
            0xf => self.vf = byte,
            _ => {
                panic!("Cannot set unknown register: V{:X}", reg);
            }
        }
    }
}
