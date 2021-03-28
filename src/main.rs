mod types {
    pub type Byte = u8;
    pub type Word = u16;
    pub type Address = Word;
    pub type Data = Byte;
}

mod cpu {

    use crate::types::*;

    #[derive(Debug)]
    pub struct CpuState {
        /// Program Counter
        pub pc: Address,

        /// Stack Pointer
        pub sp: Address,

        /// Accumulator Register
        pub a: Data,

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
                pc: 0,
                sp: 0,
                a: 0,
                ir: 0,
                mar: 0,
                mdr: 0,
            }
        }
    }
}

fn main() {
    let cpu = cpu::CpuState::new();
    println!("Hello {:?}", cpu);
}
