#[derive(strum_macros::Display)]
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
pub enum Token{
    Op(Opcode),
    Im8(u8),
    Im8Adr(u8),
    Im16(u16),
    X,
    Y,
    A,
    Comma,
    LParen,
    RParen,
}

impl FromStr for Token {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "X" | "x" => Ok(Token::X),
            "Y" | "y" => Ok(Token::Y),
            "(" => Ok(Token::LParen),
            ")" => Ok(Token::RParen),
            _ => Ok(Token::A), // <= temporary
        }
    }
}



pub struct Instruction {
    opcode: Opcode,
    addressing: Addressing,
    operand: Option<u16>,
}
impl From<String> for Instruction {
    fn from(s: String) -> Instruction {
        let line = s.trim();
        let opcode : Opcode = Opcode::from_str(&line[0..3]).unwrap();
        match opcode {
            Opcode::BCC | Opcode::BCS |Opcode::BEQ |Opcode::BMI |Opcode::BNE |Opcode::BPL |Opcode::BVC |Opcode::BVS => {
                return Instruction::new(opcode, Addressing::Relative, Some(0x11));
            }
            _ => {}
        }
        let next = &line[4..].trim();

        // match next[0] {
        //     '('
        //     ''
        // }
        return Instruction::new(opcode, Addressing::Relative, Some(0x11));
    }
}
impl From<Vec<u8>> for Instruction {
    fn from(arr: Vec<u8>) -> Instruction {
        Instruction{
            opcode: Opcode::ADC,
            addressing: Addressing::Indirect,
            operand: Some(0x11),
        }
    }
}

impl Instruction {
    pub fn new(opcode: Opcode, addressing: Addressing, operand: Option<u16>) -> Instruction {
        Instruction {
            opcode: opcode,
            addressing: addressing,
            operand: operand,
        }
    }
    pub fn get_inst_code(&self) -> u8{
        match self.opcode {
            Opcode::ADC => match self.addressing {
                Addressing::Implied => 0x69,
                Addressing::Zeropage => 0x65,
                Addressing::ZeropageX => 0x75,
                Addressing::Absolute => 0x6d,
                Addressing::AbsoluteX => 0x7d,
                Addressing::AbsoluteY => 0x79,
                Addressing::IndirectX => 0x61,
                Addressing::IndirectY => 0x71,
                _ => panic!("Invalid inst_code"),
            },
            Opcode::AND => match self.addressing {
                Addressing::Immediate => 0x29,
                Addressing::Zeropage => 0x25,
                Addressing::ZeropageX => 0x35,
                Addressing::Absolute => 0x2d,
                Addressing::AbsoluteX => 0x3d,
                Addressing::AbsoluteY => 0x39,
                Addressing::IndirectX => 0x21,
                Addressing::IndirectY => 0x31,
                _ => panic!("invalid inst_code"),
            },
            Opcode::ASL => match self.addressing {
                Addressing::Accumulator => 0x0a,
                Addressing::Zeropage => 0x06,
                Addressing::ZeropageX => 0x16,
                Addressing::Absolute => 0x0e,
                Addressing::AbsoluteX => 0x1e,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BCC => match self.addressing {
                Addressing::Relative => 0x90,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BCS => match self.addressing {
                Addressing::Relative => 0xb0,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BEQ => match self.addressing {
                Addressing::Relative => 0xf0,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BIT => match self.addressing {
                Addressing::Zeropage => 0x24,
                Addressing::Absolute => 0x2c,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BMI => match self.addressing {
                Addressing::Relative => 0x30,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BNE => match self.addressing {
                Addressing::Relative => 0xd0,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BPL => match self.addressing {
                Addressing::Relative => 0x10,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BRK => match self.addressing {
                Addressing::Implied => 0x00,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BVC => match self.addressing {
                Addressing::Relative => 0x50,
                _ => panic!("invalid inst_code"),
            },
            Opcode::BVS => match self.addressing {
                Addressing::Relative => 0x70,
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLC => match self.addressing {
                Addressing::Implied => 0x18,
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLD => match self.addressing {
                Addressing::Implied => 0xd8,
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLI => match self.addressing {
                Addressing::Implied => 0x58,
                _ => panic!("invalid inst_code"),
            },
            Opcode::CLV => match self.addressing {
                Addressing::Implied => 0xb8,
                _ => panic!("invalid inst_code"),
            },
            Opcode::CMP => match self.addressing {
                Addressing::Immediate => 0xc9,
                Addressing::Zeropage => 0xc5,
                Addressing::ZeropageX => 0xd5,
                Addressing::Absolute => 0xcd,
                Addressing::AbsoluteX => 0xdd,
                Addressing::AbsoluteY => 0xd9,
                Addressing::IndirectX => 0xc1,
                Addressing::IndirectY => 0xd1,
                _ => panic!("invalid inst_code"),
            },
            Opcode::CPX => match self.addressing {
                Addressing::Immediate => 0xe0,
                Addressing::Zeropage => 0xe4,
                Addressing::Absolute => 0xec,
                _ => panic!("invalid inst_code"),
            },
            Opcode::CPY => match self.addressing {
                Addressing::Immediate => 0xc0,
                Addressing::Zeropage => 0xc4,
                Addressing::Absolute => 0xcc,
                _ => panic!("invalid inst_code"),
            },
            Opcode::DEC => match self.addressing {
                Addressing::Zeropage => 0xc6,
                Addressing::ZeropageX => 0xd6,
                Addressing::Absolute => 0xce,
                Addressing::AbsoluteX => 0xde,
                _ => panic!("invalid inst_code"),
            },
            Opcode::DEX => match self.addressing {
                Addressing::Implied => 0xca,
                _ => panic!("invalid inst_code"),
            },
            Opcode::DEY => match self.addressing {
                Addressing::Implied => 0x88,
                _ => panic!("invalid inst_code"),
            },
            Opcode::EOR => match self.addressing {
                Addressing::Immediate => 0x49,
                Addressing::Zeropage => 0x45,
                Addressing::ZeropageX => 0x55,
                Addressing::Absolute => 0x4d,
                Addressing::AbsoluteX => 0x5d,
                Addressing::AbsoluteY => 0x59,
                Addressing::IndirectX => 0x41,
                Addressing::IndirectY => 0x51,
                _ => panic!("invalid inst_code"),
            },
            Opcode::INC => match self.addressing {
                Addressing::Zeropage => 0xe6,
                Addressing::ZeropageX => 0xf6,
                Addressing::Absolute => 0xee,
                Addressing::AbsoluteX => 0xfe,
                _ => panic!("invalid inst_code"),
            },
            Opcode::INX => match self.addressing {
                Addressing::Implied => 0xe8,
                _ => panic!("invalid inst_code"),
            },
            Opcode::INY => match self.addressing {
                Addressing::Implied => 0xc8,
                _ => panic!("invalid inst_code"),
            },
            Opcode::JMP => match self.addressing {
                Addressing::Absolute => 0x4c,
                Addressing::Indirect => 0x6c,
                _ => panic!("invalid inst_code"),
            },
            Opcode::JSR => match self.addressing {
                Addressing::Absolute => 0x20,
                _ => panic!("invalid inst_code"),
            },
            Opcode::LDA => match self.addressing {
                Addressing::Immediate => 0xa9,
                Addressing::Zeropage => 0xa5,
                Addressing::ZeropageX => 0xb5,
                Addressing::Absolute => 0xad,
                Addressing::AbsoluteX => 0xbd,
                Addressing::AbsoluteY => 0xb9,
                Addressing::IndirectX => 0xa1,
                Addressing::IndirectY => 0xb1,
                _ => panic!("invalid inst_code"),
            },
            Opcode::LDX => match self.addressing {
                Addressing::Immediate => 0xa2,
                Addressing::Zeropage => 0xa6,
                Addressing::ZeropageY => 0xb6,
                Addressing::Absolute => 0xae,
                Addressing::AbsoluteY => 0xbe,
                _ => panic!("invalid inst_code"),
            },
            Opcode::LDY => match self.addressing {
                Addressing::Immediate => 0xa0,
                Addressing::Zeropage => 0xa4,
                Addressing::ZeropageX => 0xb4,
                Addressing::Absolute => 0xac,
                Addressing::AbsoluteX => 0xbc,
                _ => panic!("invalid inst_code"),
            },
            Opcode::LSR => match self.addressing {
                Addressing::Accumulator => 0x4a,
                Addressing::Zeropage => 0x46,
                Addressing::ZeropageX => 0x56,
                Addressing::Absolute => 0x4e,
                Addressing::AbsoluteX => 0x5e,
                _ => panic!("invalid inst_code"),
            },
            Opcode::NOP => match self.addressing {
                Addressing::Implied => 0xea,
                _ => panic!("invalid inst_code"),
            },
            Opcode::ORA => match self.addressing {
                Addressing::Immediate => 0x09,
                Addressing::Zeropage => 0x06,
                Addressing::ZeropageX => 0x15,
                Addressing::Absolute => 0x0d,
                Addressing::AbsoluteX => 0x1d,
                Addressing::AbsoluteY => 0x19,
                Addressing::IndirectX => 0x01,
                Addressing::IndirectY => 0x11,
                _ => panic!("invalid inst_code"),
            },
            Opcode::PHA => match self.addressing {
                Addressing::Implied => 0x48,
                _ => panic!("invalid inst_code"),
            },
            Opcode::PHP => match self.addressing {
                Addressing::Implied => 0x08,
                _ => panic!("invalid inst_code"),
            },
            Opcode::PLA => match self.addressing {
                Addressing::Implied => 0x68,
                _ => panic!("invalid inst_code"),
            },
            Opcode::PLP => match self.addressing {
                Addressing::Implied => 0x28,
                _ => panic!("invalid inst_code"),
            },
            Opcode::ROL => match self.addressing {
                Addressing::Accumulator => 0x2a,
                Addressing::Zeropage => 0x26,
                Addressing::ZeropageX => 0x36,
                Addressing::Absolute => 0x2e,
                Addressing::AbsoluteX => 0x3e,
                _ => panic!("invalid inst_code"),
            },
            Opcode::ROR => match self.addressing {
                Addressing::Accumulator => 0x6a,
                Addressing::Zeropage => 0x66,
                Addressing::ZeropageX => 0x76,
                Addressing::Absolute => 0x6e,
                Addressing::AbsoluteX => 0x7e,
                _ => panic!("invalid inst_code"),
            },
            Opcode::RTI => match self.addressing {
                Addressing::Implied => 0x40,
                _ => panic!("invalid inst_code"),
            },
            Opcode::RTS => match self.addressing {
                Addressing::Implied => 0x60,
                _ => panic!("invalid inst_code"),
            },
            Opcode::SBC => match self.addressing {
                Addressing::Immediate => 0xe9,
                Addressing::Zeropage => 0xe5,
                Addressing::ZeropageX => 0xf5,
                Addressing::Absolute => 0xed,
                Addressing::AbsoluteX => 0xfd,
                Addressing::AbsoluteY => 0xf9,
                Addressing::IndirectX => 0xe1,
                Addressing::IndirectY => 0xf1,
                _ => panic!("invalid inst_code"),
            },
            Opcode::SEC => match self.addressing {
                Addressing::Implied => 0x38,
                _ => panic!("invalid inst_code"),
            },
            Opcode::SED => match self.addressing {
                Addressing::Implied => 0xf8,
                _ => panic!("invalid inst_code"),
            },
            Opcode::SEI => match self.addressing {
                Addressing::Implied => 0x78,
                _ => panic!("invalid inst_code"),
            },
            Opcode::STA => match self.addressing {
                Addressing::Zeropage => 0x85,
                Addressing::ZeropageX => 0x95,
                Addressing::Absolute => 0x8d,
                Addressing::AbsoluteX => 0x9d,
                Addressing::AbsoluteY => 0x99,
                Addressing::IndirectX => 0x81,
                Addressing::IndirectY => 0x91,
                _ => panic!("invalid inst_code"),
            },
            Opcode::STX => match self.addressing {
                Addressing::Zeropage => 0x86,
                Addressing::ZeropageY => 0x96,
                Addressing::Absolute => 0x8e,
                _ => panic!("invalid inst_code"),
            },
            Opcode::STY => match self.addressing {
                Addressing::Zeropage => 0x84,
                Addressing::ZeropageX => 0x94,
                Addressing::Absolute => 0x8c,
                _ => panic!("invalid inst_code"),
            },
            Opcode::TAX => match self.addressing {
                Addressing::Implied => 0xaa,
                _ => panic!("invalid inst_code"),
            },
            Opcode::TAY => match self.addressing {
                Addressing::Implied => 0xa8,
                _ => panic!("invalid inst_code"),
            },
            Opcode::TSX => match self.addressing {
                Addressing::Implied => 0xba,
                _ => panic!("invalid inst_code"),
            },
            Opcode::TXA => match self.addressing {
                Addressing::Implied => 0x8a,
                _ => panic!("invalid inst_code"),
            },
            Opcode::TXS => match self.addressing {
                Addressing::Implied => 0x9a,
                _ => panic!("invalid inst_code"),
            },
            Opcode::TYA => match self.addressing {
                Addressing::Implied => 0x98,
                _ => panic!("invalid inst_code"),
            },
            _ => panic!("invalid inst_code")
        }
    }
}
