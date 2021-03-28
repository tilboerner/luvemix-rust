mod types {
    pub type Byte = u8;
    pub type Word = u16;
    pub type Address = Word;
    pub type Data = Byte;
    pub type Flags = Data;

    pub const DATA_WIDTH: u8 = 8;
}

mod cpu {

    use crate::types::*;

    pub enum StatusFlag {
        /// Zero
        ZRO = 0b0000_0010,

        /// Negative
        NEG = 0b1000_0000,
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

        pub fn get_flag(&self, flag: StatusFlag) -> bool {
            self.sr & flag.to_mask() != 0
        }

        pub fn set_flag(&mut self, flag: StatusFlag, val: bool) {
            let mask = flag.to_mask();
            let flag_val = if val { self.sr | mask } else { self.sr & !mask };
            self.sr = flag_val;
        }

        pub fn set_a(&mut self, val: Data) {
            self.a = val;
            self.set_flag(StatusFlag::ZRO, val == 0);
            self.set_flag(StatusFlag::NEG, (val >> DATA_WIDTH - 1) > 0);
        }
    }
}

#[cfg(test)]
mod test {

    use crate::cpu::*;
    use crate::types::*;

    #[test]
    fn test_get_set_flag() {
        let mut cpu = CpuState::new();

        assert_eq!(cpu.get_flag(StatusFlag::ZRO), false);
        cpu.set_flag(StatusFlag::ZRO, true);
        assert_eq!(cpu.get_flag(StatusFlag::ZRO), true);
        cpu.set_flag(StatusFlag::ZRO, false);
        assert_eq!(cpu.get_flag(StatusFlag::ZRO), false);
    }

    #[test]
    fn test_get_set_flag_ignores_other_flags() {
        let mut cpu = CpuState::new();

        assert_eq!(cpu.get_flag(StatusFlag::ZRO), false);
        cpu.set_flag(StatusFlag::NEG, true);
        assert_eq!(cpu.get_flag(StatusFlag::ZRO), false);
        cpu.set_flag(StatusFlag::ZRO, false);
        assert_eq!(cpu.get_flag(StatusFlag::NEG), true);
    }

    #[test]
    fn test_set_a_sets_a() {
        let mut cpu = CpuState::new();

        cpu.set_a(42);

        assert_eq!(cpu.a, 42);
    }

    #[test]
    fn test_set_a_sets_zro() {
        let mut cpu = CpuState::new();

        cpu.set_a(0);

        assert_eq!(cpu.get_flag(StatusFlag::ZRO), true);

        cpu.set_a(42);

        assert_eq!(cpu.get_flag(StatusFlag::ZRO), false);
    }

    #[test]
    fn test_set_a_sets_neg() {
        let mut cpu = CpuState::new();

        cpu.set_a(1 << DATA_WIDTH - 1);

        assert_eq!(cpu.get_flag(StatusFlag::NEG), true);

        cpu.set_a(0);

        assert_eq!(cpu.get_flag(StatusFlag::NEG), false);
    }
}

fn main() {
    let mut cpu = cpu::CpuState::new();
    println!("Hello {:?}", cpu);
    cpu.set_flag(cpu::StatusFlag::ZRO, true);
    println!("Zero {:?}", cpu.get_flag(cpu::StatusFlag::ZRO));
    println!("Negative {:?}", cpu.get_flag(cpu::StatusFlag::NEG));
}
