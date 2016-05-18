use std::thread::sleep;
use std::time::Duration;
use super::memory::END_RESERVED;
use super::rand::random;
use super::interconnect::Interconnect;

// Instructions are 2 bytes long and stored as BigEndian.
const INSTRUCTION_SIZE: u16 = 2;

// Round about 60Hz delay for timers.
const TIMER_DELAY: u64 = 16;

// Around 500Hz clock speed.
const EXECUTION_DELAY: u64 = 2;

#[derive(Debug)]
pub struct Cpu {
    interconnect: Interconnect,

    // Program counter.
    pc: u16,

    // The function call stack.
    stack: [u16; 16],

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
    pub fn new(interconnect: Interconnect) -> Cpu {
        Cpu {
            // Interconnect is used to control system resources like rom and memory.
            interconnect: interconnect,

            // Program counter.
            pc: END_RESERVED as u16,

            // The function call stack.
            stack: [0; 16],

            // Stack pointer.
            sp: 0,

            // General purpose registers v0-vf.
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,

            // Address register "I".
            i: 0,

            // Timer and sound registers.
            dt: 0,
            st: 0,
        }
    }

    /// Execute instructions from ram.
    pub fn run(&mut self) {
        loop {
            // Interconnect can signal the emulator to halt.
            // This is because interconnect works with the native window system
            // and handles close events.
            if self.interconnect.halt {
                break
            }

            // Read a word from ram where the program counter currently points
            // to execute.
            let word = self.interconnect.memory.read_word(self.pc);

            // Execute until the subroutine ends if we are in one.
            if self.execute_instruction(word) {
                break
            }

            // Poll for input and set the input state.
            self.interconnect.handle_input();
        }
    }

    #[inline(always)]
    fn execute_instruction(&mut self, instr: u16) -> bool {
        self.handle_timers();

        let opcode = (instr >> 12) as u8;
        let mut ret: bool = false;
        let mut skip: bool = false;

        match opcode {
            0x0 => {
                let identifier = ((instr << 8) >> 8) as u8;

                match identifier {
                    0xE0 => {
                        // 00E0 - CLS
                        // Clears the screen.

                        self.interconnect.clear_display();
                    },
                    0xEE => {
                        // 00EE - RET
                        // Returns from a subroutine.

                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        self.stack[self.sp as usize] = 0;

                        // Break out of the execution loop for the current
                        // subroutine.
                        ret = true;
                    },
                    _ => {
                        // 0NNN - SYS NNN
                        //
                        // Jump to a machine code routine at NNN. This operation
                        // is not implemented on purpose.

                        panic!("Unhandled, 0NNN is not implemented in most \
                               modern interpreters and is not used by many \
                               roms.");
                    },
                }
            },
            0x1 => {
                // 1NNN - JP NNN
                //
                // Jumps to address NNN.

                let addr = ((instr << 4) >> 4) as u16;
                self.pc = addr;
                skip = true;
            },
            0x2 => {
                // 2NNN - CALL NNN
                //
                // Calls subroutine at NNN.

                let addr = ((instr << 4) >> 4) as u16;

                // Add the current program counter to the call stack.
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;

                // Set the program counter to the call address begin executing
                // the subroutine.
                self.pc = addr;
                self.run();
            },
            0x3 => {
                // 3XNN - SE VX, NN
                //
                // Skips the next instruction if VX equals NN.

                let regx = ((instr << 4) >> 12) as u8;
                let byte = ((instr << 8) >> 8) as u8;
                let x = self.get_reg(regx);

                if x == byte {
                    self.pc += INSTRUCTION_SIZE;
                }
            },
            0x4 => {
                // 4XNN - SNE VX, NN
                //
                // The interpreter compares register VX to NN, and if they are
                // not equal, increments the program counter by 2.

                let regx = ((instr << 4) >> 12) as u8;
                let byte = ((instr << 8) >> 8) as u8;
                let x = self.get_reg(regx);

                if x != byte {
                    self.pc += INSTRUCTION_SIZE;
                }
            },
            0x5 => {
                // 5XY0 - SE VX, VY
                //
                // Skip the next instruction if VX == VY.

                let regx = ((instr << 4) >> 12) as u8;
                let regy = ((instr << 8) >> 12) as u8;
                let x = self.get_reg(regx);
                let y = self.get_reg(regy);

                if x == y {
                    self.pc += INSTRUCTION_SIZE;
                }
            },
            0x6 => {
                // 6XNN - LD VX, NN
                //
                // Sets VX to NN.

                let regx = ((instr << 4) >> 12) as u8;
                let byte = ((instr << 8) >> 8) as u8;
                self.set_reg(regx, byte);
            },
            0x7 => {
                // 7XNN - ADD VX, NN
                //
                // Adds NN to VX.

                let regx = ((instr << 4) >> 12) as u8;
                let byte = ((instr << 8) >> 8) as u8;
                let result = self.get_reg(regx).wrapping_add(byte);
                self.set_reg(regx, result);
            },
            0x8 => {
                let regx = ((instr << 4) >> 12) as u8;
                let regy = ((instr << 8) >> 12) as u8;
                let identifier = ((instr << 12) >> 12) as u8;

                match identifier {
                    0x0 => {
                        // 8XY0 - LD VX, VY
                        //
                        // Sets VX to VY.

                        let y = self.get_reg(regy);
                        self.set_reg(regx, y);
                    },
                    0x1 => {
                        // 8XY1 - OR VX, VY
                        //
                        // Sets VX to VX or VY.

                        let x = self.get_reg(regx);
                        let y = self.get_reg(regy);
                        self.set_reg(regx, x | y);
                    },
                    0x2 => {
                        // 8XY2 - AND VX, VY
                        //
                        // Sets VX to VX and VY.

                        let x = self.get_reg(regx);
                        let y = self.get_reg(regy);
                        self.set_reg(regx, x & y);
                    },
                    0x3 => {
                        // 8XY3 - XOR VX, VY
                        //
                        // Sets VX to VX xor VY.

                        let x = self.get_reg(regx);
                        let y = self.get_reg(regy);
                        self.set_reg(regx, x ^ y);
                    },
                    0x4 => {
                        // 8XY4 - ADD VX, VY
                        //
                        // The values of VX and VY are added together. If the
                        // result is greater than 8 bits (i.e., > 255,) VF is
                        // set to 1, otherwise 0. Only the lowest 8 bits of the
                        // result are kept, and stored in VX.

                        let x = self.get_reg(regx) as u16;
                        let y = self.get_reg(regy) as u16;

                        let result = x.wrapping_add(y);
                        if result > 255 {
                            self.vf = 1;
                        } else {
                            self.vf = 0;
                        }
                        self.set_reg(regx, result as u8);
                    },
                    0x5 => {
                        // 8XY5 - SUB VX, VY
                        //
                        // If VX > VY, then VF is set to 1, otherwise 0. Then
                        // VY is subtracted from VX, and the results stored in
                        // VX.

                        let x = self.get_reg(regx) as u16;
                        let y = self.get_reg(regy) as u16;

                        if x > y {
                            self.vf = 1;
                        } else {
                            self.vf = 0;
                        }
                        let result = x.wrapping_sub(y);
                        self.set_reg(regx, result as u8);
                    },
                    0x6 => {
                        // 8XY6 - SHR VX {, VY}
                        //
                        // If the least-significant bit of VX is 1, then VF is
                        // set to 1, otherwise 0. Then VX is divided by 2.

                        let x = self.get_reg(regx);
                        let lsb = x & 0x1;

                        self.vf = lsb;
                        self.set_reg(regx, x.wrapping_div(2));
                    },
                    0x7 => {
                        // 8XY7 - SUBN VX, VY
                        //
                        // If VY > VX, then VF is set to 1, otherwise 0. Then
                        // VX is subtracted from VY, and the results stored in
                        // VX.

                        let x = self.get_reg(regx);
                        let y = self.get_reg(regy);

                        if y > x {
                            self.vf = 1;
                        } else {
                            self.vf = 0;
                        }
                        let result = y.wrapping_sub(x);
                        self.set_reg(regx, result);
                    },
                    0xe => {
                        // 8XYE - SHL VX {, VY}
                        //
                        // If the most-significant bit of VX is 1, then VF is
                        // set to 1, otherwise to 0. Then VX is multiplied by 2.

                        let x = self.get_reg(regx);
                        let msb = (x & 0x80) >> 7;

                        self.vf = msb;
                        self.set_reg(regx, x.wrapping_mul(2));
                    },
                    _ => {
                        println!("cpu: {:#?}", self);
                        panic!("Found unknown identifier at instruction: {:#x}, addr: {:#x}", instr, self.pc);
                    },
                }
            },
            0x9 => {
                // 9XY0 - SNE VX, VY
                //
                // Skip the next instruction if VX != VY.

                let regx = ((instr << 4) >> 12) as u8;
                let regy = ((instr << 8) >> 12) as u8;
                let x = self.get_reg(regx);
                let y = self.get_reg(regy);

                if x != y {
                    self.pc += INSTRUCTION_SIZE;
                }
            },
            0xa => {
                // ANNN - LD I, NNN
                //
                // Sets I to the address NNN.

                let addr = ((instr << 4) >> 4) as u16;
                self.i = addr;
            },
            0xb => {
                // BNNN - JP V0, NNN
                //
                // The program counter is set to NNN plus the value of V0.

                let addr = ((instr << 4) >> 4) as u16;
                self.pc = addr.wrapping_add(self.v0 as u16);
                skip = true;
            },
            0xc => {
                // CXNN - RND VX, NN
                //
                // Sets VX to the result of a bitwise and operation on a
                // random number and NN.

                let regx = ((instr << 4) >> 12) as u8;
                let byte = ((instr << 8) >> 8) as u8;
                let rnd = random::<u8>();
                self.set_reg(regx, rnd & byte);
            }
            0xd => {
                // DXYN - DRW VX, VY, N
                //
                // Sprites stored in memory at location in index register (I),
                // 8bits wide. Wraps around the screen. If when drawn, clears a
                // pixel, register VF is set to 1 otherwise it is zero.
                //
                // All drawing is XOR drawing (i.e. it toggles the screen
                // pixels). Sprites are drawn starting at position VX, VY. N is
                // the number of 8bit rows that need to be drawn. If N is
                // greater than 1, second line continues at position VX, VY+1,
                // and so on.

                let regx = ((instr << 4) >> 12) as u8;
                let regy = ((instr << 8) >> 12) as u8;
                let nibble = ((instr << 12) >> 12) as usize;

                // Read N (nibble) bytes out out of memory starting at address
                // register I into our sprite.
                let mut sprite = vec![0 as u8; nibble];
                for i in 0..nibble {
                    sprite[i] = self.interconnect.memory.read(self.i as usize + i);
                }

                // Get screen coordinates from the requested registers.
                let x = self.get_reg(regx);
                let y = self.get_reg(regy);

                // Draw the sprite and store collision detection results in vf.
                self.vf = self.interconnect.draw(x as usize, y as usize, sprite);
            },
            0xe => {
                let regx = ((instr << 4) >> 12) as u8;
                let identifier = ((instr << 8) >> 8) as u8;

                match identifier {
                    0x9e => {
                        // EX9E - SKP VX
                        //
                        // Skips the next instruction if the key stored in VX
                        // is pressed.

                        let x = self.get_reg(regx);
                        if self.interconnect.input_state[x as usize] {
                            self.pc += INSTRUCTION_SIZE;
                        }
                    },
                    0xa1 => {
                        // EXA1 - SKNP VX
                        //
                        // Skips the next instruction if the key stored in VX
                        // isn't pressed.

                        let x = self.get_reg(regx);
                        if !self.interconnect.input_state[x as usize] {
                            self.pc += INSTRUCTION_SIZE;
                        }
                    },
                    _ => {
                        println!("cpu: {:#?}", self);
                        panic!("Found unknown identifier at instruction: {:#x}, addr: {:#x}", instr, self.pc);
                    },
                }
            },
            0xf => {
                let regx = ((instr << 4) >> 12) as u8;
                let identifier = ((instr << 8) >> 8) as u8;

                match identifier {
                    0x07 => {
                        // FX07 - LD VX, DT
                        //
                        // Sets VX to the value of the delay timer.

                        let dt = self.dt;
                        self.set_reg(regx, dt);
                    },
                    0x0a => {
                        // FX0A - LD VX, N
                        //
                        // All execution stops until a key is pressed, then the
                        // value of that key is stored in VX.

                        println!("Waiting for input...");
                        let key = self.interconnect.wait_input();
                        self.set_reg(regx, key);
                    },
                    0x15 => {
                        // FX15 - LD DT, VX
                        //
                        // Sets the delay timer to VX.

                        let x = self.get_reg(regx);
                        self.dt = x;
                    },
                    0x18 => {
                        // FX18 - LD ST, VX
                        //
                        // ST is set equal to the value of VX.

                        let x = self.get_reg(regx);
                        self.st = x;
                    },
                    0x1e => {
                        // FX1E - ADD I, VX
                        //
                        // The values of I and VX are added, and the results
                        // are stored in I.

                        let x = self.get_reg(regx);
                        self.i = self.i.wrapping_add(x as u16);
                    },
                    0x29 => {
                        // FX29 - LD F, VX
                        //
                        // Sets I to the location of the sprite for the
                        // character in VX. Characters 0-F (in hexadecimal) are
                        // represented by a 4x5 font.

                        let x = self.get_reg(regx);
                        self.i = self.interconnect.memory.get_font(x);
                    },
                    0x33 => {
                        // FX33 - LD B, VX
                        //
                        // Stores the Binary-coded decimal representation of VX,
                        // with the most significant of three digits at the
                        // address in I, the middle digit at I plus 1, and the
                        // least significant digit at I plus 2. (In other words,
                        // take the decimal representation of VX, place the
                        // hundreds digit in memory at location in I, the tens
                        // digit at location I+1, and the ones digit at
                        // location I+2.)

                        const DECIMAL_LENGTH: usize = 3;

                        let mut x = self.get_reg(regx);
                        let mut digits = vec![0 as u8; DECIMAL_LENGTH];
                        let mut digit_count: usize = 0;

                        // Organize the digits in the decimal into a slice.
                        while x > 0 {
                            digit_count += 1;
                            digits[DECIMAL_LENGTH - digit_count] = x % 10;
                            x /= 10;
                        }

                        // Set I, I+1, and I+3 to the values of the digits.
                        let i = self.i as usize;
                        self.interconnect.memory.write(i, digits[0]);
                        self.interconnect.memory.write(i + 1, digits[1]);
                        self.interconnect.memory.write(i + 2, digits[2]);
                    },
                    0x55 => {
                        // FX55 - LD [I], VX
                        //
                        // The interpreter copies the values of registers V0
                        // through VX into memory, starting at the address in I.

                        let i = self.i as usize;
                        let end_reg = (regx + 1) as usize;

                        for register in 0x0..end_reg {
                            let val = self.get_reg(register as u8);
                            self.interconnect.memory.write(i + register, val);
                        }
                    },
                    0x65 => {
                        // FX65 - LD VX, [I]
                        //
                        // Fills V0 to VX with values from memory starting at
                        // address I.

                        let i = self.i as usize;
                        let end_reg = (regx + 1) as usize;

                        for register in 0x0..end_reg {
                            let mem = self.interconnect.memory.read(i + register);
                            self.set_reg(register as u8, mem);
                        }
                    },
                    _ => {
                        println!("cpu: {:#?}", self);
                        panic!("Found unknown identifier at instruction: {:#x}, addr: {:#x}", instr, self.pc);
                    },
                }
            },
            _ => {
                println!("cpu: {:#?}", self);
                panic!("Found unknown opcode at instruction: {:#x}, addr: {:#x}", instr, self.pc);
            },
        }

        // Increment the program counter to the next instruction.
        if !ret && !skip {
            self.pc += INSTRUCTION_SIZE;
        }

        // By default the execution loop in not broken. True will be returned
        // only by a successful RET instruction is executed.
        ret
    }

    /// Handle the delay timer and play sounds.
    fn handle_timers(&mut self) {
        let dt_enabled = self.dt > 0;
        let st_enabled = self.st > 0;

        if st_enabled {
            self.interconnect.beeping = true;
        } else {
            self.interconnect.beeping = false;
        }

        if dt_enabled || st_enabled {
            sleep(Duration::from_millis(TIMER_DELAY));

            if dt_enabled {
                self.dt -= 1;
            }
            if st_enabled {
                self.st -= 1;
            }
        } else {
            sleep(Duration::from_millis(EXECUTION_DELAY));
        }
    }

    /// Gets the value at a specified register.
    fn get_reg(&mut self, reg: u8) -> u8 {
        match reg {
            0x0 => self.v0,
            0x1 => self.v1,
            0x2 => self.v2,
            0x3 => self.v3,
            0x4 => self.v4,
            0x5 => self.v5,
            0x6 => self.v6,
            0x7 => self.v7,
            0x8 => self.v8,
            0x9 => self.v9,
            0xa => self.va,
            0xb => self.vb,
            0xc => self.vc,
            0xd => self.vd,
            0xe => self.ve,
            0xf => self.vf,
            _ => {
                panic!("Cannot get unknown register: V{:X}", reg);
            }
        }
    }

    /// Sets the value of a general purpose register.
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
