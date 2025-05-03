const ROM_SIZE: u16 = 1024;
const RAM_SIZE: u16 = 1024;
const DATA_SIZE: u8 = 8;

// ROM will occupy the lower address bits and RAM will occupy the upper bits, this assumption is made in
// ROM_SIZE and RAM_SIZE should be powers of two
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

    fn get_data(addr: u16) { // Assumes ROM before RAM in memory
        println!("hi")
    } // also might change

    // TODO: add error handling
    fn execute(opcode: u8, arg: u16) {
        match opcode {
            // Data Movement Instructions
            0x00 => transfer_AR(AR::B), // TAB
            0x01 => transfer_AR(AR::A), // TBA
            0x02 => 0,// LDAA #data
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

    fn transfer_AR(destination: AR) {
        match destination {
            AR::A => A = B,
            AR::B => B = A
        }
    }

    fn load_AR(destination: AR, data: u8) {
        match destination {
            AR::A => A = data,
            AR::B => B = data
        }
    }
}