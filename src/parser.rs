use crate::common::{Annot, Loc};
use crate::tokenizer::{Token};

#[derive(strum_macros::Display)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum OpcodeKind {
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
type Opcode = Annot<OpcodeKind>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
    Directive(Vec<char>),
    Op,
}
type Ast = Annot<AstKind>;

enum ParseError {
    UnexpectedToken(Token),
}
impl Ast{
    fn directive(d: Vec<char>, loc: Loc) -> Self {
        Self::new(AstKind::Directive(d), loc)
    }
    fn op(label: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Op, loc)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError>{
}
