mod instructions;

const ROM_SIZE: u16 = 1024;
const RAM_SIZE: u16 = 1024;
const DATA_SIZE: u8 = 8;

enum  AR { // Selects A or B from the arithmetic registers
    A,
    B
}
enum IX { // Selects X or Y from the index registers
    X,
    Y
}

struct Computer {
    A: u8, // Selected by AR::A
    B: u8, // Selected by AR::B

    X: u8, // Selected by IX::X
    Y: u8, // Selected by IX::Y


    PC: u8, // Program Counter

    rom: [u8; ROM_SIZE*DATA_SIZE],
    ram: [u8; RAM_SIZE*DATA_SIZE]

    // These registers are used in the G-CPU,
    // but commented out because they are probably not needed in implementation (for now)
    // IR: u8; // Instruction Register
    // MAR: u16; // Memory Address Register



    // Memory control signals (also probably not necessary?)
    // ROM_ENABLE: bool;
    // RAM_ENABLE: bool;
    // RAM_RD_EN: bool;
    // RAM_WR_EN: bool;
}

impl Computer {

    fn get_data(addr: u16) -> u8 { // Assumes ROM before RAM in memory
        if (addr < ROM_Size - 1) {
            return rom[addr*8]; // TODO: check address is in bounds
        } else {
            return rom[(addr - RAM_SIZE)*8];
        }
    }
    // scenarios: a function is called that needs 2 args, but only 1 is passed, so the next opcode is taken as an argument
    // in a mif it would just read this arg.. no way to verify if it's pure data, so it should be fine to just pass in the args
    // but the actual problem is that the opcode is needed to get the args, so this function doesn't quite work
    // ...enum??????? probably need to convert this whole thing
    // TODO: add error handling, need to check that args are correct before they're actually passed in

    // just needs to accept an array of hex values that starts at the opcode, and output an instruction
    // can finish putting the enums in when i've finalized that this is a useful function
    fn hex_to_instruction(hex: [u8]) -> Instruction {
        match opcode {
            // Data Movement Instructions
            0x00 => return Instruction::TAB, // TAB
            0x01 => return Instruction::TBA, // TBA
            0x02 => load_AR(AR::A),// LDAA #data
            0x03 => 0, // LDAB #data
            0x04 => 0,// LDAA addr // get address then call 
            0x05 => 0,// LDAB addr
            0x06 => 0,// STAA addr
            0x07 => 0,// STAB addr
            0x08 => 0,// LDX #data
            0x09 => 0,// LDY #data
            0x0A => 0,// LDX addr
            0x0B => 0,// LDY addr
            0x0C => 0,// LDAA dd,X
            0x0D => 0,// LDAA dd,Y
            0x0E => 0,// LDAB dd,X
            0x0F => 0,// LDAB dd,Y
            0x10 => 0,// STAA dd,X
            0x11 => 0,// STAA dd,Y
            0x12 => 0,// STAB dd,X
            0x13 => 0,// STAB dd,Y

            // ALU Instructions
            0x14 => 0,// SUM_BA
            0x15 => 0,// SUM_AB
            0x16 => 0,// AND_BA
            0x17 => 0,// AND_AB
            0x18 => 0,// OR_BA
            0x19 => 0,// OR_AB
            0x1A => 0,// COMA
            0x1B => 0,// COMB
            0x1C => 0,// SHFA_L
            0x1D => 0,// SHFA_R
            0x1E => 0,// SHFB_L
            0x1F => 0,// SHFB_R
            0x30 => 0,// INX
            0x31 => 0,// INY

            // Branch Instructions
            0x20 => 0,// BEQ
            0x21 => 0,// BNE
            0x22 => 0,// BN
            0x23 => 0// BP
        }
    }

    // takes an instruction, matches it, and does whatever operation on registers/memory it needs
    fn execute(inst: Instruction) {

    }

}