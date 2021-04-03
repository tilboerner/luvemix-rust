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

    pub enum Flag {
        /// Zero
        ZRO = 0b0000_0010,

        /// Negative
        NEG = 0b1000_0000,
    }

    impl Flag {
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
        pub ir: Data,

        // Memory Address Register
        pub mar: Address,

        // Memory Data Register
        pub mdr: Data,
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

        pub fn get_flag(&self, flag: Flag) -> bool {
            self.sr & flag.to_mask() != 0
        }

        pub fn set_flag(&mut self, flag: Flag, val: bool) {
            let mask = flag.to_mask();
            let flag_val = if val { self.sr | mask } else { self.sr & !mask };
            self.sr = flag_val;
        }

        pub fn set_a(&mut self, val: Data) {
            self.a = val;
            self.set_flag(Flag::ZRO, val == 0);
            self.set_flag(Flag::NEG, (val >> DATA_WIDTH - 1) > 0);
        }
    }

    pub trait Memory {
        fn read(&self, addr: &Address) -> Option<&Data>;
        fn write(&mut self, addr: Address, val: Data);
    }

    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct CheapoMemory {
        map: HashMap<Address, Data>,
    }

    impl CheapoMemory {
        pub fn new() -> CheapoMemory {
            CheapoMemory {
                map: HashMap::new(),
            }
        }
    }

    impl Memory for CheapoMemory {
        fn read(&self, addr: &Address) -> Option<&Data> {
            self.map.get(&addr)
        }

        fn write(&mut self, addr: Address, val: Data) {
            self.map.insert(addr, val);
        }
    }

    #[derive(Debug)]
    pub struct Cpu<'a> {
        state: CpuState,
        mem: &'a mut CheapoMemory,
    }

    impl<'a> Cpu<'a> {
        pub fn new(mem: &mut CheapoMemory) -> Cpu {
            Cpu {
                state: CpuState::new(),
                mem: mem,
            }
        }

        pub fn execute_cycle(&mut self, val: Data) {
            self.mem.write(self.state.mar, val);
            let data = self.mem.read(&self.state.mar);
            let data = data.unwrap();
            self.state.set_a(*data);
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

        assert_eq!(cpu.get_flag(Flag::ZRO), false);
        cpu.set_flag(Flag::ZRO, true);
        assert_eq!(cpu.get_flag(Flag::ZRO), true);
        cpu.set_flag(Flag::ZRO, false);
        assert_eq!(cpu.get_flag(Flag::ZRO), false);
    }

    #[test]
    fn test_get_set_flag_ignores_other_flags() {
        let mut cpu = CpuState::new();

        assert_eq!(cpu.get_flag(Flag::ZRO), false);
        cpu.set_flag(Flag::NEG, true);
        assert_eq!(cpu.get_flag(Flag::ZRO), false);
        cpu.set_flag(Flag::ZRO, false);
        assert_eq!(cpu.get_flag(Flag::NEG), true);
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

        assert_eq!(cpu.get_flag(Flag::ZRO), true);

        cpu.set_a(42);

        assert_eq!(cpu.get_flag(Flag::ZRO), false);
    }

    #[test]
    fn test_set_a_sets_neg() {
        let mut cpu = CpuState::new();

        cpu.set_a(1 << DATA_WIDTH - 1);

        assert_eq!(cpu.get_flag(Flag::NEG), true);

        cpu.set_a(0);

        assert_eq!(cpu.get_flag(Flag::NEG), false);
    }

    #[test]
    fn test_cheapo_memory_readwrite() {
        let mut m = CheapoMemory::new();

        assert_eq!(m.read(&0), None);

        m.write(0, 42);

        assert_eq!(m.read(&0).unwrap(), &42);

        m.write(0, 43);

        assert_eq!(m.read(&0).unwrap(), &43);
    }
}

fn main() {
    use cpu::*;
    let state = CpuState::new();
    println!("Hello {:?}", state);
    println!("A {:?}", state.a);
    println!("Zero {:?}", state.get_flag(Flag::ZRO));
    println!("Negative {:?}", state.get_flag(Flag::NEG));

    let mut mem = CheapoMemory::new();
    let mut cpu = Cpu::new(&mut mem);

    cpu.execute_cycle(42);

    println!("Cpu {:?}", cpu);
}
