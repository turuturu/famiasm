use crate::common::Annot;
use crate::directive::Directive;
use crate::insts::{
    AbstructAddress, AbstructInstruction, Addressing, Bin, Instruction, Label, Opcode, Operand,
    RamAddress,
};
use crate::nes_header::NesHeader;
use crate::symbol_table::SymbolTable;
use crate::tokenizer::{Token, TokenKind};
use log::debug;
use std::str::FromStr;
use std::{fs, mem};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parser {
    symtab: SymbolTable,
    current_address: RamAddress,
    insts: Vec<AbstructInstruction>,
    meta_info: NesHeader,
    base_path: Option<PathBuf>,
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
            base_path: None,
        }
    }

    pub fn set_base_path(&mut self, path: &str) {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            self.base_path = Some(parent.to_path_buf());
        } else {
            self.base_path = Some(PathBuf::from("."));
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
                    if inst.addressing == Addressing::Relative {
                        // Relative Addressing
                        let op_info = inst.get_op_info();
                        let mut adr = self.symtab.get(label).unwrap().clone();
                        let rel_address: i16 = adr.address as i16
                            - inst.address.address as i16
                            - op_info.num_bytes as i16;
                        adr.address = rel_address as u16;
                        inst.operand = Some(Operand::Address(AbstructAddress::RamAddress(adr)));
                    } else {
                        debug!("{:?}", label);
                        let adr = self.symtab.get(label);
                        inst.operand = Some(Operand::Address(AbstructAddress::RamAddress(
                            adr.unwrap().clone(),
                        )));
                        // Absolute Addressing
                    }
                }
            } else if let AbstructInstruction::Label(labelobj) = inst {
                if let AbstructAddress::Label(label) = &labelobj.label {
                    debug!("yyyyyyyyyyyyy {:?}", label);
                    let adr = self.symtab.get(label).unwrap().clone();
                    let bin =
                        Bin::new(adr.address.to_le_bytes().to_vec(), labelobj.address.clone());
                    *inst = AbstructInstruction::Bin(bin);
                }
            }
        }
    }

    pub fn gen_binary(&self) -> Vec<u8> {
        let mut nes_header: Vec<u8> = self.meta_info.gen_binary().to_vec();
        let num_prg_rom = self.meta_info.prg_rom_count as usize;
        let num_chr_rom = self.meta_info.chr_rom_count as usize;
        let mut prg_roms: Vec<Vec<u8>> = vec![vec![0xFF; 16 * 1024]; num_prg_rom];
        let mut chr_roms: Vec<Vec<u8>> = vec![vec![0; 8 * 1024]; num_chr_rom];
        debug!("start ---------------------");
        debug!("nes_header = {:?}", nes_header);
        debug!("num_prg_rom = {:?}", num_prg_rom);
        debug!("num_chr_rom = {:?}", num_chr_rom);
        for inst in &self.insts {
            let target_bank = match inst {
                AbstructInstruction::Instruction(inst) => {
                    if num_prg_rom == 1 && inst.address.bank != 0 {
                        inst.address.bank - 1
                    } else {
                        inst.address.bank
                    }
                }
                AbstructInstruction::Bin(bin) => {
                    if num_prg_rom == 1 && bin.address.bank != 0 {
                        bin.address.bank - 1
                    } else {
                        bin.address.bank
                    }
                }
                _ => panic!(),
            } as usize;
            let target_address = match inst {
                AbstructInstruction::Instruction(inst) => inst.address.address,
                AbstructInstruction::Bin(bin) => bin.address.address,
                _ => panic!(),
            };
            let target_index = if target_address < 0x2000 {
                target_address as usize
            } else if 0xC000 <= target_address {
                target_address as usize - 0xC000
            } else if 0x8000 <= target_address {
                target_address as usize - 0x8000
            } else {
                panic!();
            } as usize;
            let target_rom = if num_prg_rom <= target_bank as usize {
                &mut chr_roms[target_bank - num_prg_rom]
            } else if 0xC000 <= target_address {
                &mut prg_roms[num_prg_rom - 1]
            } else {
                &mut prg_roms[target_bank]
            };
            // debug!("{:?}", inst);
            // debug!("target_bank {:?}", target_bank);
            // debug!("target_address {:?}", target_address);
            // debug!("target_index {:?}", target_index);
            match inst {
                AbstructInstruction::Instruction(inst) => {
                    let inst_code = inst.get_inst_code();
                    target_rom[target_index..target_index + inst_code.len()]
                        .copy_from_slice(&inst_code);
                }
                AbstructInstruction::Bin(bin) => {
                    let len = bin.dat.len();
                    target_rom[target_index..target_index + len].copy_from_slice(&bin.dat);
                }
                _ => panic!(),
            }
        }
        // concatenate
        nes_header.extend(prg_roms.into_iter().flatten());
        nes_header.extend(chr_roms.into_iter().flatten());
        return nes_header;
    }

    pub fn parse(&mut self, token_lines: Vec<Vec<Token>>) {
        debug!("parse");
        // let mut current_bank = 0;
        // todo .org処理
        // let mut current_address = 0;
        for tokens in token_lines {
            debug!("{:?}", &tokens);
            let address = RamAddress {
                bank: self.current_address.bank,
                address: self.current_address.address,
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
                debug!("labelDef({:?})", label);
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
                            debug!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                self.meta_info.chr_rom_count = val;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::INESMIR => {
                            debug!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                self.meta_info.mirror = val;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::INESMAP => {
                            debug!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                self.meta_info.mapper = val;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::BANK => {
                            debug!("directive({:?})", d);
                            if let Operand::U8(bank) = val {
                                self.current_address.bank = bank;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::DB | Directive::BYTE => {
                            debug!("directive({:?})", d);
                            if let Operand::U8(val) = val {
                                // let bin = Bin::new(vec![val], self.current_address.clone());
                                let bin = Bin::new(
                                    vec![val],
                                    RamAddress {
                                        bank: self.current_address.bank,
                                        address: self.current_address.address,
                                    },
                                );
                                self.insts.push(AbstructInstruction::Bin(bin));
                                self.current_address.address += 1;
                                continue;
                            } else {
                                panic!();
                            }
                        }
                        Directive::DW | Directive::WORD => {
                            debug!("directive({:?})", d);
                            match val {
                                Operand::U16(val) => {
                                    let val = val.to_le_bytes();
                                    let bin = Bin::new(
                                        val.iter().cloned().collect(),
                                        RamAddress {
                                            bank: self.current_address.bank,
                                            address: self.current_address.address,
                                        },
                                    );

                                    self.insts.push(AbstructInstruction::Bin(bin));
                                    self.current_address.address =
                                        self.current_address.address.wrapping_add(2);
                                    continue;
                                }
                                Operand::U8(val) => {
                                    let val: u16 = From::from(val);
                                    let val = val.to_le_bytes();
                                    let bin = Bin::new(
                                        val.iter().cloned().collect(),
                                        RamAddress {
                                            bank: self.current_address.bank,
                                            address: self.current_address.address,
                                        },
                                    );
                                    self.insts.push(AbstructInstruction::Bin(bin));
                                    self.current_address.address =
                                        self.current_address.address.wrapping_add(2);
                                    continue;
                                }
                                Operand::Address(adr) => {
                                    debug!("zzzzzzzz adr = {:?}", adr);
                                    let label = Label::new(
                                        adr,
                                        RamAddress {
                                            bank: self.current_address.bank,
                                            address: self.current_address.address,
                                        },
                                    );
                                    self.insts.push(AbstructInstruction::Label(label));
                                    self.current_address.address =
                                        self.current_address.address.wrapping_add(2);
                                    continue;
                                }
                                _ => {
                                    debug!("aa {:?}", val);
                                    panic!();
                                }
                            }
                        }
                        Directive::INCBIN => {
                            debug!("directive({:?})", d);
                            let val = self.get_operand(&tokens, current_pos);
                            if let Some(Operand::String(filename)) = val {
                                debug!("filename({:?})", filename);
                                let filename_str: String = filename.iter().collect();
                                
                                // Resolve the path relative to the ASM file
                                let file_path = if let Some(ref base) = self.base_path {
                                    base.join(&filename_str)
                                } else {
                                    PathBuf::from(&filename_str)
                                };
                                
                                let data: Vec<u8> = fs::read(&file_path)
                                    .unwrap_or_else(|e| panic!("Failed to read file {:?}: {}", file_path, e));
                                let file_size = data.len() as u16;
                                let bin = Bin::new(
                                    data,
                                    RamAddress {
                                        bank: self.current_address.bank,
                                        address: self.current_address.address,
                                    },
                                );
                                self.insts.push(AbstructInstruction::Bin(bin));
                                self.current_address.address += file_size;
                            } else {
                                panic!();
                            }
                            continue;
                        }
                    }
                }
                /*TokenKind::LabelDef(label) => {
                    // tood 同一行のop処理
                    debug!("labelDef({:?})", label);
                    let label = &label[..label.len()-1];
                    self.symtab
                        .insert(label.iter().collect::<String>(), address);
                    //     self.symtab
                    //     .insert(label.iter().collect::<String>(), address);
                    continue;
                }*/
                TokenKind::Opcode(x) => {
                    debug!("{:?}, Opcode(x) => {:?}", token_length, x);
                    let op: Opcode = (&x.iter().collect::<String>()).parse().unwrap();
                    // Implied op
                    if token_length == 1 {
                        let inst = Instruction::new(op, Addressing::Implied, None, address);
                        let inst_info = inst.get_op_info();
                        debug!("ooooppppinfo {:?}", inst.get_op_info());
                        debug!("inst {:?}", inst);
                        self.insts.push(AbstructInstruction::Instruction(inst));
                        self.current_address.address =
                            self.current_address.address + (inst_info.num_bytes as u16);
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
                            self.current_address.address =
                                self.current_address.address + (inst_info.num_bytes as u16);
                            debug!("{:?}", next);
                            self.insts.push(AbstructInstruction::Instruction(inst));
                            continue;
                        }
                        _ => (),
                    }
                    debug!("next = {:?}", next);
                    match next {
                        // Accumulator op
                        TokenKind::A => {
                            let inst = Instruction::new(op, Addressing::Accumulator, None, address);
                            let inst_info = inst.get_op_info();
                            self.current_address.address =
                                self.current_address.address + (inst_info.num_bytes as u16);
                            debug!("{:?}", inst.get_op_info());
                            debug!("{:?}", inst.get_inst_code());
                            self.insts.push(AbstructInstruction::Instruction(inst));
                            continue;
                        }
                        // Immediate op
                        TokenKind::Im(_val) => {
                            let operand = self.get_operand(&tokens, current_pos);
                            let inst = Instruction::new(
                                op,
                                Addressing::Immediate,
                                Some(operand.unwrap()),
                                address,
                            );
                            let inst_info = inst.get_op_info();
                            self.current_address.address =
                                self.current_address.address + (inst_info.num_bytes as u16);
                            debug!("{:?}", inst.get_op_info());
                            debug!("{:?}", inst.get_inst_code());
                            self.insts.push(AbstructInstruction::Instruction(inst));
                            continue;
                        }
                        // Absolute|Zeropage op
                        TokenKind::Adr8(_) | TokenKind::Adr16(_) | TokenKind::Label(_) => {
                            let operand = self.get_operand(&tokens, current_pos);
                            let addressing = match next {
                                TokenKind::Adr8(_) => {
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
                                TokenKind::Adr16(_) | TokenKind::Label(_) => {
                                    if token_length == 2 {
                                        Addressing::Absolute
                                    } else {
                                        current_pos += 1;
                                        let next = &tokens[current_pos].value;
                                        debug!("next = {:?}", next);
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
                            self.current_address.address =
                                self.current_address.address + (inst_info.num_bytes as u16);
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
                                TokenKind::Adr8(_) => {
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
                                TokenKind::Adr16(_) | TokenKind::Label(_) => {
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
                            self.current_address.address =
                                self.current_address.address + (inst_info.num_bytes as u16);
                        }
                        _ => (),
                    }
                    debug!("op");
                    //                debug!("{:?}", op);
                }
                _ => {
                    continue;
                }
            }
        }
        debug!("xxxxxxxxxxxxxxxxxxxxxxxxxx");
        debug!("{:?}", &self.symtab);
        // debug!("xxxxxxxxxxxxxxxxxxxxxxxxxx");
        // for inst in &self.insts {
        //     debug!("{:?}", inst);
        // }
        self.resolve_address();
        // debug!("xxxxxxxxxxxxxxxxxxxxxxxxxx");
        // for inst in &self.insts {
        //     debug!("{:?}", inst);
        // }
    }
}
