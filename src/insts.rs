use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RamAddress {
    pub bank: u8,
    pub address: u16,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AbstructAddress{
    RamAddress(RamAddress),
    Label(String),
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Operand {
    Im(u16),
    Address(AbstructAddress),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct OpInfo {
    pub num_bytes: u8,
    pub num_cycles: u8,
    pub opcode: u8,
}
#[derive(strum_macros::Display, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}
use std::str::FromStr;
impl FromStr for Opcode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let s : &str = &s.to_uppercase();
        match s {
            "ADC" => Ok(Opcode::ADC),
            "AND" => Ok(Opcode::AND),
            "ASL" => Ok(Opcode::ASL),
            "BCC" => Ok(Opcode::BCC),
            "BCS" => Ok(Opcode::BCS),
            "BEQ" => Ok(Opcode::BEQ),
            "BIT" => Ok(Opcode::BIT),
            "BMI" => Ok(Opcode::BMI),
            "BNE" => Ok(Opcode::BNE),
            "BPL" => Ok(Opcode::BPL),
            "BRK" => Ok(Opcode::BRK),
            "BVC" => Ok(Opcode::BVC),
            "BVS" => Ok(Opcode::BVS),
            "CLC" => Ok(Opcode::CLC),
            "CLD" => Ok(Opcode::CLD),
            "CLI" => Ok(Opcode::CLI),
            "CLV" => Ok(Opcode::CLV),
            "CMP" => Ok(Opcode::CMP),
            "CPX" => Ok(Opcode::CPX),
            "CPY" => Ok(Opcode::CPY),
            "DEC" => Ok(Opcode::DEC),
            "DEX" => Ok(Opcode::DEX),
            "DEY" => Ok(Opcode::DEY),
            "EOR" => Ok(Opcode::EOR),
            "INC" => Ok(Opcode::INC),
            "INX" => Ok(Opcode::INX),
            "INY" => Ok(Opcode::INY),
            "JMP" => Ok(Opcode::JMP),
            "JSR" => Ok(Opcode::JSR),
            "LDA" => Ok(Opcode::LDA),
            "LDX" => Ok(Opcode::LDX),
            "LDY" => Ok(Opcode::LDY),
            "LSR" => Ok(Opcode::LSR),
            "NOP" => Ok(Opcode::NOP),
            "ORA" => Ok(Opcode::ORA),
            "PHA" => Ok(Opcode::PHA),
            "PHP" => Ok(Opcode::PHP),
            "PLA" => Ok(Opcode::PLA),
            "PLP" => Ok(Opcode::PLP),
            "ROL" => Ok(Opcode::ROL),
            "ROR" => Ok(Opcode::ROR),
            "RTI" => Ok(Opcode::RTI),
            "RTS" => Ok(Opcode::RTS),
            "SBC" => Ok(Opcode::SBC),
            "SEC" => Ok(Opcode::SEC),
            "SED" => Ok(Opcode::SED),
            "SEI" => Ok(Opcode::SEI),
            "STA" => Ok(Opcode::STA),
            "STX" => Ok(Opcode::STX),
            "STY" => Ok(Opcode::STY),
            "TAX" => Ok(Opcode::TAX),
            "TAY" => Ok(Opcode::TAY),
            "TSX" => Ok(Opcode::TSX),
            "TXA" => Ok(Opcode::TXA),
            "TXS" => Ok(Opcode::TXS),
            "TYA" => Ok(Opcode::TYA),        
            _ => Err(())
        }
    }
}
#[derive(strum_macros::Display, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Addressing{
    Implied,
    Accumulator,
    Immediate,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Instruction {
    opcode: Opcode,
    addressing: Addressing,
    operand: Option<Operand>,
    address: RamAddress,
}

impl Instruction {
    pub fn new(opcode: Opcode, addressing: Addressing, operand: Option<Operand>, address: RamAddress) -> Instruction {
        Instruction {
            opcode: opcode,
            addressing: addressing,
            operand: operand,
            address: address,
        }
    }
    pub fn get_inst_code(&self) -> Vec<u8> {
        if let None = self.operand {
            return vec![];
        }
        let address = match self.operand.as_ref().unwrap() {
            Operand::Address(address) => {
                match address {
                    AbstructAddress::RamAddress(ramAddress) => ramAddress.address,
                    AbstructAddress::Label(label) => 0, // todo label解決
                }
            },
            Operand::Im(im) => {
                im.clone()
            }
        };
        
        let info = self.get_op_info();
        let mut v = vec![info.opcode];
        match info.num_bytes {
            1 => return v,
            2 => {
                let operand: u8 = TryFrom::try_from(address).unwrap();
                v.push(operand);
                return v;
            },
            3 => {
                let bytes = address.to_le_bytes(); // read as little endian
                v.push(bytes[0]);
                v.push(bytes[1]);
                return v;
            },
            _ => panic!(),
        }
    }
    pub fn get_op_info(&self) -> OpInfo{
        match self.opcode {
            Opcode::ADC => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode: 0x69},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode: 0x65},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode: 0x75},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode: 0x6d},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode: 0x7d},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode: 0x79},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode: 0x61},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 5, opcode: 0x71},
                _ => panic!("Invalid inst_code"),
            },
            Opcode::AND => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode: 0x29},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode: 0x25},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode: 0x35},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode: 0x2d},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode: 0x3d},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode: 0x39},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode: 0x21},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 5, opcode: 0x31},
                _ => panic!("invalid inst_code"),
            },
            Opcode::ASL => match self.addressing {
                Addressing::Accumulator => OpInfo{num_bytes: 1, num_cycles: 2, opcode: 0x0a},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 5, opcode: 0x06},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 6, opcode: 0x16},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 6, opcode: 0x0e},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 7, opcode: 0x1e},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BCC => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0x90},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BCS => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xb0},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BEQ => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xf0},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BIT => match self.addressing {
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0x24},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x2c},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BMI => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0x30},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BNE => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xd0},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BPL => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0x10},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BRK => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 7, opcode:  0x00},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BVC => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0x50},
                _ => panic!("invalid inst_code"),
            },
            Opcode::BVS => match self.addressing {
                Addressing::Relative => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0x70},
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLC => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x18},
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLD => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xd8},
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLI => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x58},
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLV => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xb8},
                _ => panic!("invalid inst_code"),
            },
            Opcode::CMP => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xc9},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0xc5},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0xd5},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xcd},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xdd},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xd9},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0xc1},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0xd1},
                _ => panic!("invalid inst_code"),
            },
            Opcode::CPX => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xe0},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0xe4},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xec},
                _ => panic!("invalid inst_code"),
            },
            Opcode::CPY => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xc0},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0xc4},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xcc},
                _ => panic!("invalid inst_code"),
            },
            Opcode::DEC => match self.addressing {
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0xc6},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0xd6},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 6, opcode:  0xce},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 7, opcode:  0xde},
                _ => panic!("invalid inst_code"),
            },
            Opcode::DEX => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xca},
                _ => panic!("invalid inst_code"),
            },
            Opcode::DEY => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x88},
                _ => panic!("invalid inst_code"),
            },
            Opcode::EOR => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0x49},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0x45},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0x55},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x4d},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x5d},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x59},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0x41},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0x51},
                _ => panic!("invalid inst_code"),
            },
            Opcode::INC => match self.addressing {
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0xe6},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0xf6},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 6, opcode:  0xee},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 7, opcode:  0xfe},
                _ => panic!("invalid inst_code"),
            },
            Opcode::INX => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xe8},
                _ => panic!("invalid inst_code"),
            },
            Opcode::INY => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xc8},
                _ => panic!("invalid inst_code"),
            },
            Opcode::JMP => match self.addressing {
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 3, opcode:  0x4c},
                Addressing::Indirect => OpInfo{num_bytes: 3, num_cycles: 5, opcode:  0x6c},
                _ => panic!("invalid inst_code"),
            },
            Opcode::JSR => match self.addressing {
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 6, opcode:  0x20},
                _ => panic!("invalid inst_code"),
            },
            Opcode::LDA => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xa9},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0xa5},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0xb5},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xad},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xbd},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xb9},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0xa1},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0xb1},
                _ => panic!("invalid inst_code"),
            },
            Opcode::LDX => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xa2},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0xa6},
                Addressing::ZeropageY => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0xb6},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xae},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xbe},
                _ => panic!("invalid inst_code"),
            },
            Opcode::LDY => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xa0},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0xa4},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0xb4},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xac},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xbc},
                _ => panic!("invalid inst_code"),
            },
            Opcode::LSR => match self.addressing {
                Addressing::Accumulator => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x4a},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0x46},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0x56},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 6, opcode:  0x4e},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 7, opcode:  0x5e},
                _ => panic!("invalid inst_code"),
            },
            Opcode::NOP => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xea},
                _ => panic!("invalid inst_code"),
            },
            Opcode::ORA => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0x09},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0x05},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0x15},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x0d},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x1d},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x19},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0x01},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0x11},
                _ => panic!("invalid inst_code"),
            },
            Opcode::PHA => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 3, opcode:  0x48},
                _ => panic!("invalid inst_code"),
            },
            Opcode::PHP => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 3, opcode:  0x08},
                _ => panic!("invalid inst_code"),
            },
            Opcode::PLA => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 4, opcode:  0x68},
                _ => panic!("invalid inst_code"),
            },
            Opcode::PLP => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 4, opcode:  0x28},
                _ => panic!("invalid inst_code"),
            },
            Opcode::ROL => match self.addressing {
                Addressing::Accumulator => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x2a},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0x26},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0x36},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 6, opcode:  0x2e},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 7, opcode:  0x3e},
                _ => panic!("invalid inst_code"),
            },
            Opcode::ROR => match self.addressing {
                Addressing::Accumulator => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x6a},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0x66},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0x76},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 6, opcode:  0x6e},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 7, opcode:  0x7e},
                _ => panic!("invalid inst_code"),
            },
            Opcode::RTI => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 6, opcode:  0x40},
                _ => panic!("invalid inst_code"),
            },
            Opcode::RTS => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 6, opcode:  0x60},
                _ => panic!("invalid inst_code"),
            },
            Opcode::SBC => match self.addressing {
                Addressing::Immediate => OpInfo{num_bytes: 2, num_cycles: 2, opcode:  0xe9},
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0xe5},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0xf5},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xed},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xfd},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0xf9},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0xe1},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 5, opcode:  0xf1},
                _ => panic!("invalid inst_code"),
            },
            Opcode::SEC => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x38},
                _ => panic!("invalid inst_code"),
            },
            Opcode::SED => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xf8},
                _ => panic!("invalid inst_code"),
            },
            Opcode::SEI => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x78},
                _ => panic!("invalid inst_code"),
            },
            Opcode::STA => match self.addressing {
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0x85},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0x95},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x8d},
                Addressing::AbsoluteX => OpInfo{num_bytes: 3, num_cycles: 5, opcode:  0x9d},
                Addressing::AbsoluteY => OpInfo{num_bytes: 3, num_cycles: 5, opcode:  0x99},
                Addressing::IndirectX => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0x81},
                Addressing::IndirectY => OpInfo{num_bytes: 2, num_cycles: 6, opcode:  0x91},
                _ => panic!("invalid inst_code"),
            },
            Opcode::STX => match self.addressing {
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0x86},
                Addressing::ZeropageY => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0x96},
                Addressing::Absolute => OpInfo{num_bytes: 4, num_cycles: 4, opcode:  0x8e},
                _ => panic!("invalid inst_code"),
            },
            Opcode::STY => match self.addressing {
                Addressing::Zeropage => OpInfo{num_bytes: 2, num_cycles: 3, opcode:  0x84},
                Addressing::ZeropageX => OpInfo{num_bytes: 2, num_cycles: 4, opcode:  0x94},
                Addressing::Absolute => OpInfo{num_bytes: 3, num_cycles: 4, opcode:  0x8c},
                _ => panic!("invalid inst_code"),
            },
            Opcode::TAX => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xaa},
                _ => panic!("invalid inst_code"),
            },
            Opcode::TAY => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xa8},
                _ => panic!("invalid inst_code"),
            },
            Opcode::TSX => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0xba},
                _ => panic!("invalid inst_code"),
            },
            Opcode::TXA => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x8a},
                _ => panic!("invalid inst_code"),
            },
            Opcode::TXS => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x9a},
                _ => panic!("invalid inst_code"),
            },
            Opcode::TYA => match self.addressing {
                Addressing::Implied => OpInfo{num_bytes: 1, num_cycles: 2, opcode:  0x98},
                _ => panic!("invalid inst_code"),
            }
        }
    }
}
