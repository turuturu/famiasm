use crate::common::{Annot, Loc};
use crate::directive::Directive;
use crate::insts::{
    AbstructAddress, AbstructInstruction, Addressing, Instruction, Opcode, Operand, RamAddress,
};
use crate::nes_header::NesHeader;
use crate::symbol_table::SymbolTable;
use crate::tokenizer::{Token, TokenKind};
use std::mem;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parser {
    symtab: SymbolTable,
    current_address: RamAddress,
    insts: Vec<AbstructInstruction>,
    meta_info: NesHeader,
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
            meta_info: Default::default(),
        }
    }
    fn get_operand(&self, tokens: &Vec<Annot<TokenKind>>, offset: usize) -> Option<Operand> {
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
                    let ram_address = RamAddress {
                        bank: self.current_address.bank,
                        address: adr,
                    };
                    let adr = AbstructAddress::RamAddress(ram_address);
                    return Some(Operand::Address(adr));
                }
                TokenKind::Adr16(adr) => {
                    let adr: u16 = From::from(adr.clone());
                    let ram_address = RamAddress {
                        bank: self.current_address.bank,
                        address: adr,
                    };
                    let adr = AbstructAddress::RamAddress(ram_address);
                    return Some(Operand::Address(adr));
                }
                TokenKind::U8(val) => {
                    return Some(Operand::U8(val.clone()));
                }
                TokenKind::U16(val) => {
                    return Some(Operand::U16(val.clone()));
                }
                TokenKind::String(str) => {
                    return Some(Operand::String(str.clone()));
                }
                _ => (),
            }
        }
        return None;
    }
    pub fn resolve_address(&mut self) {
        for inst in &mut self.insts {
            if let AbstructInstruction::Instruction(inst) = inst {
                if let Some(Operand::Address(AbstructAddress::Label(label))) = &inst.operand {
                    println!("{:?}", label);
                    let adr = self.symtab.get(label);
                    inst.operand = Some(Operand::Address(AbstructAddress::RamAddress(
                        adr.unwrap().clone(),
                    )));
                }
            }
        }
    }

    pub fn parse(&mut self, token_lines: Vec<Vec<Token>>) {
        println!("parse");
        let mut current_bank = 0;
        // todo .org処理
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
            let mut token_length = tokens.len();
            if token_length == 0 {
                continue;
            }
            let mut current_pos = 0;
            if let TokenKind::LabelDef(label) = &tokens[current_pos].value {
                // tood 同一行のop処理
                println!("labelDef({:?})", label);
                let label = &label[..label.len() - 1];
                self.symtab
                    .insert(label.iter().collect::<String>(), address.clone());
                current_pos += 1;
                token_length -= 1;
            }
            if token_length == 0 {
                continue;
            }
            match &tokens[current_pos].value {
                TokenKind::Directive(directive) => {
                    let d = Directive::from_str(&(directive.iter().collect::<String>())).unwrap();
                    current_pos += 1;
                    println!("get_operand({:?})", &tokens);
                    let val = self.get_operand(&tokens, current_pos).unwrap();
                    match d {
                        Directive::ORG => {
                            if let Operand::U16(adr) = val {
                                self.current_address.address = adr;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::INESPRG => {
                            if let Operand::U8(val) = val {
                                self.meta_info.prg_rom_count = val;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::INESCHR => {
                            println!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                self.meta_info.chr_rom_count = val;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::INESMIR => {
                            println!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                self.meta_info.mirror = val;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::INESMAP => {
                            println!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                self.meta_info.mapper = val;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::BANK => {
                            println!("directive({:?})", d);
                            if let Operand::U8(bank) = val {
                                self.current_address.bank = bank;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::DB | Directive::BYTE => {
                            println!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                self.insts.push(AbstructInstruction::Bin(vec![val]));
                                current_address += 1;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::DW | Directive::WORD => {
                            println!("directive({:?})", d);
                            match val {
                                Operand::U16(val) => {
                                    let val = val.to_le_bytes();
                                    self.insts.push(AbstructInstruction::Bin(
                                        val.iter().cloned().collect(),
                                    ));
                                    current_address += 2;
                                    continue;
                                }
                                Operand::U8(val) => {
                                    let val: u16 = From::from(val);
                                    let val = val.to_le_bytes();
                                    self.insts.push(AbstructInstruction::Bin(
                                        val.iter().cloned().collect(),
                                    ));
                                    current_address += 2;
                                    continue;
                                }
                                Operand::Address(adr) => {
                                    continue;
                                }
                                _ => {
                                    println!("aa {:?}", val);
                                    panic!();
                                }
                            }
                        }
                        Directive::INCBIN => {
                            println!("directive({:?})", d);
                            let val = self.get_operand(&tokens, current_pos);
                            current_address += 0; // todo ファイルサイズ分だけincrement
                            println!("val({:?})", val);
                        }
                        _ => {
                            println!("unsupported directive {:?}", d);
                        }
                    }
                }
                /*TokenKind::LabelDef(label) => {
                    // tood 同一行のop処理
                    println!("labelDef({:?})", label);
                    let label = &label[..label.len()-1];
                    self.symtab
                        .insert(label.iter().collect::<String>(), address);
                    //     self.symtab
                    //     .insert(label.iter().collect::<String>(), address);
                    continue;
                }*/
                TokenKind::Opcode(x) => {
                    println!("{:?}, Opcode(x) => {:?}", token_length, x);
                    let op: Opcode = (&x.iter().collect::<String>()).parse().unwrap();
                    // Implied op
                    if token_length == 1 {
                        let inst = Instruction::new(op, Addressing::Implied, None, address);
                        let inst_info = inst.get_op_info();
                        println!("{:?}", inst.get_op_info());
                        self.insts.push(AbstructInstruction::Instruction(inst));
                        current_address = current_address + (inst_info.num_bytes as u16);
                        continue;
                    }
                    current_pos += 1;
                    let next = &tokens[current_pos].value;
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
                            let operand = self.get_operand(&tokens, current_pos);
                            let inst = Instruction::new(
                                op,
                                Addressing::Relative,
                                Some(operand.unwrap()),
                                address,
                            );
                            let inst_info = inst.get_op_info();
                            current_address = current_address + (inst_info.num_bytes as u16);
                            println!("{:?}", next);
                            self.insts.push(AbstructInstruction::Instruction(inst));
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
                            self.insts.push(AbstructInstruction::Instruction(inst));
                            continue;
                        }
                        // Immediate op
                        TokenKind::Im(val) => {
                            let operand = self.get_operand(&tokens, current_pos);
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
                            self.insts.push(AbstructInstruction::Instruction(inst));
                            continue;
                        }
                        // Absolute|Zeropage op
                        TokenKind::U8(_) | TokenKind::U16(_) | TokenKind::Label(_) => {
                            let operand = self.get_operand(&tokens, current_pos);
                            let addressing = match next {
                                TokenKind::U8(_) => {
                                    if token_length == 2 {
                                        Addressing::Zeropage
                                    } else {
                                        current_pos += 1;
                                        let next = &tokens[current_pos].value;
                                        match next {
                                            TokenKind::X => Addressing::ZeropageX,
                                            TokenKind::Y => Addressing::ZeropageY,
                                            _ => panic!(),
                                        }
                                    }
                                }
                                TokenKind::U16(_) | TokenKind::Label(_) => {
                                    if token_length == 2 {
                                        Addressing::Absolute
                                    } else {
                                        current_pos += 1;
                                        let next = &tokens[current_pos].value;
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
                            self.insts.push(AbstructInstruction::Instruction(inst));
                            current_address = current_address + (inst_info.num_bytes as u16);
                        }
                        // Indirect op
                        TokenKind::LParen => {
                            if token_length < 4 {
                                panic!();
                            }
                            current_pos += 1;
                            let operand = self.get_operand(&tokens, current_pos);
                            let next = &tokens[current_pos].value;
                            let addressing = match next {
                                TokenKind::U8(_) => {
                                    current_pos += 1;
                                    let next = &tokens[current_pos].value;
                                    match next {
                                        TokenKind::X => {
                                            current_pos += 1;
                                            let next = &tokens[current_pos].value;
                                            if let TokenKind::RParen = next {
                                                Addressing::IndirectX
                                            } else {
                                                panic!()
                                            }
                                        }
                                        TokenKind::RParen => {
                                            current_pos += 1;
                                            let next = &tokens[current_pos].value;
                                            if let TokenKind::Y = next {
                                                Addressing::IndirectY
                                            } else {
                                                panic!()
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                                TokenKind::U16(_) | TokenKind::Label(_) => {
                                    current_pos += 1;
                                    let next = &tokens[current_pos].value;
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
                            self.insts.push(AbstructInstruction::Instruction(inst));
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
        println!("xxxxxxxxxxxxxxxxxxxxxxxxxx");
        println!("{:?}", &self.symtab);
        println!("xxxxxxxxxxxxxxxxxxxxxxxxxx");
        for inst in &self.insts {
            println!("{:?}", inst);
        }
        self.resolve_address();
        println!("xxxxxxxxxxxxxxxxxxxxxxxxxx");
        for inst in &self.insts {
            println!("{:?}", inst);
        }
    }
}
