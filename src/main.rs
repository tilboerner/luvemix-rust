mod types {
    pub type Byte = u8;
    pub type Word = u16;
    pub type Address = Word;
    pub type Data = Byte;
    pub type Flags = Data;
}

mod cpu {

    use crate::types::*;

    pub enum StatusFlag {
        /// Zero
        ZRO = 0b0000_0010,
    }

    impl StatusFlag {
        pub fn to_mask(self) -> Flags {
            self as Flags
        }
    }

    #[derive(Debug)]
    pub struct CpuState {
        /// Program Counter
        pub pc: Address,

        /// Stack Pointer
        pub sp: Address,

        /// Accumulator Register
        pub a: Data,

        // Status Register
        sr: Flags,

        // Instruction Register
        ir: Data,

        // Memory Address Register
        mar: Address,

        // Memory Data Register
        mdr: Data,
    }

    impl CpuState {
        pub fn new() -> CpuState {
            CpuState {
                sr: 0,
                pc: 0,
                sp: 0,
                a: 0,
                ir: 0,
                mar: 0,
                mdr: 0,
            }
        }

        pub fn get_flag(self, flag: StatusFlag) -> bool {
            self.sr & flag.to_mask() != 0
        }

        pub fn set_flag(&mut self, flag: StatusFlag, val: bool) {
            let mask = flag.to_mask();
            let flag_val = if val { self.sr | mask } else { self.sr & !mask };
            self.sr = flag_val;
        }
    }
}

fn main() {
    let mut cpu = cpu::CpuState::new();
    println!("Hello {:?}", cpu);
    cpu.set_flag(cpu::StatusFlag::ZRO, true);
    println!("Zero {:?}", cpu.get_flag(cpu::StatusFlag::ZRO));
}
