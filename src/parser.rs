use crate::common::{Annot, Loc};
use crate::insts::{AbstructAddress, Addressing, Instruction, Opcode, Operand, RamAddress};
use crate::symbol_table::SymbolTable;
use crate::tokenizer::{Token, TokenKind};
use std::mem;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parser {
    symtab: SymbolTable,
    current_address: RamAddress,
    insts: Vec<Instruction>,
}
impl Parser {
    pub fn new() -> Parser {
        Parser {
            symtab: SymbolTable::new(),
            current_address: RamAddress {
                bank: 0,
                address: 0,
            },
            insts: vec![],
        }
    }
    fn get_operand(
        &self,
        tokens: &Vec<Annot<TokenKind>>,
        offset: usize,
        bank: u8,
    ) -> Option<Operand> {
        let length = tokens.len();
        if length <= offset {
            return None;
        }
        let tokens = &tokens[offset..];
        for token in tokens {
            match &token.value {
                TokenKind::Im(im) => {
                    let im: u16 = From::from(im.clone());
                    return Some(Operand::Im(im));
                }
                TokenKind::Label(label) => {
                    return Some(Operand::Address(AbstructAddress::Label(
                        label.iter().collect::<String>(),
                    )));
                }
                TokenKind::Adr8(adr) => {
                    let adr: u16 = From::from(adr.clone());
                    let rom_address = RamAddress {
                        bank: bank,
                        address: adr,
                    };
                    let adr = AbstructAddress::RamAddress(rom_address);
                    return Some(Operand::Address(adr));
                }
                TokenKind::Adr16(adr) => {
                    let adr: u16 = From::from(adr.clone());
                    let rom_address = RamAddress {
                        bank: bank,
                        address: adr,
                    };
                    let adr = AbstructAddress::RamAddress(rom_address);
                    return Some(Operand::Address(adr));
                }
                _ => (),
            }
        }
        return None;
    }

    pub fn parse(&mut self, token_lines: Vec<Vec<Token>>) {
        println!("parse");
        let mut current_bank = 0;
        let mut current_address = 0;
        for tokens in token_lines {
            println!("{:?}", &tokens);
            let address = RamAddress {
                bank: current_bank,
                address: current_address,
            };
            let tokens: Vec<Token> = tokens
                .into_iter()
                .filter(|t| {
                    mem::discriminant(&t.value)
                        != mem::discriminant(&TokenKind::Comment("".chars().collect()))
                })
                .collect();
            let token_length = tokens.len();
            if token_length == 0 {
                continue;
            }
            match &tokens[0].value {
                TokenKind::LabelDef(label) => {
                    println!("labelDef({:?})", label);
                    self.symtab
                        .insert(label.iter().collect::<String>(), address);
                    continue;
                }
                TokenKind::Opcode(x) => {
                    println!("{:?}, Opcode(x) => {:?}", token_length, x);
                    let op: Opcode = (&x.iter().collect::<String>()).parse().unwrap();
                    // Implied op
                    if token_length == 1 {
                        let inst = Instruction::new(op, Addressing::Implied, None, address);
                        let inst_info = inst.get_op_info();
                        current_address = current_address + (inst_info.num_bytes as u16);
                        println!("{:?}", inst.get_op_info());
                        continue;
                    }
                    let next = &tokens[1].value;
                    // Relative op
                    match &op {
                        Opcode::BCC
                        | Opcode::BCS
                        | Opcode::BEQ
                        | Opcode::BMI
                        | Opcode::BNE
                        | Opcode::BPL
                        | Opcode::BVC
                        | Opcode::BVS => {
                            let operand = self.get_operand(&tokens, 1, current_bank);
                            let inst = Instruction::new(
                                op,
                                Addressing::Relative,
                                Some(operand.unwrap()),
                                address,
                            );
                            let inst_info = inst.get_op_info();
                            current_address = current_address + (inst_info.num_bytes as u16);
                            println!("{:?}", next);
                            continue;
                        }
                        _ => (),
                    }
                    match next {
                        // Accumulator op
                        TokenKind::A => {
                            let inst = Instruction::new(op, Addressing::Accumulator, None, address);
                            let inst_info = inst.get_op_info();
                            current_address = current_address + (inst_info.num_bytes as u16);
                            println!("{:?}", inst.get_op_info());
                            println!("{:?}", inst.get_inst_code());
                            continue;
                        }
                        // Immediate op
                        TokenKind::Im(val) => {
                            let operand = self.get_operand(&tokens, 1, current_bank);
                            let inst = Instruction::new(
                                op,
                                Addressing::Immediate,
                                Some(operand.unwrap()),
                                address,
                            );
                            let inst_info = inst.get_op_info();
                            current_address = current_address + (inst_info.num_bytes as u16);
                            println!("{:?}", inst.get_op_info());
                            println!("{:?}", inst.get_inst_code());
                            continue;
                        }
                        // Absolute op
                        TokenKind::Adr8(_) | TokenKind::Adr16(_) | TokenKind::Label(_) => {
                            let operand = self.get_operand(&tokens, 1, current_bank);
                            let addressing = match next {
                                TokenKind::Adr8(_) => {
                                    if token_length == 2 {
                                        Addressing::Zeropage
                                    } else {
                                        let next = &tokens[2].value;
                                        match next {
                                            TokenKind::X => Addressing::ZeropageX,
                                            TokenKind::Y => Addressing::ZeropageY,
                                            _ => panic!(),
                                        }
                                    }
                                }
                                TokenKind::Adr16(_) | TokenKind::Label(_) => {
                                    if token_length == 2 {
                                        Addressing::Absolute
                                    } else {
                                        let next = &tokens[2].value;
                                        println!("next = {:?}", next);
                                        match next {
                                            TokenKind::X => Addressing::AbsoluteX,
                                            TokenKind::Y => Addressing::AbsoluteY,
                                            _ => panic!(),
                                        }
                                    }
                                }
                                _ => panic!(), // never reached here
                            };
                            let inst = Instruction::new(op, addressing, operand, address);
                            let inst_info = inst.get_op_info();
                            current_address = current_address + (inst_info.num_bytes as u16);
                        }
                        // Indirect op
                        TokenKind::LParen => {
                            if token_length < 4 {
                                panic!();
                            }
                            let operand = self.get_operand(&tokens, 2, current_bank);
                            let next = &tokens[2].value;
                            let addressing = match next {
                                TokenKind::Adr8(_) => {
                                    let next = &tokens[3].value;
                                    match next {
                                        TokenKind::X => {
                                            let next = &tokens[4].value;
                                            if let TokenKind::RParen = next {
                                                Addressing::IndirectX
                                            } else {
                                                panic!()
                                            }
                                        }
                                        TokenKind::RParen => {
                                            let next = &tokens[4].value;
                                            if let TokenKind::Y = next {
                                                Addressing::IndirectY
                                            } else {
                                                panic!()
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                                TokenKind::Adr16(_) | TokenKind::Label(_) => {
                                    let next = &tokens[3].value;
                                    if let TokenKind::RParen = next {
                                        Addressing::Indirect
                                    } else {
                                        panic!()
                                    }
                                }
                                _ => panic!(),
                            };
                            let inst = Instruction::new(op, addressing, operand, address);
                            let inst_info = inst.get_op_info();
                            current_address = current_address + (inst_info.num_bytes as u16);
                        }
                        _ => (),
                    }
                    println!("op");
                    //                println!("{:?}", op);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
